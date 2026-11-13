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

#include <cstdlib>
#include <cstring>
#include <set>
#include <string>
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

struct Conn {
#ifdef TPLC_HAVE_MQTT
    MQTTClient conn = nullptr;
    int32_t  state = 0;
    uint32_t last_msg_id = 0;
    int32_t  last_error = 0;
    int      fd = -1;
#else
    void*    conn = nullptr;
    int32_t  state = 0;
    uint32_t last_msg_id = 0;
    int32_t  last_error = 0;
#endif
};

} // namespace

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
}

MqttService::~MqttService() {
#ifdef TPLC_HAVE_MQTT
    for (Conn& c : impl_->conns)
        if (c.conn) {
            MQTTClient_disconnect(c.conn, 500);
            MQTTClient_destroy(&c.conn);
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
            std::snprintf(uri, sizeof(uri), "tcp://%s:%u",
                          host, port ? port : 1883);
            std::set<int> fds_before = socket_fds();
            int rc = MQTTClient_create(&c, uri,
                        cfg && cfg->client_id ? cfg->client_id : "tplc-client",
                        MQTTCLIENT_PERSISTENCE_NONE, nullptr);
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
            rc = MQTTClient_connect(c, &opts);
            if (rc != MQTTCLIENT_SUCCESS) {
                MQTTClient_destroy(&c);
                return TPLC_MQTT_E_CONN;
            }
            Conn& cn = impl_->conns[i];
            cn.conn = c; cn.state = 2;
            cn.last_msg_id = 0; cn.last_error = 0;
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
    MQTTClient_disconnect(c.conn, 500);
    MQTTClient_destroy(&c.conn);
#endif
    c.conn = nullptr; c.state = 0; c.fd = -1;
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
    out->state = c.state;
    out->last_msg_id = c.last_msg_id;
    out->last_error = c.last_error;
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
    int mid;
    int rc = MQTTClient_publish(c.conn, topic, int(len),
                 payload, int(qos), int(retain), &mid);
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

int32_t MqttService::subscribe(void*, uint32_t h, const char* filter,
                               uint32_t qos) {
    if (h == 0 || h > TPLC_MQTT_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_MQTT_E_HANDLE;
    if (!filter || !*filter) return TPLC_MQTT_E_PARAM;
#ifdef TPLC_HAVE_MQTT
    Conn& c = impl_->conns[h - 1];
    int rc = MQTTClient_subscribe(c.conn, filter, int(qos));
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
    int rc = MQTTClient_unsubscribe(impl_->conns[h - 1].conn, filter);
    if (rc != MQTTCLIENT_SUCCESS) {
        impl_->conns[h - 1].last_error = rc;
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
    MQTTClient_destroy(&c.conn);
#endif
    c.conn = nullptr; c.state = 0; c.fd = -1;
    return TPLC_MQTT_OK;
}

} // namespace tplc
