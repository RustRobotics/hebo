// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_MQTT_CONNECTION_INFO_H_
#define HEBOUI_SRC_MQTT_CONNECTION_INFO_H_

#include <QDebug>
#include <QObject>

namespace hebo {



struct ConnectionInfo{

  ConnectionState state{ConnectionState::ConnectionDisconnected};
};



}  // namespace hebo

#endif  // HEBOUI_SRC_MQTT_CONNECTION_INFO_H_
