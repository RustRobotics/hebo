// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_INTERNAL_MESSAGES_DOCUMENT_H_
#define HEBO_SRC_FRAMES_INTERNAL_MESSAGES_DOCUMENT_H_

#include <QColor>
#include <QTextDocument>

#include "mqtt/message_stream_model.h"

namespace hebo {

class MessagesDocument : public QTextDocument {
  Q_OBJECT
 public:
  explicit MessagesDocument(MessageStreamModel* model, QObject* parent = nullptr);

 signals:
  void messageAdded();

 private slots:
  void onRowsInserted(const QModelIndex& index, int first, int last);

 private:
  void initSignals();

  MessageStreamModel* model_;
  QColor pub_bg_color_;
  QColor sub_bg_color_;
  QColor pub_font_color_;
  QColor sub_font_color_;
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_INTERNAL_MESSAGES_DOCUMENT_H_
