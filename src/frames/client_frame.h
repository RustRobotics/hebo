// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_CLIENT_FRAME_H_
#define HEBO_SRC_FRAMES_CLIENT_FRAME_H_

#include <QFrame>
#include <QLabel>
#include <QLineEdit>
#include <QListView>
#include <QPlainTextEdit>
#include <QTextEdit>

#include "frames/internal/new_subscription_window.h"
#include "mqtt/mqtt_client.h"
#include "widgets/font_icon_button.h"

namespace hebo {

class ClientFrame : public QFrame {
  Q_OBJECT
 public:
  ClientFrame(const QString& client_id, MqttClient* client, QWidget* parent = nullptr);

  [[nodiscard]] const QString& clientId() const { return this->client_id_; }

 protected:
  void resizeEvent(QResizeEvent* event) override;

 private slots:
  void onClientStateChanged(ConnectionState state);

  void onPublishButtonClicked();

  void onSubscribeButtonClicked();

  void onNewSubscriptionWindowConfirmed();

 private:
  void initUi();
  void initSignals();

  QString client_id_;
  MqttClient* client_;

  QFrame* tool_bar_{nullptr};
  QLabel* title_label_{nullptr};
  FontIconButton* connect_button_{nullptr};
  FontIconButton* disconnect_button_{nullptr};
  FontIconButton* edit_button_{nullptr};
  FontIconButton* options_button_{nullptr};
  FontIconButton* publish_button_{nullptr};

  NewSubscriptionWindow* new_subscription_window_{nullptr};
  QPushButton* subscribe_button_{nullptr};
  QListView* subscriptions_list_view_{nullptr};

  QTextEdit* messages_edit_{nullptr};
  QLineEdit* topic_edit_{nullptr};
  QPlainTextEdit* payload_edit_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_CLIENT_FRAME_H_
