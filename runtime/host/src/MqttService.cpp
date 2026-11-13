/* MqttService.cpp — tplc-mqtt MQTT client via libpaho-mqtt3c
 * (Eclipse Paho synchronous MQTTClient — opt-in like the other
 * special features, inert when TPLC_HAVE_MQTT is off).
 *
 * The PLC is the client; Paho owns the MQTT wire protocol
 * (CONNECT/CONNACK/PUBLISH/PUBACK/SUBSCRIBE/SUBACK/PINGREQ).  The
 * surface maps connect -> MQTTClient_connect, publish ->
 * MQTTClient_publish, subscribe -> MQTTClient_subscribe, receive ->
 * MQTTClient_receive (nonblocking poll).  Every call is bounded so a
 * cyclic scan can't stall past the watchdog.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/MqttService.hpp>

#ifdef TPLC_HAVE_MQTT
#include <MQTTClient.h>
#endif

#include <atomic>
#include <chrono>
#include <cstdlib>
#include <cstring>
#include <set>
#include <string>
#include <thread>
#include <vector>

#ifdef TPLC_HAVE_MQTT
#include <dirent.h>
#include <sys/socket.h>
#include <unistd.h>
#endif

namespace tplc {

namespace {

#ifdef TPLC_HAVE_MQTT
/* socket fds owned by this process; used to identify the fd Paho
   opened for a client so abort() can drop it without DISCONNECT. */
std::set<int> socket_fds() {
    std::set<int> out;
    DIR* d = opendir("/proc/self/fd");
    if (!d) return out;
    while (dirent* e = readdir(d)) {
        if (e->d_name[0] < '0' || e->d_name[0] > '9') continue;
        char path[280], l[128];
        std::snprintf(path, sizeof(path), "/proc/self/fd/%s", e->d_name);
        ssize_t n = readlink(path, l, sizeof(l) - 1);
        if (n <= 0) continue;
        l[n] = '\0';
        if (std::strncmp(l, "socket:", 7) == 0)
            out.insert(std::atoi(e->d_name));
    }
    closedir(d);
    return out;
}
#endif

/* owned copy of everything needed to re-issue MQTTClient_connect
   from the backoff watchdog (the ABI cfg's pointers reference PLC
   stack memory that is gone by the time a reconnect is needed). */
struct ConnectSpec {
    std::string host;
    uint32_t    port = 0;
    int32_t     clean_session = 1;
    uint32_t    keepalive_s = 60;
    std::string username;
    std::string password;
    std::string client_id;
    std::string will_topic;
    std::vector<uint8_t> will_payload;
    uint32_t    will_qos = 0;
    uint32_t    will_retain = 0;
    uint32_t    use_tls = 0;
    uint32_t    tls_insecure = 0;
    std::string tls_ca_file;
    std::string tls_cert_file;
    std::string tls_key_file;
    uint32_t    version = 3;
    uint32_t    session_expiry_s = 0;
    uint32_t    receive_max = 0;
    uint32_t    topic_alias_max = 0;
    uint32_t    req_resp_info = 0;
    uint32_t    req_prob_info = 0;
};

struct Conn {
#ifdef TPLC_HAVE_MQTT
    MQTTClient conn = nullptr;
    int32_t  state = 0;
    uint32_t last_msg_id = 0;
    int32_t  last_error = 0;
    int      fd = -1;
    int      version = 3;
    std::chrono::steady_clock::time_point last_rx{};
    ConnectSpec spec;
    bool     bo_en = false;
    uint32_t bo_min = 1000;
    uint32_t bo_max = 30000;
    std::atomic<bool> wd_stop{true};
    std::thread wd;
#else
    void*    conn = nullptr;
    int32_t  state = 0;
    uint32_t last_msg_id = 0;
    int32_t  last_error = 0;
#endif
};

} // namespace

