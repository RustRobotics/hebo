// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_CLIENT_FRAME_H_
#define HEBO_SRC_FRAMES_CLIENT_FRAME_H_

#include <QFrame>
#include <QLineEdit>
#include <QListView>
#include <QPlainTextEdit>
#include <QTextEdit>

namespace hebo {

class ClientFrame : public QFrame {
  Q_OBJECT
 public:
  explicit ClientFrame(const QString& client_id, QWidget* parent = nullptr);

  [[nodiscard]] const QString& clientId() const { return this->client_id_; }

 private:
  void initUi();
  void initSignals();

  QString client_id_;
  QFrame* tool_bar_{nullptr};
  QListView* subscriptions_list_view_{nullptr};
  QPlainTextEdit* messages_edit_{nullptr};
  QLineEdit* topic_edit_{nullptr};
  QTextEdit* payload_edit_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_CLIENT_FRAME_H_
