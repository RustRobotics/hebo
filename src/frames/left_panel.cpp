// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/left_panel.h"

#include <QVBoxLayout>

#include "widgets/round_font_button.h"

namespace hebo {

LeftPanel::LeftPanel(QWidget* parent) : QWidget(parent) {
  this->initUi();
  this->initSignals();
}

void LeftPanel::initUi() {
  auto* main_layout = new QVBoxLayout();
  main_layout->setContentsMargins(0, 0, 0, 0);
  main_layout->setSpacing(0);
  this->setLayout(main_layout);

  this->btn_group_ = new QButtonGroup(this);
  this->btn_group_->setExclusive(true);

  main_layout->addStretch();

  auto* about_btn = new RoundFontButton(tr("About"));
  this->btn_group_->addButton(about_btn, ButtonId::kAbout);
  main_layout->addWidget(about_btn);

  auto* settings_btn = new RoundFontButton(tr("Settings"));
  this->btn_group_->addButton(settings_btn);
  main_layout->addWidget(settings_btn, ButtonId::kSettings);
}

void LeftPanel::initSignals() {
  connect(this->btn_group_, &QButtonGroup::idClicked, [=](int id) {
    emit this->activeChanged(static_cast<ButtonId>(id));
  });
}

LeftPanel::ButtonId LeftPanel::activeButton() const {
  return static_cast<ButtonId>(this->btn_group_->checkedId());
}

void LeftPanel::setActiveButton(LeftPanel::ButtonId id) {
  this->btn_group_->button(id)->click();
}

}  // namespace hebo