#ifdef TPLC_HAVE_MQTT
namespace {

/* build MQTT5 connect properties from a spec (session expiry,
   receive maximum, topic alias maximum, request response/problem
   information); empty properties are omitted so the broker applies
   its defaults. */
static void fill_connect_props(const ConnectSpec& s, MQTTProperties& p) {
    MQTTProperty prop;
    memset(&prop, 0, sizeof(prop));
    if (s.session_expiry_s) {
        prop.identifier = MQTTPROPERTY_CODE_SESSION_EXPIRY_INTERVAL;
        prop.value.integer4 = s.session_expiry_s;
        MQTTProperties_add(&p, &prop);
    }
    if (s.receive_max) {
        prop.identifier = MQTTPROPERTY_CODE_RECEIVE_MAXIMUM;
        prop.value.integer2 = (unsigned short)s.receive_max;
        MQTTProperties_add(&p, &prop);
    }
    if (s.topic_alias_max) {
        prop.identifier = MQTTPROPERTY_CODE_TOPIC_ALIAS_MAXIMUM;
        prop.value.integer2 = (unsigned short)s.topic_alias_max;
        MQTTProperties_add(&p, &prop);
    }
    if (s.req_resp_info) {
        prop.identifier = MQTTPROPERTY_CODE_REQUEST_RESPONSE_INFORMATION;
        prop.value.byte = 1;
        MQTTProperties_add(&p, &prop);
    }
    if (s.req_prob_info) {
        prop.identifier = MQTTPROPERTY_CODE_REQUEST_PROBLEM_INFORMATION;
        prop.value.byte = 1;
        MQTTProperties_add(&p, &prop);
    }
}

/* re-issue MQTTClient_connect from a stored ConnectSpec (used by
   the backoff watchdog; mirrors the option setup in connect()). */
int spec_connect(MQTTClient c, const ConnectSpec& s) {
    MQTTClient_connectOptions opts = MQTTClient_connectOptions_initializer;
    opts.keepAliveInterval = int(s.keepalive_s ? s.keepalive_s : 60);
    opts.cleansession = s.clean_session;
    opts.connectTimeout = 5;
    if (!s.username.empty()) opts.username = s.username.c_str();
    if (!s.password.empty()) opts.password = s.password.c_str();
    MQTTClient_willOptions will;
    MQTTClient_SSLOptions sslo;
    if (!s.will_topic.empty()) {
        will = MQTTClient_willOptions_initializer;
        will.topicName = s.will_topic.c_str();
        will.payload.len = int(s.will_payload.size());
        will.payload.data = s.will_payload.empty()
            ? nullptr : const_cast<uint8_t*>(s.will_payload.data());
        will.qos = int(s.will_qos);
        will.retained = s.will_retain ? 1 : 0;
        opts.will = &will;
    }
    if (s.use_tls) {
        sslo = MQTTClient_SSLOptions_initializer;
        sslo.trustStore = s.tls_ca_file.empty() ? nullptr
                                              : s.tls_ca_file.c_str();
        sslo.keyStore = s.tls_cert_file.empty() ? nullptr
                                              : s.tls_cert_file.c_str();
        sslo.privateKey = s.tls_key_file.empty() ? nullptr
                                               : s.tls_key_file.c_str();
        sslo.enableServerCertAuth = s.tls_insecure ? 0 : 1;
        opts.ssl = &sslo;
    }
    if (s.version == 5) {
        opts.MQTTVersion = MQTTVERSION_5;
        /* v5 rejects cleansession != 0; clean start lives in
           cleanstart instead */
        opts.cleansession = 0;
        opts.cleanstart = s.clean_session;
        MQTTProperties props = MQTTProperties_initializer;
        fill_connect_props(s, props);
        MQTTResponse r = MQTTClient_connect5(c, &opts, &props, nullptr);
        MQTTProperties_free(&props);
        int rc = (r.reasonCode == MQTTREASONCODE_SUCCESS)
                     ? MQTTCLIENT_SUCCESS : MQTTCLIENT_FAILURE;
        MQTTResponse_free(r);
        return rc;
    }
    return MQTTClient_connect(c, &opts);
}

/* exponential-backoff watchdog: while the connection is down,
   wait a doubling delay (min..max) between reconnect attempts;
   a successful connect resets the delay.  Paho serializes its own
   calls internally, so this may run concurrently with PLC ops. */
void backoff_loop(Conn* c) {
    uint32_t delay = c->bo_min;
    while (!c->wd_stop.load() && c->conn) {
        if (MQTTClient_isConnected(c->conn)) {
            delay = c->bo_min;
            std::this_thread::sleep_for(std::chrono::milliseconds(200));
            continue;
        }
        if (!c->bo_en) {
            std::this_thread::sleep_for(std::chrono::milliseconds(200));
            continue;
        }
        for (uint32_t t = 0; t < delay && !c->wd_stop.load(); t += 50)
            std::this_thread::sleep_for(std::chrono::milliseconds(50));
        if (c->wd_stop.load() || !c->conn) break;
        if (spec_connect(c->conn, c->spec) == MQTTCLIENT_SUCCESS) {
            delay = c->bo_min;
            c->last_rx = std::chrono::steady_clock::now();
        } else {
            delay = delay > c->bo_max / 2 ? c->bo_max
                                          : delay * 2;
            if (delay > c->bo_max) delay = c->bo_max;
        }
    }
}

void stop_watchdog(Conn& c) {
    c.wd_stop.store(true);
    if (c.wd.joinable()) c.wd.join();
}

} // namespace
#endif

