// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_DELEGATES_CONNECTIONS_DELEGATE_H_
#define HEBO_SRC_FRAMES_DELEGATES_CONNECTIONS_DELEGATE_H_

#include <QStyledItemDelegate>

namespace hebo {

class ConnectionsDelegate : public QStyledItemDelegate {
  Q_OBJECT
 public:
  explicit ConnectionsDelegate(QObject* parent = nullptr);

  void paint(QPainter* painter, const QStyleOptionViewItem& option, const QModelIndex& index) const override;

  [[nodiscard]] QSize sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const override;
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_DELEGATES_CONNECTIONS_DELEGATE_H_
