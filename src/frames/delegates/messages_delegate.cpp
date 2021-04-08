// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/delegates/messages_delegate.h"

#include <QPainter>
#include <QRegion>

#include "mqtt/message_stream_model.h"

namespace hebo {

MessagesDelegate::MessagesDelegate(QObject* parent)
  : QStyledItemDelegate(parent),
    pub_bg_color_("#34c388"),
    sub_bg_color_("#edeef3"),
    pub_font_color_("#868A9F"),
    sub_font_color_("#B2B5C0") {
}

void MessagesDelegate::paint(QPainter* painter, const QStyleOptionViewItem& option, const QModelIndex& index) const {
  const QString topic = index.data(MessageStreamModel::kTopicRole).toString();
  const QoS qos = index.data(MessageStreamModel::kQoSRole).value<QoS>();
  const bool is_publish = index.data(MessageStreamModel::kIsPublishRole).toBool();
  const auto timestamp = index.data(MessageStreamModel::kTimestampRole).value<QDateTime>();
  const QByteArray payload = index.data(MessageStreamModel::kPayloadRole).toByteArray();

  QRect rect{option.rect};
  painter->fillRect(rect, is_publish ? this->pub_bg_color_ : this->sub_bg_color_);

  QPen pen(painter->pen());
  pen.setColor(is_publish ? this->pub_font_color_ : this->sub_font_color_);
  painter->setPen(pen);

  QFont font(painter->font());
  font.setPixelSize(12);
  painter->setFont(font);

  const QString msg = QString("Topic: %1  QoS: %2").arg(topic).arg(static_cast<int>(qos));
  const int kTextFlag = Qt::AlignLeft | Qt::AlignTop;
  QRect bounding_rect;
  painter->drawText(rect, kTextFlag, msg, &bounding_rect);

  const QRect payload_rect{bounding_rect.bottomLeft(),
                           QSize{rect.width(), rect.height() - bounding_rect.height()}};
  painter->drawText(payload_rect, kTextFlag, payload);

  const QString ts = timestamp.toString();
  painter->drawText(rect, ts);
}

QSize MessagesDelegate::sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const {
  return QStyledItemDelegate::sizeHint(option, index);
}

}  // namespace hebo