struct MqttService::Impl {
    Conn conns[TPLC_MQTT_HDL_MAX]{};
};

MqttService::MqttService() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcMqttService);
    svc_.abi_version = TPLC_MQTT_ABI_VERSION;
    svc_.host = this;
    svc_.mqtt_connect = [](void* s, const char* h, uint32_t p,
                           const TplcMqttConnectCfg* c, uint32_t* o)
        { return self(s)->connect(s, h, p, c, o); };
    svc_.mqtt_disconnect = [](void* s, uint32_t h)
        { return self(s)->disconnect(s, h); };
    svc_.mqtt_status = [](void* s, uint32_t h, TplcMqttStatus* o)
        { return self(s)->status(s, h, o); };
    svc_.mqtt_publish = [](void* s, uint32_t h, const char* t,
                           const uint8_t* p, uint32_t n,
                           uint32_t q, uint32_t r)
        { return self(s)->publish(s, h, t, p, n, q, r); };
    svc_.mqtt_subscribe = [](void* s, uint32_t h, const char* f,
                             uint32_t q)
        { return self(s)->subscribe(s, h, f, q); };
    svc_.mqtt_unsubscribe = [](void* s, uint32_t h, const char* f)
        { return self(s)->unsubscribe(s, h, f); };
    svc_.mqtt_receive = [](void* s, uint32_t h, TplcMqttRx* rx)
        { return self(s)->receive(s, h, rx); };
    svc_.mqtt_is_connected = [](void* s, uint32_t h)
        { return self(s)->isConnected(s, h); };
    svc_.mqtt_abort = [](void* s, uint32_t h)
        { return self(s)->abort(s, h); };
    svc_.mqtt_backoff = [](void* s, uint32_t h, uint32_t mn,
                           uint32_t mx, uint32_t e)
        { return self(s)->backoff(s, h, mn, mx, e); };
    svc_.mqtt_publish5 = [](void* s, uint32_t h, const char* t,
                            const uint8_t* p, uint32_t n,
                            uint32_t q, uint32_t r,
                            const TplcMqtt5PubProps* pr)
        { return self(s)->publish5(s, h, t, p, n, q, r, pr); };
    svc_.mqtt_subscribe5 = [](void* s, uint32_t h, const char* f,
                              uint32_t q, const TplcMqtt5SubProps* pr)
        { return self(s)->subscribe5(s, h, f, q, pr); };
}

MqttService::~MqttService() {
#ifdef TPLC_HAVE_MQTT
    for (Conn& c : impl_->conns) {
        stop_watchdog(c);
        if (c.conn) {
            MQTTClient_disconnect(c.conn, 500);
            MQTTClient_destroy(&c.conn);
        }
    }
#endif
}

