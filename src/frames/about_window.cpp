// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/about_window.h"

#include <QDebug>
#include <QDesktopServices>
#include <QLabel>
#include <QUrl>
#include <QVBoxLayout>

#include "base/file.h"
#include "resources/images/images.h"
#include "resources/styles/styles.h"

namespace hebo {
namespace {

constexpr const char* kReleasesUrl = "https://github.com/xushaohua/hebo-ui/releases";
constexpr const char* kIssueUrl = "https://github.com/xushaohua/hebo-ui/issue";

}  // namespace

AboutWindow::AboutWindow(QWidget* parent) : QFrame(parent) {
  this->initUi();
  this->initSignals();
}

void AboutWindow::initUi() {
  this->setWindowTitle(tr("About"));
  this->setObjectName("about-window");
  this->setStyleSheet(readTextFile(kStyleAboutWindow));
  auto* main_layout = new QVBoxLayout();
  main_layout->setSpacing(0);
  main_layout->setContentsMargins(0, 0, 0, 0);
  this->setLayout(main_layout);

  main_layout->addSpacing(32);
  auto* logo_label = new QLabel();
  logo_label->setObjectName("logo-label");
  logo_label->resize(128, 128);
  const QPixmap logo_pixmap(kImageHeboX128);
  logo_label->setPixmap(logo_pixmap);
  main_layout->addWidget(logo_label, 0, Qt::AlignHCenter);
  main_layout->addSpacing(16);

  auto* version_label = new QLabel("v0.1.2");
  version_label->setObjectName("version-label");
  version_label->setAlignment(Qt::AlignHCenter);
  main_layout->addWidget(version_label, 0, Qt::AlignHCenter);

  auto* update_layout = new QHBoxLayout();
  update_layout->setSpacing(24);
  main_layout->addSpacing(10);
  main_layout->addLayout(update_layout);

  this->update_button_ = new TextButton(tr("Check for Update"));
  this->update_button_->setObjectName("update-button");
  update_layout->addStretch();
  update_layout->addWidget(update_button_);

  this->releases_button_ = new TextButton(tr("Releases"));
  this->releases_button_->setObjectName("releases-button");
  update_layout->addWidget(this->releases_button_);

  this->support_button_ = new TextButton(tr("Support"));
  this->support_button_->setObjectName("support-button");
  update_layout->addWidget(this->support_button_);
  update_layout->addStretch();

  main_layout->addStretch();
}

void AboutWindow::initSignals() {
  connect(this->update_button_, &TextButton::clicked,
          this, &AboutWindow::requestUpdate);
  connect(this->releases_button_, &TextButton::clicked, [=]() {
    this->openExternalUrl(kReleasesUrl);
  });
  connect(this->support_button_, &TextButton::clicked, [=]() {
    this->openExternalUrl(kIssueUrl);
  });
}

void AboutWindow::openExternalUrl(const QString& url) {
  const bool ok = QDesktopServices::openUrl(QUrl(url));
  if (!ok) {
    qWarning() << "Failed to open url:" << url;
  }
}

}  // namespace hebo