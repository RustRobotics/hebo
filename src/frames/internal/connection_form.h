// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_INTERNAL_CONNECTION_FORM_H_
#define HEBO_SRC_FRAMES_INTERNAL_CONNECTION_FORM_H_

#include <QComboBox>
#include <QFrame>
#include <QLineEdit>
#include <QSpinBox>
#include <QTextEdit>
#include <QVBoxLayout>
#include <rusty/widgets/switch_button.h>

#include "frames/models/protocol_model.h"
#include "frames/models/qos_model.h"
#include "frames/models/version_model.h"

namespace hebo {

class ConnectionForm : public QFrame {
  Q_OBJECT
 public:
  explicit ConnectionForm(QWidget* parent = nullptr);

 public slots:
  void regenerateClientId();

 signals:
  void connectRequested(const ConnectConfig& config);

 private slots:
  void onResetButtonClicked();
  void onConnectButtonClicked();

 private:
  void initUi();
  void initSignals();

  void initGeneralForm(QVBoxLayout* main_layout);
  void initAdvancedForm(QVBoxLayout* main_layout);
  void initLastWillForm(QVBoxLayout* main_layout);

  QLineEdit* name_edit_{nullptr};
  QLineEdit* client_id_edit_{nullptr};
  QPushButton* random_client_id_button_{nullptr};
  QComboBox* protocol_box_{nullptr};
  ProtocolModel* protocol_model_{nullptr};
  QLineEdit* hostname_edit_{nullptr};
  QSpinBox* port_box_{nullptr};
  QLineEdit* username_edit_{nullptr};
  QLineEdit* password_edit_{nullptr};
  rusty::SwitchButton* tls_switch_{nullptr};

  QSpinBox* timeout_box_{nullptr};
  QSpinBox* keepalive_box_{nullptr};
  rusty::SwitchButton* clean_session_btn_{nullptr};
  rusty::SwitchButton* auto_reconnect_btn_{nullptr};
  QComboBox* mqtt_version_box_{nullptr};
  VersionModel* mqtt_version_model_{nullptr};

  QLineEdit* last_will_topic_edit_{nullptr};
  QComboBox* last_will_qos_box_{nullptr};
  QoSModel* qos_model_{nullptr};
  rusty::SwitchButton* last_will_retain_button_{nullptr};
  QTextEdit* last_will_payload_edit_{nullptr};

  QPushButton* reset_button_{nullptr};
  QPushButton* connect_button_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_INTERNAL_CONNECTION_FORM_H_