int32_t MqttService::connect(void*, const char* host, uint32_t port,
                             const TplcMqttConnectCfg* cfg,
                             uint32_t* out_handle) {
    if (!host || !*host || !out_handle) return TPLC_MQTT_E_PARAM;
#ifdef TPLC_HAVE_MQTT
    for (uint32_t i = 0; i < TPLC_MQTT_HDL_MAX; ++i) {
        if (!impl_->conns[i].conn) {
            MQTTClient c;
            char uri[128];
            std::snprintf(uri, sizeof(uri), "%s://%s:%u",
                          (cfg && cfg->use_tls) ? "ssl" : "tcp",
                          host, port ? port : (cfg && cfg->use_tls
                                               ? 8883u : 1883u));
            std::set<int> fds_before = socket_fds();
            const bool v5 = cfg && cfg->version == 5;
            int rc;
            if (v5) {
                MQTTClient_createOptions copts =
                    MQTTClient_createOptions_initializer;
                copts.MQTTVersion = MQTTVERSION_5;
                rc = MQTTClient_createWithOptions(&c, uri,
                        cfg && cfg->client_id ? cfg->client_id
                                              : "tplc-client",
                        MQTTCLIENT_PERSISTENCE_NONE, nullptr, &copts);
            } else {
                rc = MQTTClient_create(&c, uri,
                        cfg && cfg->client_id ? cfg->client_id
                                              : "tplc-client",
                        MQTTCLIENT_PERSISTENCE_NONE, nullptr);
            }
            if (rc != MQTTCLIENT_SUCCESS) return TPLC_MQTT_E_CONN;
            MQTTClient_connectOptions opts = MQTTClient_connectOptions_initializer;
            opts.keepAliveInterval = int(cfg && cfg->keepalive_s
                                            ? cfg->keepalive_s : 60);
            opts.cleansession = cfg ? cfg->clean_session : 1;
            if (cfg && cfg->username) opts.username = cfg->username;
            if (cfg && cfg->password) opts.password = cfg->password;
            MQTTClient_willOptions will;
            if (cfg && cfg->will_topic && cfg->will_topic[0]) {
                will = MQTTClient_willOptions_initializer;
                will.topicName = cfg->will_topic;
                will.payload.len = int(cfg->will_payload_len);
                will.payload.data = cfg->will_payload;
                will.qos = int(cfg->will_qos);
                will.retained = cfg->will_retain ? 1 : 0;
                opts.will = &will;
            }
            MQTTClient_SSLOptions sslo;
            if (cfg && cfg->use_tls) {
                sslo = MQTTClient_SSLOptions_initializer;
                sslo.trustStore = cfg->tls_ca_file;
                sslo.keyStore = cfg->tls_cert_file;
                sslo.privateKey = cfg->tls_key_file;
                sslo.enableServerCertAuth = cfg->tls_insecure ? 0 : 1;
                opts.ssl = &sslo;
            }
            if (v5) {
                opts.MQTTVersion = MQTTVERSION_5;
                opts.cleansession = 0;
                opts.cleanstart = cfg ? cfg->clean_session : 1;
                ConnectSpec tmp;
                tmp.session_expiry_s = cfg->session_expiry_s;
                tmp.receive_max = cfg->receive_max;
                tmp.topic_alias_max = cfg->topic_alias_max;
                tmp.req_resp_info = cfg->req_resp_info;
                tmp.req_prob_info = cfg->req_prob_info;
                MQTTProperties props = MQTTProperties_initializer;
                fill_connect_props(tmp, props);
                MQTTResponse r = MQTTClient_connect5(c, &opts,
                                                     &props, nullptr);
                MQTTProperties_free(&props);
                rc = (r.reasonCode == MQTTREASONCODE_SUCCESS)
                         ? MQTTCLIENT_SUCCESS : MQTTCLIENT_FAILURE;
                MQTTResponse_free(r);
            } else {
                rc = MQTTClient_connect(c, &opts);
            }
            if (rc != MQTTCLIENT_SUCCESS) {
                MQTTClient_destroy(&c);
                return TPLC_MQTT_E_CONN;
            }
            Conn& cn = impl_->conns[i];
            cn.conn = c; cn.state = 2;
            cn.version = v5 ? 5 : 3;
            cn.last_msg_id = 0; cn.last_error = 0;
            cn.last_rx = std::chrono::steady_clock::now();
            /* owned copy for the backoff watchdog */
            cn.spec.host = host;
            cn.spec.port = port;
            if (cfg) {
                cn.spec.clean_session = cfg->clean_session;
                cn.spec.keepalive_s = cfg->keepalive_s;
                if (cfg->username) cn.spec.username = cfg->username;
                if (cfg->password) cn.spec.password = cfg->password;
                if (cfg->client_id) cn.spec.client_id = cfg->client_id;
                if (cfg->will_topic) cn.spec.will_topic = cfg->will_topic;
                if (cfg->will_payload && cfg->will_payload_len)
                    cn.spec.will_payload.assign(cfg->will_payload,
                        cfg->will_payload + cfg->will_payload_len);
                cn.spec.will_qos = cfg->will_qos;
                cn.spec.will_retain = cfg->will_retain;
                cn.spec.use_tls = cfg->use_tls;
                cn.spec.tls_insecure = cfg->tls_insecure;
                if (cfg->tls_ca_file) cn.spec.tls_ca_file = cfg->tls_ca_file;
                if (cfg->tls_cert_file)
                    cn.spec.tls_cert_file = cfg->tls_cert_file;
                cn.spec.version = cfg->version;
                cn.spec.session_expiry_s = cfg->session_expiry_s;
                cn.spec.receive_max = cfg->receive_max;
                cn.spec.topic_alias_max = cfg->topic_alias_max;
                cn.spec.req_resp_info = cfg->req_resp_info;
                cn.spec.req_prob_info = cfg->req_prob_info;
                if (cfg->tls_key_file)
                    cn.spec.tls_key_file = cfg->tls_key_file;
            }
            for (int fd : socket_fds())
                if (!fds_before.count(fd)) { cn.fd = fd; break; }
            *out_handle = i + 1;
            return TPLC_MQTT_OK;
        }
    }
    return TPLC_MQTT_E_HANDLE;
#else
    (void)host; (void)port; (void)cfg;
    return TPLC_MQTT_E_CONN;
#endif
}

