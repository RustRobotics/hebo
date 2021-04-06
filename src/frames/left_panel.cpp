// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/left_panel.h"

#include <QVBoxLayout>

#include "resources/fonts/fonts.h"
#include "widgets/round_font_button.h"

namespace hebo {

LeftPanel::LeftPanel(QWidget* parent) : QWidget(parent) {
  this->initUi();
  this->initSignals();
}

void LeftPanel::initUi() {
  auto* main_layout = new QVBoxLayout();
  main_layout->setContentsMargins(0, 0, 0, 0);
  main_layout->setSpacing(12);
  this->setLayout(main_layout);

  this->btn_group_ = new QButtonGroup(this);
  this->btn_group_->setExclusive(true);

  auto* connections_btn = new RoundFontButton(kFontIconConnection);
  this->btn_group_->addButton(connections_btn, ButtonId::kConnectionsButton);
  main_layout->addWidget(connections_btn);

  auto* new_connection_btn = new RoundFontButton(kFontIconCirclePlus);
  this->btn_group_->addButton(new_connection_btn, ButtonId::kNewConnectionButton);
  main_layout->addWidget(new_connection_btn);

  auto* benchmark_btn = new RoundFontButton(kFontIconStopwatch);
  this->btn_group_->addButton(benchmark_btn, ButtonId::kBenchmarkButton);
  main_layout->addWidget(benchmark_btn);

  auto* bag_btn = new RoundFontButton(kFontIconBox);
  this->btn_group_->addButton(bag_btn, ButtonId::kBagButton);
  main_layout->addWidget(bag_btn);

  auto* log_btn = new RoundFontButton(kFontIconNotebook);
  this->btn_group_->addButton(log_btn, ButtonId::kLogButton);
  main_layout->addWidget(log_btn);

  main_layout->addStretch();

  auto* about_btn = new RoundFontButton(kFontIconWarning);
  this->btn_group_->addButton(about_btn, ButtonId::kAboutButton);
  main_layout->addWidget(about_btn);

  auto* settings_btn = new RoundFontButton(kFontIconSettings);
  this->btn_group_->addButton(settings_btn, ButtonId::kSettingsButton);
  main_layout->addWidget(settings_btn);
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