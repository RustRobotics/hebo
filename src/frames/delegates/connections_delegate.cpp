// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/delegates/connections_delegate.h"

#include <QPainter>
#include <QStyleOptionViewItem>

#include "mqtt/connections_model.h"

namespace hebo {

ConnectionsDelegate::ConnectionsDelegate(QObject* parent) : QStyledItemDelegate(parent) {

}

void ConnectionsDelegate::paint(QPainter* painter, const QStyleOptionViewItem& option,
                                const QModelIndex& index) const {
  const QString description = index.data(ConnectionsModel::kDescriptionRole).toString();
  painter->drawText(option.rect, description);
}

QSize ConnectionsDelegate::sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const {
  return QStyledItemDelegate::sizeHint(option, index);
}

}  // namespace hebo