int32_t MqttService::disconnect(void*, uint32_t h) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
    Conn& c = impl_->conns[h - 1];
#ifdef TPLC_HAVE_MQTT
    stop_watchdog(c);
    MQTTClient_disconnect(c.conn, 500);
    MQTTClient_destroy(&c.conn);
#endif
    c.conn = nullptr; c.state = 0; c.fd = -1; c.bo_en = false;
    return TPLC_MQTT_OK;
}

int32_t MqttService::status(void*, uint32_t h, TplcMqttStatus* out) {
    if (!out) return TPLC_MQTT_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn) {
        out->state = 0;
        return TPLC_MQTT_OK;
    }
    const Conn& c = impl_->conns[h - 1];
#ifdef TPLC_HAVE_MQTT
    /* live connection state — c.state stays 2 after connect, so
       report what Paho currently sees (updated by socket IO in
       receive/publish/etc). */
    out->state = MQTTClient_isConnected(c.conn) ? 2 : 0;
#else
    out->state = c.state;
#endif
    out->last_msg_id = c.last_msg_id;
    out->last_error = c.last_error;
    auto age = std::chrono::duration_cast<std::chrono::milliseconds>(
        std::chrono::steady_clock::now() - c.last_rx).count();
    out->last_msg_age_ms = age < 0 ? 0
        : (age > 0x7fffffffLL ? 0x7fffffffu : uint32_t(age));
    return TPLC_MQTT_OK;
}

int32_t MqttService::publish(void*, uint32_t h, const char* topic,
                             const uint8_t* payload, uint32_t len,
                             uint32_t qos, uint32_t retain) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
    if (!topic || !*topic || (!payload && len)) return TPLC_MQTT_E_PARAM;
#ifdef TPLC_HAVE_MQTT
    Conn& c = impl_->conns[h - 1];
    int mid = 0;
    int rc;
    if (c.version == 5) {
        MQTTResponse r = MQTTClient_publish5(c.conn, topic, int(len),
                          payload, int(qos), int(retain), nullptr, &mid);
        rc = (r.reasonCode == MQTTREASONCODE_SUCCESS)
                 ? MQTTCLIENT_SUCCESS : MQTTCLIENT_FAILURE;
        MQTTResponse_free(r);
    } else {
        rc = MQTTClient_publish(c.conn, topic, int(len),
                 payload, int(qos), int(retain), &mid);
    }
    if (rc != MQTTCLIENT_SUCCESS) {
        c.last_error = rc;
        return TPLC_MQTT_E_PROTO;
    }
    c.last_msg_id = uint32_t(mid);
    return TPLC_MQTT_OK;
