// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FORMATS_PAYLOAD_TYPE_H_
#define HEBO_SRC_FORMATS_PAYLOAD_TYPE_H_

#include <QDebug>
#include <QString>

namespace hebo {

enum class PayloadType : uint8_t {
  kPlainText = 0,
  kBase64,
  kJson,
  kHex,
  kPayloadTypeMax,
};

}  // namespace hebo

Q_DECLARE_METATYPE(hebo::PayloadType);

#endif  // HEBO_SRC_FORMATS_PAYLOAD_TYPE_H_
