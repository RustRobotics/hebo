// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_MODELS_QOS_MODEL_H_
#define HEBO_SRC_FRAMES_MODELS_QOS_MODEL_H_

#include <QAbstractListModel>

#include "formats/connect_config.h"

namespace hebo {

class QoSModel : public QAbstractListModel {
  Q_OBJECT
 public:
  enum RoleList : int32_t {
    kNameRole = Qt::DisplayRole,
    kIdRole = Qt::UserRole + 1,
  };
  Q_ENUM(RoleList);

  explicit QoSModel(QObject* parent = nullptr);

  [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

  [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_MODELS_QOS_MODEL_H_