#else
    return TPLC_MQTT_E_CONN;
#endif
}

int32_t MqttService::publish5(void*, uint32_t h, const char* topic,
                              const uint8_t* payload, uint32_t len,
                              uint32_t qos, uint32_t retain,
                              const TplcMqtt5PubProps* props) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
    if (!topic || !*topic || (!payload && len)) return TPLC_MQTT_E_PARAM;
#ifdef TPLC_HAVE_MQTT
    Conn& c = impl_->conns[h - 1];
    if (c.version != 5) return TPLC_MQTT_E_PARAM;
    MQTTProperties p = MQTTProperties_initializer;
    MQTTProperty prop;
    memset(&prop, 0, sizeof(prop));
    auto add_str = [&](enum MQTTPropertyCodes id, const char* v) {
        if (!v || !*v) return;
        prop.identifier = id;
        prop.value.data.len = int(std::strlen(v));
        prop.value.data.data = const_cast<char*>(v);
        MQTTProperties_add(&p, &prop);
    };
    if (props) {
        if (props->payload_format) {
            prop.identifier = MQTTPROPERTY_CODE_PAYLOAD_FORMAT_INDICATOR;
            prop.value.byte = 1;
            MQTTProperties_add(&p, &prop);
        }
        if (props->message_expiry_s) {
            prop.identifier = MQTTPROPERTY_CODE_MESSAGE_EXPIRY_INTERVAL;
            prop.value.integer4 = props->message_expiry_s;
            MQTTProperties_add(&p, &prop);
        }
        add_str(MQTTPROPERTY_CODE_CONTENT_TYPE, props->content_type);
        add_str(MQTTPROPERTY_CODE_RESPONSE_TOPIC, props->response_topic);
        if (props->correlation_data && props->correlation_len) {
            prop.identifier = MQTTPROPERTY_CODE_CORRELATION_DATA;
            prop.value.data.len = int(props->correlation_len);
            prop.value.data.data =
                const_cast<char*>(reinterpret_cast<const char*>(
                    props->correlation_data));
            MQTTProperties_add(&p, &prop);
        }
        /* user properties: "k1=v1;k2=v2" pairs */
        if (props->user_properties && props->user_properties[0]) {
            char buf[512];
            std::strncpy(buf, props->user_properties, sizeof(buf) - 1);
            buf[sizeof(buf) - 1] = 0;
            for (char* pair = std::strtok(buf, ";"); pair;
                 pair = std::strtok(nullptr, ";")) {
                char* eq = std::strchr(pair, '=');
                if (!eq) continue;
                *eq = 0;
                prop.identifier = MQTTPROPERTY_CODE_USER_PROPERTY;
                prop.value.data.len = int(std::strlen(pair));
                prop.value.data.data = pair;
                prop.value.value.len = int(std::strlen(eq + 1));
                prop.value.value.data = eq + 1;
                MQTTProperties_add(&p, &prop);
            }
        }
    }
    MQTTClient_deliveryToken dt = 0;
    MQTTResponse r = MQTTClient_publish5(c.conn, topic, int(len),
                      payload, int(qos), int(retain), &p, &dt);
    MQTTProperties_free(&p);
    int rc = (r.reasonCode == MQTTREASONCODE_SUCCESS)
                 ? MQTTCLIENT_SUCCESS : MQTTCLIENT_FAILURE;
    MQTTResponse_free(r);
    if (rc != MQTTCLIENT_SUCCESS) {
        c.last_error = rc;
        return TPLC_MQTT_E_PROTO;
    }
    c.last_msg_id = uint32_t(dt);
    return TPLC_MQTT_OK;
#else
    return TPLC_MQTT_E_CONN;
#endif
}

int32_t MqttService::subscribe(void*, uint32_t h, const char* filter,
                               uint32_t qos) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
    if (!filter || !*filter) return TPLC_MQTT_E_PARAM;
