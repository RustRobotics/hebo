// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_STATE_H_
#define HEBOUI_SRC_MQTT_CONNECTION_STATE_H_

#include <QDebug>

namespace hebo {

enum class ConnectionState : uint8_t {
  kDisconnected = 0,
  kConnecting = 1,
  kConnected = 2,
  kConnectFailed = 3,
  kDisconnecting = 4,
};

QDebug operator<<(QDebug stream, ConnectionState state);

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECTION_STATE_H_
