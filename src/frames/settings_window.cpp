// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/settings_window.h"

#include <QGridLayout>
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
  this->setLayout(main_layout);
  main_layout->addSpacing(32);

  auto* grid_layout = new QGridLayout();
  grid_layout->setHorizontalSpacing(16);
  grid_layout->setVerticalSpacing(24);
  main_layout->addLayout(grid_layout);
  main_layout->addStretch();

  grid_layout->addWidget(new QLabel(tr("Language")), 0, 0, Qt::AlignRight);
  this->locale_box_ = new QComboBox();
  this->locales_ << "en_US" << "zh_CN";
  this->locale_box_->addItems({"English", "简体中文"});
  grid_layout->addWidget(this->locale_box_, 0, 1, Qt::AlignLeft);

  grid_layout->addWidget(new QLabel(tr("Auto check update")), 1, 0, Qt::AlignRight);
  this->auto_update_button_ = new SwitchButton();
  grid_layout->addWidget(this->auto_update_button_, 1, 1, Qt::AlignLeft);

  grid_layout->addWidget(new QLabel(tr("Max retry Connections")), 2, 0, Qt::AlignRight);
  this->retry_connection_box_ = new QSpinBox();
  this->retry_connection_box_->setRange(0, kRetryConnectionsMax);
  grid_layout->addWidget(this->retry_connection_box_, 2, 1, Qt::AlignLeft);

  grid_layout->addWidget(new QLabel(tr("Theme")), 3, 0, Qt::AlignRight);
  this->theme_box_ = new QComboBox();
  this->themes_ << "light" << "dark" << "night";
  this->theme_box_->addItems({
    tr("Light"),
    tr("Dark"),
    tr("Night")
  });
  grid_layout->addWidget(this->theme_box_, 3, 1, Qt::AlignLeft);
}

void SettingsWindow::initSignals() {
  connect(this->locale_box_, &QComboBox::currentTextChanged,
          this, &SettingsWindow::localeChanged);
  connect(this->auto_update_button_, &SwitchButton::toggled,
          this, &SettingsWindow::autoUpdateChanged);
  connect(this->retry_connection_box_, QOverload<int>::of(&QSpinBox::valueChanged),
          this, &SettingsWindow::retryConnectionChanged);
  connect(this->theme_box_, &QComboBox::currentTextChanged,
          this, &SettingsWindow::themeChanged);
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

void SettingsWindow::setTheme(const QString& theme) {
  QSignalBlocker blocker(this->theme_box_);
  this->theme_box_->setCurrentText(theme);
}

}  // namespace hebo