#ifdef TPLC_HAVE_MQTT
    Conn& c = impl_->conns[h - 1];
    int rc;
    if (c.version == 5) {
        MQTTSubscribe_options opts = MQTTSubscribe_options_initializer;
        MQTTResponse r = MQTTClient_subscribe5(c.conn, filter, int(qos),
                                               &opts, nullptr);
        rc = (r.reasonCode == MQTTREASONCODE_SUCCESS)
                 ? MQTTCLIENT_SUCCESS : MQTTCLIENT_FAILURE;
        MQTTResponse_free(r);
    } else {
        rc = MQTTClient_subscribe(c.conn, filter, int(qos));
    }
    if (rc != MQTTCLIENT_SUCCESS) {
        c.last_error = rc;
        return TPLC_MQTT_E_PROTO;
    }
    return TPLC_MQTT_OK;
#else
    return TPLC_MQTT_E_CONN;
#endif
}

int32_t MqttService::subscribe5(void*, uint32_t h, const char* filter,
                                uint32_t qos, const TplcMqtt5SubProps* sp) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
    if (!filter || !*filter) return TPLC_MQTT_E_PARAM;
#ifdef TPLC_HAVE_MQTT
    Conn& c = impl_->conns[h - 1];
    if (c.version != 5) return TPLC_MQTT_E_PARAM;
    MQTTSubscribe_options opts = MQTTSubscribe_options_initializer;
    if (sp) {
        opts.noLocal = sp->no_local ? 1 : 0;
        opts.retainAsPublished = sp->retain_as_published ? 1 : 0;
        opts.retainHandling = (unsigned char)sp->retain_handling;
    }
    MQTTProperties p = MQTTProperties_initializer;
    MQTTProperty prop;
    memset(&prop, 0, sizeof(prop));
    if (sp && sp->subscription_id) {
        prop.identifier = MQTTPROPERTY_CODE_SUBSCRIPTION_IDENTIFIER;
        prop.value.integer4 = sp->subscription_id;
        MQTTProperties_add(&p, &prop);
    }
    if (sp && sp->user_properties && sp->user_properties[0]) {
        char buf[512];
        std::strncpy(buf, sp->user_properties, sizeof(buf) - 1);
        buf[sizeof(buf) - 1] = 0;
        for (char* pair = std::strtok(buf, ";"); pair;
             pair = std::strtok(nullptr, ";")) {
            char* eq = std::strchr(pair, '=');
            if (!eq) continue;
            *eq = 0;
            prop.identifier = MQTTPROPERTY_CODE_USER_PROPERTY;
            prop.value.data.len = int(std::strlen(pair));
            prop.value.data.data = pair;
            prop.value.value.len = int(std::strlen(eq + 1));
            prop.value.value.data = eq + 1;
            MQTTProperties_add(&p, &prop);
        }
    }
    MQTTResponse r = MQTTClient_subscribe5(c.conn, filter, int(qos),
                                           &opts, &p);
    MQTTProperties_free(&p);
    int rc = (r.reasonCode == MQTTREASONCODE_SUCCESS)
                 ? MQTTCLIENT_SUCCESS : MQTTCLIENT_FAILURE;
    MQTTResponse_free(r);
    if (rc != MQTTCLIENT_SUCCESS) {
        c.last_error = rc;
        return TPLC_MQTT_E_PROTO;
    }
    return TPLC_MQTT_OK;
#else
    return TPLC_MQTT_E_CONN;
#endif
}

int32_t MqttService::unsubscribe(void*, uint32_t h, const char* filter) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
    if (!filter || !*filter) return TPLC_MQTT_E_PARAM;
#ifdef TPLC_HAVE_MQTT
    Conn& c = impl_->conns[h - 1];
    int rc;
    if (c.version == 5) {
        MQTTResponse r = MQTTClient_unsubscribe5(c.conn, filter, nullptr);
        rc = (r.reasonCode == MQTTREASONCODE_SUCCESS)
                 ? MQTTCLIENT_SUCCESS : MQTTCLIENT_FAILURE;
        MQTTResponse_free(r);
    } else {
        rc = MQTTClient_unsubscribe(c.conn, filter);
    }
    if (rc != MQTTCLIENT_SUCCESS) {
        c.last_error = rc;
        return TPLC_MQTT_E_PROTO;
    }
    return TPLC_MQTT_OK;
