// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_STATE_H_
#define HEBOUI_SRC_MQTT_CONNECTION_STATE_H_

#include <QtGlobal>
#include <QObject>

namespace hebo {

enum ConnectionState : int32_t {
  ConnectionDisconnected = 0,
  ConnectionConnecting = 1,
  ConnectionConnected = 2,
  ConnectionConnectFailed = 3,
  ConnectionDisconnecting = 4,
};
Q_ENUM_NS(ConnectionState);

}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECTION_STATE_H_
