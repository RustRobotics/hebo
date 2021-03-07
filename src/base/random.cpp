// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "base/random.h"

#include <QDateTime>
#include <QDebug>
#include <QRandomGenerator>
#include <QUuid>

namespace hebo {

QString randomClientId() {
  auto* rng = QRandomGenerator::global();
  const qint64 time =  QDateTime::currentMSecsSinceEpoch();
  const quint64 num = rng->generate64() + time;
  return QString::number(num, 16).right(8);
}

QString generateConfigId() {
  QUuid uuid = QUuid::createUuid();
  return uuid.toString();
}

}  // namespace hebo