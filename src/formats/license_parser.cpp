// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "formats/license_parser.h"

#include <QDebug>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>

#include "base/file.h"

namespace hebo {
namespace {

constexpr const char* kKeyName = "name";
constexpr const char* kKeyVersion = "version";
constexpr const char* kKeyUrl = "url";
constexpr const char* kKeyLicense = "license";
constexpr const char* kKeyLicenseUrl = "licenseUrl";

}  // namespace

QDebug operator<<(QDebug stream, const SoftwareLicense& license) {
  stream << "AppLicense {"
         << "\n  name:" << license.name
         << "\n  version:" << license.version
         << "\n  url:" << license.url
         << "\n  license:" << license.license
         << "\n  licenseUrl:" << license.license_url
         << "\n}";
  return stream;
}

SoftwareLicenseList parseAppLicense(const QString& file) {
  SoftwareLicenseList list{};

  const QByteArray bytes = readBinaryFile(file);
  const QJsonDocument document = QJsonDocument::fromJson(bytes);
  if (!document.isArray()) {
    qWarning() << "Failed to parse app license file:" << file;
    return list;
  }

  const QJsonArray array = document.array();
  for (const QJsonValue& item : array) {
    const QJsonObject object = item.toObject();
    SoftwareLicense license;
    license.name = object.value(kKeyName).toString();
    license.version = object.value(kKeyVersion).toString();
    license.url = object.value(kKeyUrl).toString();
    license.license = object.value(kKeyLicense).toString();
    license.license_url = object.value(kKeyLicenseUrl).toString();
    list.append(license);
  }

  return list;
}

}  // namespace hebo