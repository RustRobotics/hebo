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
#include "config/config.h"
#include "frames/software_license_window.h"
#include "resources/images/images.h"
#include "resources/styles/styles.h"

namespace hebo {
namespace {

constexpr const char* kReleasesUrl = "https://github.com/xushaohua/hebo-ui/releases";
constexpr const char* kIssueUrl = "https://github.com/xushaohua/hebo-ui/issue";

constexpr const int kContentMaxWidth = 580;

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

  auto* version_label = new QLabel(QString("v%1").arg(kAppVersion));
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

  this->third_party_software_button_ = new TextButton(tr("Open Source Software"));
  this->third_party_software_button_->setObjectName("third-party-software-button");
  update_layout->addWidget(this->third_party_software_button_);
  update_layout->addStretch();

  auto* server_note_label = new QLabel(tr(
      R"(<p>To run MQTT broker locally,
<a href="https://biofan.org" style="text-decoration: none; color: #34c388;">Hebo</a>
is recommended.
<a href="https://biofan.org" style="text-decoration: none; color: #34c388;">Hebo</a>
is a fully open source, highly scalable, highly available distributed MQTT 5.0 messaging broker for IoT,
M2M and mobile applications.</p>)"));
  server_note_label->setTextFormat(Qt::RichText);
  server_note_label->setFixedSize(kContentMaxWidth, 48);
  server_note_label->setObjectName("server-note-label");
  server_note_label->setOpenExternalLinks(true);
  server_note_label->setAlignment(Qt::AlignLeft);
  server_note_label->setWordWrap(true);
  main_layout->addSpacing(32);
  main_layout->addWidget(server_note_label, 0, Qt::AlignHCenter);

  auto* docker_note_label = new QLabel(tr("Install Hebo by using Docker:"));
  docker_note_label->setObjectName("docker-note-label");
  docker_note_label->setFixedWidth(kContentMaxWidth);
  docker_note_label->setAlignment(Qt::AlignLeft);
  main_layout->addSpacing(10);
  main_layout->addWidget(docker_note_label, 0, Qt::AlignHCenter);

  auto* docker_cmd_label = new QLabel(
      R"(docker run -d --name hebo -p 1883:1883 -p 8083:8083 -p 8883:8883
-p 8084:8084 -p 18083:18083 hebo/hebo)");
  docker_cmd_label->setObjectName("docker-cmd-label");
  docker_cmd_label->setFixedWidth(kContentMaxWidth);
  docker_cmd_label->setWordWrap(true);
  docker_cmd_label->setTextInteractionFlags(Qt::TextSelectableByKeyboard | Qt::TextSelectableByMouse);
  main_layout->addSpacing(10);
  main_layout->addWidget(docker_cmd_label, 0, Qt::AlignHCenter);

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
  connect(this->third_party_software_button_, &TextButton::clicked,
          this, &AboutWindow::showSoftwareLicenseWindow);
}

void AboutWindow::openExternalUrl(const QString& url) {
  const bool ok = QDesktopServices::openUrl(QUrl(url));
  if (!ok) {
    qWarning() << "Failed to open url:" << url;
  }
}

void AboutWindow::showSoftwareLicenseWindow() {
  auto* window = new SoftwareLicenseWindow();
  connect(window, &SoftwareLicenseWindow::requestOpenUrl,
          this, &AboutWindow::openExternalUrl);
  connect(window, &SoftwareLicenseWindow::destroyed,
          window, &SoftwareLicenseWindow::deleteLater);
  window->resize(720, 600);
  window->show();
}

}  // namespace hebo