#else
    return TPLC_MQTT_E_CONN;
#endif
}

int32_t MqttService::receive(void*, uint32_t h, TplcMqttRx* rx) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
    if (!rx || !rx->topic || !rx->payload) return TPLC_MQTT_E_PARAM;
#ifdef TPLC_HAVE_MQTT
    Conn& c = impl_->conns[h - 1];
    char* topic = nullptr; int tlen = 0;
    MQTTClient_message* msg = nullptr;
    /* bounded nonblocking poll — 0 ms wait */
    int rc = MQTTClient_receive(c.conn, &topic, &tlen, &msg, 0);
    if (rc != MQTTCLIENT_SUCCESS && rc != MQTTCLIENT_TOPICNAME_TRUNCATED) {
        c.last_error = rc;
        return TPLC_MQTT_E_PROTO;
    }
    if (!msg) {
        if (topic) MQTTClient_free(topic);
        return TPLC_MQTT_E_NOMSG;
    }
    c.last_rx = std::chrono::steady_clock::now();
    rx->topic_len = uint32_t(tlen < 0 ? int(std::strlen(topic)) : tlen);
    rx->payload_len = uint32_t(msg->payloadlen);
    rx->qos = uint32_t(msg->qos);
    rx->message_id = uint32_t(msg->msgid);
    rx->retained = uint32_t(msg->retained);
    uint32_t tn = rx->topic_len < rx->topic_cap ? rx->topic_len : rx->topic_cap;
    std::memcpy(rx->topic, topic, tn);
    uint32_t pn = rx->payload_len < rx->payload_cap ? rx->payload_len
                                                  : rx->payload_cap;
    std::memcpy(rx->payload, msg->payload, pn);
    MQTTClient_freeMessage(&msg);
    MQTTClient_free(topic);
    return TPLC_MQTT_OK;
#else
    return TPLC_MQTT_E_CONN;
#endif
}

int32_t MqttService::isConnected(void*, uint32_t h) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return 0;
#ifdef TPLC_HAVE_MQTT
    return MQTTClient_isConnected(impl_->conns[h - 1].conn) ? 1 : 0;
#else
    return 0;
#endif
}

int32_t MqttService::abort(void*, uint32_t h) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
    Conn& c = impl_->conns[h - 1];
#ifdef TPLC_HAVE_MQTT
    /* Drop the TCP connection without an MQTT DISCONNECT so the
       broker publishes the registered last-will.  Paho has no abort
       API and MQTTClient_destroy on a live client corrupts the
       shared client state, so shut the socket down directly and let
       Paho observe the EOF before freeing the (now disconnected)
       client. */
    if (c.fd > 0) shutdown(c.fd, SHUT_RDWR);
    for (int i = 0; i < 50 && MQTTClient_isConnected(c.conn); ++i)
        usleep(10000);
    stop_watchdog(c);
    MQTTClient_destroy(&c.conn);
#endif
    c.conn = nullptr; c.state = 0; c.fd = -1; c.bo_en = false;
    return TPLC_MQTT_OK;
}

int32_t MqttService::backoff(void*, uint32_t h, uint32_t min_ms,
                             uint32_t max_ms, uint32_t enable) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
#ifdef TPLC_HAVE_MQTT
    Conn& c = impl_->conns[h - 1];
    if (!enable) {
        c.bo_en = false;
        return TPLC_MQTT_OK;
    }
    c.bo_min = min_ms ? min_ms : 1000;
    c.bo_max = max_ms ? max_ms : c.bo_min;
    if (c.bo_max < c.bo_min) c.bo_max = c.bo_min;
    c.bo_en = true;
    if (!c.wd.joinable()) {
        c.wd_stop.store(false);
        c.wd = std::thread(backoff_loop, &c);
    }
    return TPLC_MQTT_OK;
#else
    (void)min_ms; (void)max_ms;
    return TPLC_MQTT_E_CONN;
#endif
}

} // namespace tplc
