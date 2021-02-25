// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/connection_info.h"

#include <QDebug>
#include <QJsonDocument>
#include <QJsonArray>
#include <QJsonObject>

#include "base/file.h"

namespace hebo {
namespace {

constexpr const char* kKeyVersion = "version";
constexpr const int32_t kCurrentVersion = 1;
constexpr const char* kKeyItems = "items";
constexpr const char* kKeyName = "name";
constexpr const char* kKeyClientId = "clientId";
constexpr const char* kKeyProtocol = "protocol";
constexpr const char* kKeyHost = "host";
constexpr const char* kKeyPort = "port";
constexpr const char* kKeyUsername = "username";
constexpr const char* kKeyPassword = "password";
constexpr const char* kKeyTls = "tls";

constexpr const char* kKeyQoS = "qos";
constexpr const char* kKeyCleanSession = "cleanSession";

bool parseItems(const QJsonArray& array, ConnectionInfoList& list) {
  for (const auto& item : array) {
    const QJsonObject object = item.toObject();
    ConnectionInfo info;
    info.name = object.value(kKeyName).toString();
    info.client_id = object.value(kKeyClientId).toString();
    info.protocol = object.value(kKeyProtocol).toString();
    info.host = object.value(kKeyHost).toString();
    info.port = object.value(kKeyPort).toInt();
    info.qos = static_cast<QoS>(object.value(kKeyQoS).toInt());
    info.username = object.value(kKeyUsername).toString();
    info.password = object.value(kKeyPassword).toString();
    info.with_tls = object.value(kKeyTls).toBool();
    info.clean_session = object.value(kKeyCleanSession).toBool();
    info.description = generateConnDescription(info);
    list.append(info);
  }
  return true;
}

}  // namespace hebo

QString generateConnDescription(const ConnectionInfo& info) {
  return QString("%1@%2:%3").arg(info.name).arg(info.host).arg(info.port);
}

QDebug operator<<(QDebug stream, const ConnectionInfo& info) {
  stream << "ConnInfo {"
         << "\n  name:" << info.name
         << "\n  clientId:" << info.client_id
         << "\n  host:" << info.host
         << "\n  port:" << info.port
         << "\n  username:" << info.username
         << "\n  password:" << info.password
         << "\n  tls:" << info.with_tls
         << "\n  cleanSession:" << info.clean_session
         << "\n  description:" << info.description
         << "\n  state:" << info.state
         << "}";
  return stream;
}

bool parseConnectionInfos(const QString& file, ConnectionInfoList& list) {
  const QByteArray contents = readBinaryFile(file);
  const QJsonDocument document{QJsonDocument::fromJson(contents)};
  if (!document.isObject()) {
    qWarning() << "Invalid conn info file:" << file;
    return false;
  }
  const QJsonObject root_object = document.object();
  const int32_t version =  root_object.value(kKeyVersion).toInt();

  if (version == kCurrentVersion) {
      return parseItems(root_object.value(kKeyItems).toArray(), list);
  } else {
    Q_UNIMPLEMENTED();
    // TODO(Shaohua): Do migration
    return false;
  }
}

QJsonObject dumpConnectionInfo(const ConnectionInfo& info) {
  QJsonObject object;
  object.insert(kKeyName, info.name);
  object.insert(kKeyClientId, info.client_id);
  object.insert(kKeyProtocol, info.protocol);
  object.insert(kKeyHost, info.host);
  object.insert(kKeyPort, info.port);
  object.insert(kKeyQoS, static_cast<int>(info.qos));
  object.insert(kKeyUsername, info.username);
  object.insert(kKeyPassword, info.password);
  object.insert(kKeyTls, info.with_tls);
  object.insert(kKeyCleanSession, info.clean_session);
  return object;
}

bool dumpConnectionInfos(const QString& file, const ConnectionInfoList& list) {
  QJsonArray array;
  for (const auto& info : list) {
    array.append(dumpConnectionInfo(info));
  }

  QJsonObject root_object;
  root_object.insert(kKeyVersion, kCurrentVersion);
  root_object.insert(kKeyItems, array);
  QJsonDocument document;
  document.setObject(root_object);
  const QByteArray contents = document.toJson();
  return writeBinaryFile(file, contents);
}

}  // namespace hebo
