// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/settings_window.h"

#include <QGridLayout>
#include <QLabel>

namespace hebo {

SettingsWindow::SettingsWindow(QWidget* parent) : QFrame(parent) {
  this->initUi();
  this->initSignals();
}

void SettingsWindow::initUi() {
  auto* grid_layout = new QGridLayout(this);
  this->setLayout(grid_layout);

  grid_layout->addWidget(new QLabel(tr("Language")), 0, 0);
  this->languages_box_ = new QComboBox();
  this->locale_names_ << "English" << "简体中文";
  this->locales_ << "en_US" << "zh_CN";
  this->languages_box_->addItems(this->locale_names_);
  grid_layout->addWidget(this->languages_box_, 0, 1);

  this->theme_names_ << tr("Light") << tr("Dark") << tr("Night");
  this->themes_ << "light" << "dark" << "night";
}

void SettingsWindow::initSignals() {

}

}  // namespace hebo