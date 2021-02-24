// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONN_STATE_H_
#define HEBOUI_SRC_MQTT_CONN_STATE_H_

#include <QDebug>

namespace hebo {

enum class ConnectState : uint8_t {
  kDisconnected,
  kConnecting,
  kConnected,
  kConnectFailed,
  kDisconnecting,
};

QDebug operator<<(QDebug stream, ConnectState state);

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONN_STATE_H_