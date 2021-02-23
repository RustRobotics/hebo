// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/conn_info.h"

namespace hebo {

QDebug operator<<(QDebug stream, const ConnInfo& info) {
  stream << "ConnInfo {"
         << "\n  name:" << info.name()
//         << "\n  clientId:" << info.client_id
//         << "\n  host:" << info.host
//         << "\n  port:" << info.port
//         << "\n  username:" << info.username
//         << "\n  password:" << info.password
//         << "\n  tls:" << info.with_tls
//         << "\n  cleanSession:" << info.clean_session
         << "}";
  return stream;
}

ConnInfo::ConnInfo(QObject* parent) : QObject(parent) {

}


}  // namespace hebo
