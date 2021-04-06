// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_INTERNAL_CONNECTION_FORM_H_
#define HEBO_SRC_FRAMES_INTERNAL_CONNECTION_FORM_H_

#include <QComboBox>
#include <QFrame>
#include <QLineEdit>
#include <QSpinBox>
#include <QVBoxLayout>

#include "frames/models/protocol_model.h"
#include "frames/models/version_model.h"
#include "widgets/switch_button.h"

namespace hebo {

class ConnectionForm : public QFrame {
  Q_OBJECT
 public:
  explicit ConnectionForm(QWidget* parent = nullptr);

 private:
  void initUi();
  void initGeneralForm(QVBoxLayout* main_layout);
  void initAdvancedForm(QVBoxLayout* main_layout);

  QLineEdit* name_edit_{nullptr};
  QLineEdit* client_id_edit_{nullptr};
  QComboBox* protocol_box_{nullptr};
  ProtocolModel* protocol_model_{nullptr};
  QLineEdit* hostname_edit_{nullptr};
  QSpinBox* port_box_{nullptr};
  QLineEdit* username_edit_{nullptr};
  QLineEdit* password_edit_{nullptr};
  SwitchButton* tls_switch_{nullptr};

  QSpinBox* timeout_box_{nullptr};
  QSpinBox* keepalive_box_{nullptr};
  SwitchButton* clean_session_btn_{nullptr};
  SwitchButton* auto_reconnect_btn_{nullptr};
  QComboBox* mqtt_version_box_{nullptr};
  VersionModel* mqtt_version_model_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_INTERNAL_CONNECTION_FORM_H_
