// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/settings_window.h"

#include <QFormLayout>
#include <QLabel>
#include <QVBoxLayout>

namespace hebo {
namespace {

constexpr const int kRetryConnectionsMax = 1 << 10;

}  // namespace

SettingsWindow::SettingsWindow(QWidget* parent) : QFrame(parent) {
  this->initUi();
  this->initSignals();
}

void SettingsWindow::initUi() {
  this->setWindowTitle(tr("Settings"));
  auto* main_layout = new QVBoxLayout();
  main_layout->setContentsMargins(0, 0, 108, 0);
  this->setLayout(main_layout);
  main_layout->addSpacing(32);

  auto* form_layout = new QFormLayout();
  form_layout->setHorizontalSpacing(24);
  form_layout->setVerticalSpacing(12);
  form_layout->setFormAlignment(Qt::AlignHCenter | Qt::AlignTop);
  form_layout->setLabelAlignment(Qt::AlignRight);
  form_layout->setRowWrapPolicy(QFormLayout::DontWrapRows);
  form_layout->setFieldGrowthPolicy(QFormLayout::FieldsStayAtSizeHint);
  main_layout->addLayout(form_layout);
  main_layout->addStretch();

  this->locale_box_ = new QComboBox();
  this->locales_ << "en_US" << "zh_CN";
  this->locale_box_->addItems({"English", "简体中文"});
  form_layout->addRow(new QLabel(tr("Language")), this->locale_box_);

  this->auto_update_button_ = new SwitchButton();
  form_layout->addRow(new QLabel(tr("Auto check update")), this->auto_update_button_);

  this->retry_connection_box_ = new SpinBox();
  this->retry_connection_box_->setRange(0, kRetryConnectionsMax);
  form_layout->addRow(new QLabel(tr("Max retry Connections")), this->retry_connection_box_);

  this->theme_box_ = new QComboBox();
  this->theme_box_->addItems({
    tr("Day"),
    tr("Night")
  });
  form_layout->addRow(new QLabel(tr("Theme")), this->theme_box_);
}

void SettingsWindow::initSignals() {
  connect(this->locale_box_, &QComboBox::currentTextChanged,
          this, &SettingsWindow::localeChanged);
  connect(this->auto_update_button_, &SwitchButton::toggled,
          this, &SettingsWindow::autoUpdateChanged);
  connect(this->retry_connection_box_, QOverload<int>::of(&SpinBox::valueChanged),
          this, &SettingsWindow::retryConnectionChanged);
  connect(this->theme_box_, QOverload<int>::of(&QComboBox::currentIndexChanged),
          [=](int index) {
    qDebug() << "fuck" << index;
    emit this->themeChanged(static_cast<ThemeType>(index));
  });
}

void SettingsWindow::setLocale(const QString& locale) {
  QSignalBlocker blocker(this->locale_box_);
  this->locale_box_->setCurrentText(locale);
}

void SettingsWindow::setAutoUpdate(bool auto_update) {
  QSignalBlocker blocker(this->auto_update_button_);
  this->auto_update_button_->setChecked(auto_update);
}

void SettingsWindow::setRetryConnection(int retry) {
  QSignalBlocker blocker(this->retry_connection_box_);
  this->retry_connection_box_->setValue(retry);
}

void SettingsWindow::setTheme(ThemeType theme) {
  QSignalBlocker blocker(this->theme_box_);
  this->theme_box_->setCurrentIndex(static_cast<int>(theme));
}

}  // namespace hebo