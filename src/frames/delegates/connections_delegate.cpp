// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/delegates/connections_delegate.h"

namespace hebo {

ConnectionsDelegate::ConnectionsDelegate(QObject* parent) : QStyledItemDelegate(parent) {

}

void ConnectionsDelegate::paint(QPainter* painter, const QStyleOptionViewItem& option,
                                const QModelIndex& index) const {
  QStyledItemDelegate::paint(painter, option, index);
}

QSize ConnectionsDelegate::sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const {
  return QStyledItemDelegate::sizeHint(option, index);
}

}  // namespace hebo