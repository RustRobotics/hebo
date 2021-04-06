// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FORMATS_LICENSE_PARSER_H_
#define HEBO_SRC_FORMATS_LICENSE_PARSER_H_

#include <QDebug>
#include <QString>
#include <QVector>

namespace hebo {

struct SoftwareLicense {
  QString name{};
  QString version{};
  QString url{};
  QString license{};
  QString license_url{};
};

using SoftwareLicenseList = QVector<SoftwareLicense>;

QDebug operator<<(QDebug stream, const SoftwareLicense& license);

SoftwareLicenseList parseAppLicense(const QString& file);

}  // namespace hebo

#endif  // HEBO_SRC_FORMATS_LICENSE_PARSER_H_
