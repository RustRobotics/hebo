// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_MODELS_SOFTWARE_LICENSE_MODEL_H_
#define HEBO_SRC_FRAMES_MODELS_SOFTWARE_LICENSE_MODEL_H_

#include <QAbstractTableModel>

#include "formats/license_parser.h"

namespace hebo {

class SoftwareLicenseModel : public QAbstractTableModel {
  Q_OBJECT
 public:
  enum RoleList : int32_t {
    kNameRole = Qt::UserRole + 1,
    kVersionRole,
    kUrlRole,
    kLicenseRole,
    kLicenseUrlRole,
  };
  Q_ENUM(RoleList);

  explicit SoftwareLicenseModel(QObject* parent = nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] int columnCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

  [[nodiscard]] QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

  [[nodiscard]] QHash<int, QByteArray> roleNames() const override;

  static constexpr int kSoftwareColumn = 0;
  static constexpr int kLicenseColumn = 1;

 private:
  SoftwareLicenseList list_{};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_MODELS_SOFTWARE_LICENSE_MODEL_H_
