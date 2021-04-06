// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/internal/connection_form.h"

#include <QFormLayout>
#include <QLabel>

#include "base/random.h"

namespace hebo {
namespace {

constexpr const char* kDefaultHostname = "localhost";
constexpr int kDefaultPort = 1883;
constexpr int kMaxPort = 65535;
constexpr int kDefaultConnectTimeout = 10;
constexpr int kMaxConnectTimeout = 1 << 20;
constexpr int kDefaultKeepalive = 60;
constexpr int kMaxKeepalive = 1 << 20;

}  // namespace

ConnectionForm::ConnectionForm(QWidget* parent) : QFrame(parent) {
  this->initUi();
  this->initSignals();
  this->regenerateClientId();
}

void ConnectionForm::initUi() {
  auto* main_layout = new QVBoxLayout();
  this->setLayout(main_layout);

  this->initGeneralForm(main_layout);
  this->initAdvancedForm(main_layout);
  this->initLastWillForm(main_layout);

  auto* button_layout = new QHBoxLayout();
  this->reset_button_ = new QPushButton(tr("Reset"));
  button_layout->addWidget(this->reset_button_);
  this->connect_button_ = new QPushButton(tr("Connect"));
  button_layout->addWidget(this->connect_button_);
  button_layout->addStretch();
  main_layout->addLayout(button_layout);
}

void ConnectionForm::initGeneralForm(QVBoxLayout* main_layout) {
  auto* title_label = new QLabel(tr("General"));
  main_layout->addWidget(title_label);

  auto* layout = new QFormLayout();
  main_layout->addLayout(layout);

  this->name_edit_ = new QLineEdit();
  layout->addRow(new QLabel(tr("Name")), this->name_edit_);

  auto* client_id_layout = new QHBoxLayout();
  this->client_id_edit_ = new QLineEdit();
  this->random_client_id_button_ = new QPushButton("Refresh");
  client_id_layout->addWidget(this->client_id_edit_);
  client_id_layout->addWidget(this->random_client_id_button_);
  layout->addRow(new QLabel(tr("Client ID")), client_id_layout);

  this->protocol_box_ = new QComboBox();
  this->protocol_model_ = new ProtocolModel(this);
  this->protocol_box_->setModel(this->protocol_model_);
  this->hostname_edit_ = new QLineEdit();
  this->hostname_edit_->setText(kDefaultHostname);
  auto* host_layout = new QHBoxLayout();
  host_layout->addWidget(this->protocol_box_);
  host_layout->addWidget(this->hostname_edit_);
  layout->addRow(new QLabel("Host"), host_layout);

  this->port_box_ = new QSpinBox();
  this->port_box_->setRange(1, kMaxPort);
  this->port_box_->setValue(kDefaultPort);
  layout->addRow(new QLabel("Port"), this->port_box_);

  this->username_edit_ = new QLineEdit();
  layout->addRow(new QLabel("Username"), this->username_edit_);

  this->password_edit_ = new QLineEdit();
  layout->addRow(new QLabel("Password"), this->password_edit_);

  this->tls_switch_ = new SwitchButton();
  layout->addRow(new QLabel("SSL/TLS"), this->tls_switch_);
}

void ConnectionForm::initAdvancedForm(QVBoxLayout* main_layout) {
  auto* title_label = new QLabel(tr("Advanced"));
  main_layout->addWidget(title_label);

  auto* layout = new QFormLayout();
  main_layout->addLayout(layout);

  this->timeout_box_ = new QSpinBox();
  this->timeout_box_->setRange(0, kMaxConnectTimeout);
  this->timeout_box_->setValue(kDefaultConnectTimeout);
  layout->addRow(new QLabel(tr("Connect Timeout(s)")), this->timeout_box_);

  this->keepalive_box_ = new QSpinBox();
  this->keepalive_box_->setRange(0, kMaxKeepalive);
  this->keepalive_box_->setValue(kDefaultKeepalive);
  layout->addRow(new QLabel(tr("Keep Alive(s)")), this->keepalive_box_);

  this->clean_session_btn_ = new SwitchButton();
  this->clean_session_btn_->setChecked(true);
  layout->addRow(new QLabel(tr("Clean Session")), this->clean_session_btn_);

  this->auto_reconnect_btn_ = new SwitchButton();
  layout->addRow(new QLabel(tr("Auto Reconnect")), this->auto_reconnect_btn_);

  this->mqtt_version_box_ = new QComboBox();
  this->mqtt_version_model_ = new VersionModel();
  this->mqtt_version_box_->setModel(this->mqtt_version_model_);
  layout->addRow(new QLabel("MQTT Version"), this->mqtt_version_box_);
}

void ConnectionForm::initLastWillForm(QVBoxLayout* main_layout) {
  auto* title_label = new QLabel(tr("Last Will"));
  main_layout->addWidget(title_label);

  auto* layout = new QFormLayout();
  main_layout->addLayout(layout);

  this->last_will_topic_edit_ = new QLineEdit();
  layout->addRow(new QLabel(tr("Last-Will Topic")), this->last_will_topic_edit_);

  this->last_will_qos_box_ = new QComboBox();
  this->qos_model_ = new QoSModel();
  this->last_will_qos_box_->setModel(this->qos_model_);
  layout->addRow(new QLabel(tr("Last-Will QoS")), this->last_will_qos_box_);

  this->last_will_retain_button_ = new SwitchButton();
  layout->addRow(new QLabel(tr("Last-Will Retain")), this->last_will_retain_button_);

  this->last_will_payload_edit_ = new QTextEdit();
  layout->addRow(new QLabel(tr("Last-Will Payload")), this->last_will_payload_edit_);
}

void ConnectionForm::initSignals() {
  connect(this->reset_button_, &QPushButton::clicked,
          this, &ConnectionForm::onResetButtonClicked);
  connect(this->connect_button_, &QPushButton::clicked,
          this, &ConnectionForm::onConnectButtonClicked);
  connect(this->random_client_id_button_, &QPushButton::clicked,
          this, &ConnectionForm::regenerateClientId);
}

void ConnectionForm::onResetButtonClicked() {
  this->name_edit_->clear();
  this->regenerateClientId();
  this->protocol_box_->setCurrentIndex(0);
  this->hostname_edit_->setText(kDefaultHostname);
  this->port_box_->setValue(kDefaultPort);
  this->username_edit_->clear();
  this->password_edit_->clear();
  this->tls_switch_->setChecked(false);

  this->timeout_box_->setValue(kDefaultConnectTimeout);
  this->keepalive_box_->setValue(kDefaultKeepalive);
  this->clean_session_btn_->setChecked(true);
  this->auto_reconnect_btn_->setChecked(false);
  this->mqtt_version_box_->setCurrentIndex(0);

  this->last_will_topic_edit_->clear();
  this->last_will_qos_box_->setCurrentIndex(0);
  this->last_will_retain_button_->setChecked(false);
  this->last_will_payload_edit_->clear();
}

void ConnectionForm::onConnectButtonClicked() {
  ConnectConfig config{};
  config.name = this->name_edit_->text();
  config.client_id = this->client_id_edit_->text();
  // TODOS(Shaohua): Replace with protocol enum.
  config.protocol = this->protocol_box_->currentText();
  config.host = this->hostname_edit_->text();
  config.port = this->port_box_->value();
  config.username = this->username_edit_->text();
  config.password = this->password_edit_->text();
  config.with_tls = this->tls_switch_->isChecked();

  config.timeout = this->timeout_box_->value();
  config.keep_alive = this->keepalive_box_->value();
  config.clean_session = this->clean_session_btn_->isChecked();
  config.auto_reconnect = this->auto_reconnect_btn_->isChecked();
  // TODO(Shaohua): Add version enum.

  config.last_will_topic = this->last_will_topic_edit_->text();
  const auto qos_index = this->qos_model_->index(this->last_will_qos_box_->currentIndex());
  config.last_will_qos = this->qos_model_->data(qos_index, QoSModel::kIdRole).value<QoS>();
  config.last_will_retain = this->last_will_retain_button_->isChecked();
  config.last_will_payload = this->last_will_payload_edit_->toPlainText().toUtf8();

  config.id = generateConfigId();
  config.description = generateConnDescription(config);
  emit this->connectRequested(config);
}

void ConnectionForm::regenerateClientId() {
  const QString client_id = "hebo_" + randomClientId();
  this->client_id_edit_->setText(client_id);
}

}  // namespace hebo