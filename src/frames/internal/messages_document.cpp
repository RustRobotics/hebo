// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/internal/messages_document.h"

#include <QDebug>
#include <QTextBlockFormat>
#include <QTextCharFormat>
#include <QTextCursor>

namespace hebo {

MessagesDocument::MessagesDocument(MessageStreamModel* model, QObject* parent)
  : QTextDocument(parent),
    model_(model),
    pub_bg_color_("#34c388"),
    sub_bg_color_("#edeef3"),
    pub_font_color_("#fafafa"),
    sub_font_color_("#B2B5C0") {
  this->initSignals();
}

void MessagesDocument::initSignals() {
  connect(this->model_, &MessageStreamModel::rowsInserted,
          this, &MessagesDocument::onRowsInserted);
}

void MessagesDocument::onRowsInserted(const QModelIndex& index, int first, int last) {
  Q_UNUSED(index);
  MqttMessage msg;
  for (int row = first; row <= last; ++row) {
    if (!this->model_->row(row, msg)) {
      break;
    }
    QTextCursor cursor(this);
    cursor.movePosition(QTextCursor::End);
    auto block_fmt = cursor.blockFormat();
    if (msg.is_publish) {
      block_fmt.setAlignment(Qt::AlignLeft);
      block_fmt.setBackground(this->pub_bg_color_);
      block_fmt.setLeftMargin(100);
    } else {
      block_fmt.setAlignment(Qt::AlignLeft);
      block_fmt.setBackground(this->sub_bg_color_);
      block_fmt.setLeftMargin(0);
    }
    QTextCharFormat char_fmt(cursor.charFormat());

    const QString header = QString("Topic: %1  QoS: %2\n")
        .arg(msg.topic)
        .arg(static_cast<int>(msg.qos));
    cursor.setBlockFormat(block_fmt);
    char_fmt.setFontPointSize(12);
    cursor.setCharFormat(char_fmt);
    cursor.insertText(header);

    cursor.insertText(QString::fromUtf8(msg.payload));

    const QString ts = msg.timestamp.toString();
    char_fmt.setFontPointSize(9);
    cursor.setCharFormat(char_fmt);
    cursor.insertText("\n");
    cursor.insertText(ts);

    block_fmt.setBackground(QBrush(Qt::white));
    cursor.setBlockFormat(block_fmt);
    cursor.insertText("\n\n");
  }
}

}  // namespace hebo