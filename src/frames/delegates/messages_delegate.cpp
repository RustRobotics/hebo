// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/delegates/messages_delegate.h"

#include <QPainter>
#include <QRegion>
#include <QtMath>
#include <QApplication>

#include "mqtt/message_stream_model.h"

namespace hebo {
namespace {

constexpr int kTextFlag = Qt::AlignLeft | Qt::AlignTop | Qt::TextWrapAnywhere;
constexpr int kTextFlagNoWrap = Qt::AlignLeft | Qt::AlignTop;
constexpr qreal kWidthRatio = 2.0 / 3;

}  // namespace

MessagesDelegate::MessagesDelegate(QObject* parent)
  : QStyledItemDelegate(parent),
    pub_bg_color_("#34c388"),
    sub_bg_color_("#edeef3"),
    pub_font_color_("#fafafa"),
    sub_font_color_("#B2B5C0"),
    font_(QApplication::font()) {
  this->font_.setPixelSize(12);
}

void MessagesDelegate::paint(QPainter* painter, const QStyleOptionViewItem& option, const QModelIndex& index) const {
  const QString topic = index.data(MessageStreamModel::kTopicRole).toString();
  const QoS qos = index.data(MessageStreamModel::kQoSRole).value<QoS>();
  const bool is_publish = index.data(MessageStreamModel::kIsPublishRole).toBool();
  const auto timestamp = index.data(MessageStreamModel::kTimestampRole).value<QDateTime>();
  const QByteArray payload_bytes = index.data(MessageStreamModel::kPayloadRole).toByteArray();

  const QRect rect{option.rect};
  painter->fillRect(rect, is_publish ? this->pub_bg_color_ : this->sub_bg_color_);

  QPen pen(painter->pen());
  pen.setColor(is_publish ? this->pub_font_color_ : this->sub_font_color_);
  painter->setPen(pen);
  painter->setFont(this->font_);

  const QString msg = QString("Topic: %1  QoS: %2").arg(topic).arg(static_cast<int>(qos));
  QRect bounding_rect;
  painter->drawText(rect, kTextFlag, msg, &bounding_rect);

  const QRect payload_rect{bounding_rect.left(), bounding_rect.bottom() + 8,
                           rect.width(), rect.bottom() - bounding_rect.bottom()};
  const QString payload = QString::fromUtf8(payload_bytes);
  painter->drawText(payload_rect, kTextFlag, payload, &bounding_rect);

  const QString ts = timestamp.toString();
  const QRect ts_rect{bounding_rect.x(), bounding_rect.bottom() + 8,
                      rect.width(), rect.bottom() - bounding_rect.bottom()};
  painter->drawText(ts_rect, kTextFlagNoWrap, ts);
}

QSize MessagesDelegate::sizeHint(const QStyleOptionViewItem& option, const QModelIndex& index) const {
  QSize size = QStyledItemDelegate::sizeHint(option, index);
  const QString topic = index.data(MessageStreamModel::kTopicRole).toString();
  const QString msg = QString("Topic: %1  QoS: 0").arg(topic);
  const QByteArray payload_bytes = index.data(MessageStreamModel::kPayloadRole).toByteArray();
  const QString payload = QString::fromUtf8(payload_bytes);

  QFontMetrics metrics(this->font_);
  QRect rect{0, 0, size.width(), size.height()};
  const QRect topic_rect = metrics.boundingRect(rect, kTextFlag, msg);
  qDebug() << "topic rect:" << topic_rect;
  const QRect payload_rect = metrics.boundingRect(rect, kTextFlag, payload);
  qDebug() << "payload rect:" << payload_rect;
  const QRect ts_rect = metrics.boundingRect(rect, kTextFlagNoWrap, QChar('1'));
  qDebug() << "ts_rect:" << ts_rect;
  size.setHeight(topic_rect.height() + payload_rect.height() + ts_rect.height());
  return size;
}

}  // namespace hebo