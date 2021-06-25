// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/internal/new_subscription_window.h"

#include <QFormLayout>
#include <QLabel>
#include <rusty/gui/color.h>

#include "resources/fonts/fonts.h"
#include "resources/misc/misc.h"

namespace hebo {

NewSubscriptionWindow::NewSubscriptionWindow(QWidget* parent) : QDialog(parent) {
  this->initUi();
  this->initSignals();
  this->setModal(true);
}

void NewSubscriptionWindow::initUi() {
  this->setWindowTitle(tr("New Subscription"));

  auto* main_layout = new QVBoxLayout();
  this->setLayout(main_layout);

  auto* form_layout = new QFormLayout();
  main_layout->addLayout(form_layout);

  this->topic_edit_ = new QLineEdit();
  form_layout->addRow(new QLabel(tr("Topic")), this->topic_edit_);

  this->qos_box_ = new QComboBox();
  this->qos_model_ = new QoSModel(this);
  this->qos_box_->setModel(this->qos_model_);
  form_layout->addRow(new QLabel(tr("QoS")), this->qos_box_);

  bool ok = true;
  const auto palette = rusty::parseColorPalette(kMiscGtkPalette, &ok);
  Q_ASSERT(ok);
  auto* color_layout = new QHBoxLayout();
  this->color_chooser_button_ = new rusty::ColorChooserButton();
  this->color_chooser_button_->setColorPalette(palette);

  color_layout->addWidget(this->color_chooser_button_);
  this->refresh_color_button_ = new FontIconButton(kFontElIconRefresh);
  this->refresh_color_button_->setFixedSize(24, 24);
  color_layout->addSpacing(12);
  color_layout->addWidget(this->refresh_color_button_);
  form_layout->addRow(new QLabel(tr("Color")), color_layout);

  this->alias_edit_ = new QLineEdit();
  form_layout->addRow(new QLabel(tr("Alias")), this->alias_edit_);

  this->payload_type_box_ = new QComboBox();
  this->payload_type_model_ = new PayloadTypeModel(this);
  this->payload_type_box_->setModel(this->payload_type_model_);
  form_layout->addRow(new QLabel(tr("Payload:")), this->payload_type_box_);

  auto* buttons_layout = new QHBoxLayout();
  main_layout->addSpacing(12);
  main_layout->addLayout(buttons_layout);
  this->cancel_button_ = new QPushButton(tr("Cancel"));
  this->ok_button_ = new QPushButton(tr("Ok"));
  buttons_layout->addWidget(this->cancel_button_);
  buttons_layout->addWidget(this->ok_button_);
}

void NewSubscriptionWindow::initSignals() {
  connect(this->cancel_button_, &QPushButton::clicked,
          this, &NewSubscriptionWindow::hide);
  connect(this->ok_button_, &QPushButton::clicked,
          this, &NewSubscriptionWindow::accept);
  connect(this->refresh_color_button_, &FontIconButton::clicked,
          this, &NewSubscriptionWindow::generateRandomColor);
}

void NewSubscriptionWindow::generateRandomColor() {
  const auto color = rusty::randomColor();
  this->color_chooser_button_->setColor(color);
}

void NewSubscriptionWindow::resetForm() {
  this->topic_edit_->clear();
  this->qos_box_->setCurrentIndex(0);
  this->generateRandomColor();
  this->alias_edit_->clear();
}

PayloadType NewSubscriptionWindow::payloadType() const {
  const auto index = this->payload_type_model_->index(this->payload_type_box_->currentIndex(), 0);
  return this->payload_type_model_->data(index, PayloadTypeModel::kIdRole).value<PayloadType>();
}

}  // namespace hebo