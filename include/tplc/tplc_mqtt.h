/* tplc_mqtt.h — C ABI for the tplc-mqtt MQTT client host service.
 * The PLC-side builtin FB_Mqtt dispatches ops to this vtable via
 * getService("tplc-mqtt"); the service wraps an MQTT client
 * (libpaho-mqtt3c / Eclipse Paho, opt-in).  All calls are
 * synchronous; the vtable is registered once per process.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_MQTT_H
#define TPLC_MQTT_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_MQTT_ABI_VERSION 1u
/* a connect may produce up to this many concurrent subscriptions */
#define TPLC_MQTT_SUB_MAX   16u
#define TPLC_MQTT_HDL_MAX   16u

/* ---- status struct (48 bytes, 8-byte aligned) ---- */
typedef struct TplcMqttStatus {
    int32_t  state;        /* 0 disconnected, 1 connecting, 2 connected */
    uint32_t conn_flags;   /* MQTT connack session-present / return code */
    uint32_t last_msg_id;  /* last pub/sub message id */
    uint32_t last_qos;     /* last pub/sub qos */
    int32_t  last_error;   /* last MQTTReasonCode-ish error */
    uint32_t reserved[7];  /* pad to 48 */
} TplcMqttStatus;

/* ---- connection config ---- */
typedef struct TplcMqttConnectCfg {
    int32_t  clean_session;   /* 1 = clean start (MQTT3 cleanSession /
                                  MQTT5 clean start) */
    uint32_t keepalive_s;     /* keepalive seconds (default 60) */
    uint32_t version;         /* 3 = v3.1.1, 5 = v5 (service maps) */
    uint32_t connect_timeout_ms;
    const char* username;     /* NUL-terminated or NULL */
    const char* password;     /* NUL-terminated or NULL */
    const char* client_id;    /* NUL-terminated or NULL (auto) */
} TplcMqttConnectCfg;

/* ---- received-message struct ---- */
typedef struct TplcMqttRx {
    /* in: buffer capacity; out: actual sizes */
    uint32_t topic_cap;
    uint32_t payload_cap;
    uint32_t topic_len;     /* out */
    uint32_t payload_len;   /* out */
    uint32_t qos;           /* out */
    uint32_t message_id;    /* out */
    uint32_t retained;      /* out: 1 = broker-stored retained msg */
    /* out buffers follow the struct (32/40, 8-byte aligned) */
    char*    topic;
    uint8_t* payload;
} TplcMqttRx;

/* ---- service vtable ----
 * Returned by getService("tplc-mqtt").  All entries take the service
 * `this` as first arg. */
typedef struct TplcMqttService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* ops */
    int32_t (*mqtt_connect)(void*, const char* host, uint32_t port,
                            const TplcMqttConnectCfg* cfg,
                            uint32_t* out_handle);
    int32_t (*mqtt_disconnect)(void*, uint32_t h);
    int32_t (*mqtt_status)(void*, uint32_t h, TplcMqttStatus* out);

    /* publish: topic + payload + qos + retain */
    int32_t (*mqtt_publish)(void*, uint32_t h, const char* topic,
                            const uint8_t* payload, uint32_t len,
                            uint32_t qos, uint32_t retain);

    /* subscribe/unsubscribe to a topic filter */
    int32_t (*mqtt_subscribe)(void*, uint32_t h, const char* filter,
                              uint32_t qos);
    int32_t (*mqtt_unsubscribe)(void*, uint32_t h, const char* filter);

    /* receive: poll for a pending publish (nonblocking).  Returns 0
       when nothing arrived, >0 bytes when a message landed in rx. */
    int32_t (*mqtt_receive)(void*, uint32_t h, TplcMqttRx* rx);

    /* isConnected check (cheap, no IO) */
    int32_t (*mqtt_is_connected)(void*, uint32_t h);

    void* reserved[8];
} TplcMqttService;

/* status / error codes */
#define TPLC_MQTT_OK           0
#define TPLC_MQTT_E_PARAM     -1
#define TPLC_MQTT_E_HANDLE    -2
#define TPLC_MQTT_E_CONN      -3
#define TPLC_MQTT_E_PROTO     -4
#define TPLC_MQTT_E_TIMEOUT   -5
#define TPLC_MQTT_E_NOBUF     -6
#define TPLC_MQTT_E_NOMSG      1   /* receive: nothing pending */

#ifdef __cplusplus
}
#endif
#endif /* TPLC_MQTT_H */
