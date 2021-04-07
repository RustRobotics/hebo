// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/left_panel.h"

#include <QVBoxLayout>

#include "base/file.h"
#include "resources/fonts/fonts.h"
#include "resources/styles/styles.h"
#include "widgets/left_panel_button.h"

namespace hebo {

LeftPanel::LeftPanel(QWidget* parent) : QFrame(parent) {
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

  auto* connections_btn = new LeftPanelButton(kFontIconConnection, tr("Connections"));
  this->btn_group_->addButton(connections_btn, ButtonId::kConnectionsButton);
  main_layout->addWidget(connections_btn);

  auto* new_connection_btn = new LeftPanelButton(kFontIconCirclePlus, tr("New"));
  this->btn_group_->addButton(new_connection_btn, ButtonId::kNewConnectionButton);
  main_layout->addWidget(new_connection_btn);

  auto* benchmark_btn = new LeftPanelButton(kFontIconStopwatch, tr("Benchmark"));
  this->btn_group_->addButton(benchmark_btn, ButtonId::kBenchmarkButton);
  main_layout->addWidget(benchmark_btn);

  auto* bag_btn = new LeftPanelButton(kFontIconBox, tr("Bag"));
  this->btn_group_->addButton(bag_btn, ButtonId::kBagButton);
  main_layout->addWidget(bag_btn);

  auto* log_btn = new LeftPanelButton(kFontIconNotebook, tr("Log"));
  log_btn->setToolTip(tr(""));
  this->btn_group_->addButton(log_btn, ButtonId::kLogButton);
  main_layout->addWidget(log_btn);

  main_layout->addStretch();

  auto* about_btn = new LeftPanelButton(kFontIconWarning, tr("About"));
  this->btn_group_->addButton(about_btn, ButtonId::kAboutButton);
  main_layout->addWidget(about_btn);

  auto* settings_btn = new LeftPanelButton(kFontIconSettings, tr("Settings"));
  this->btn_group_->addButton(settings_btn, ButtonId::kSettingsButton);
  main_layout->addWidget(settings_btn);

  this->setObjectName("left-panel");
  this->setStyleSheet(readTextFile(kStyleLeftPanel));
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