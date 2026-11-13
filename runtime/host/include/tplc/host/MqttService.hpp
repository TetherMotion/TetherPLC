/* MqttService.hpp — tplc-mqtt MQTT client host service.  Wraps an
 * MQTT client (libpaho-mqtt3c / Eclipse Paho synchronous MQTTClient,
 * opt-in — inert when TPLC_HAVE_MQTT is off) behind the tplc-mqtt
 * vtable.  The PLC-side builtin FB_Mqtt dispatches ops to it via
 * getService("tplc-mqtt").  Each op is bounded so a cyclic scan
 * can't block past the scan-stall watchdog.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_MQTT_SERVICE_HPP
#define TPLC_MQTT_SERVICE_HPP

#include <tplc/tplc_mqtt.h>
#include <memory>

namespace tplc {

class MqttService {
public:
    MqttService();
    ~MqttService();

    MqttService(const MqttService&) = delete;
    MqttService& operator=(const MqttService&) = delete;

    const TplcMqttService& table() const { return svc_; }
    bool active() const {
#ifdef TPLC_HAVE_MQTT
        return true;
#else
        return false;
#endif
    }

private:
    static MqttService* self(void* s) {
        return static_cast<MqttService*>(
            reinterpret_cast<TplcMqttService*>(s)->host);
    }

    int32_t connect(void*, const char* host, uint32_t port,
                    const TplcMqttConnectCfg* cfg, uint32_t* out_handle);
    int32_t disconnect(void*, uint32_t h);
    int32_t status(void*, uint32_t h, TplcMqttStatus* out);
    int32_t publish(void*, uint32_t h, const char* topic,
                    const uint8_t* payload, uint32_t len,
                    uint32_t qos, uint32_t retain);
    int32_t subscribe(void*, uint32_t h, const char* filter, uint32_t qos);
    int32_t unsubscribe(void*, uint32_t h, const char* filter);
    int32_t receive(void*, uint32_t h, TplcMqttRx* rx);
    int32_t isConnected(void*, uint32_t h);
    int32_t abort(void*, uint32_t h);
    int32_t backoff(void*, uint32_t h, uint32_t min_ms,
                    uint32_t max_ms, uint32_t enable);
    int32_t publish5(void*, uint32_t h, const char* topic,
                     const uint8_t* payload, uint32_t len,
                     uint32_t qos, uint32_t retain,
                     const TplcMqtt5PubProps* props);
    int32_t subscribe5(void*, uint32_t h, const char* filter,
                       uint32_t qos, const TplcMqtt5SubProps* props);

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcMqttService svc_{};
};

} /* namespace tplc */
#endif /* TPLC_MQTT_SERVICE_HPP */
