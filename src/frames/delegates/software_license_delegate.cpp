// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/delegates/software_license_delegate.h"

#include <QDebug>
#include <QFont>
#include <QPainter>
#include <QPen>

#include "frames/models/software_license_model.h"

namespace hebo {

SoftwareLicenseDelegate::SoftwareLicenseDelegate(QObject* parent) : QStyledItemDelegate(parent) {

}

void SoftwareLicenseDelegate::paint(QPainter* painter,
                                    const QStyleOptionViewItem& option,
                                    const QModelIndex& index) const {
  QFont font(painter->font());
  QPen pen(painter->pen());
  constexpr int kTextFlags = Qt::AlignLeft | Qt::AlignVCenter;
  const bool is_hover = (option.state & QStyle::State_MouseOver) != 0;

  if (index.column() == SoftwareLicenseModel::kSoftwareColumn) {
    const QString name = index.data(SoftwareLicenseModel::kNameRole).toString();
    const QString url = index.data(SoftwareLicenseModel::kUrlRole).toString();
    if (!url.isEmpty()) {
      pen.setColor(Qt::blue);
    } else {
      pen.setColor(Qt::black);
    }
    painter->setPen(pen);
    if (is_hover) {
      font.setUnderline(true);
      painter->setFont(font);
    }
    QRect bounding_rect{};
    painter->drawText(option.rect, kTextFlags, name, &bounding_rect);

    const QString version = index.data(SoftwareLicenseModel::kVersionRole).toString();
    constexpr const int kLeftMargin = 8;
    pen.setColor(Qt::black);
    painter->setPen(pen);
    font.setUnderline(false);
    painter->setFont(font);
    painter->drawText(bounding_rect.x() + bounding_rect.width() + kLeftMargin, option.rect.y(),
                      option.rect.width() - bounding_rect.width(), option.rect.height(),
                      kTextFlags, version);
  } else if (index.column() == SoftwareLicenseModel::kLicenseColumn) {
    const QString license = index.data(SoftwareLicenseModel::kLicenseRole).toString();
    const QString license_url = index.data(SoftwareLicenseModel::kLicenseUrlRole).toString();
    if (!license_url.isEmpty()) {
      pen.setColor(Qt::blue);
    } else {
      pen.setColor(Qt::black);
    }
    painter->setPen(pen);
    if (is_hover) {
      font.setUnderline(true);
      painter->setFont(font);
    }
    painter->drawText(option.rect, kTextFlags, license);
  } else {
    qWarning() << "Invalid column index:" << index;
    return;
  }
}

}  // namespace hebo