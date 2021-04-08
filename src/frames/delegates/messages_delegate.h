// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_DELEGATES_MESSAGES_DELEGATE_H_
#define HEBO_SRC_FRAMES_DELEGATES_MESSAGES_DELEGATE_H_

#include <QStyledItemDelegate>

namespace hebo {

class MessagesDelegate : public QStyledItemDelegate {
  Q_OBJECT
 public:
  explicit MessagesDelegate(QObject* parent = nullptr);

  void paint(QPainter* painter, const QStyleOptionViewItem& option, const QModelIndex& index) const override;

  [[nodiscard]] QSize sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const override;

 private:
  QColor pub_bg_color_;
  QColor sub_bg_color_;
  QColor pub_font_color_;
  QColor sub_font_color_;
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_DELEGATES_MESSAGES_DELEGATE_H_
