// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_MODELS_PAYLOAD_TYPE_MODEL_H_
#define HEBO_SRC_FRAMES_MODELS_PAYLOAD_TYPE_MODEL_H_

#include <QAbstractListModel>

#include "formats/payload_type.h"

namespace hebo {

class PayloadTypeModel : public QAbstractListModel {
  Q_OBJECT
 public:
  enum RoleList : int32_t {
    kNameRole = Qt::UserRole + 1,
    kIdRole,
  };
  Q_ENUM(RoleList);

  explicit PayloadTypeModel(QObject* parent = nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

 private:
  QStringList type_list_;
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_MODELS_PAYLOAD_TYPE_MODEL_H_
