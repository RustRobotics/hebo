// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/software_license_window.h"

#include <QHeaderView>
#include <QLabel>
#include <QStandardItemModel>
#include <QVBoxLayout>

#include "frames/delegates/software_license_delegate.h"

namespace hebo {

SoftwareLicenseWindow::SoftwareLicenseWindow(QWidget* parent) : QWidget(parent) {
  this->initUi();
  this->initSignals();
}

void SoftwareLicenseWindow::initUi() {
  auto* main_layout = new QVBoxLayout();
  this->setLayout(main_layout);

  this->table_view_ = new QTableView();
  main_layout->addWidget(this->table_view_);
  this->model_ = new SoftwareLicenseModel(this);
  this->table_view_->setModel(this->model_);
  this->table_view_->setShowGrid(false);
  this->table_view_->setSortingEnabled(false);
  this->table_view_->verticalHeader()->hide();
  this->table_view_->setMouseTracking(true);
  this->table_view_->setSelectionMode(QTableView::SelectionMode::SingleSelection);

  auto* horizontal_header = this->table_view_->horizontalHeader();
  horizontal_header->setSectionResizeMode(QHeaderView::ResizeMode::Stretch);
  horizontal_header->setSectionsClickable(false);
  horizontal_header->setSectionsMovable(false);
  horizontal_header->setBackgroundRole(QPalette::Window);

  auto* delegate = new SoftwareLicenseDelegate(this);
  this->table_view_->setItemDelegate(delegate);

  this->close_button_ = new QPushButton(tr("Close"));
  main_layout->addWidget(this->close_button_, 0, Qt::AlignRight);
}

void SoftwareLicenseWindow::initSignals() {
  connect(this->close_button_, &QPushButton::clicked,
          this, &SoftwareLicenseWindow::close);
  connect(this->table_view_, &QTableView::clicked,
          this, &SoftwareLicenseWindow::onItemClicked);
}

void SoftwareLicenseWindow::onItemClicked(const QModelIndex& index) {
  if (!index.isValid()) {
    return;
  }
  if (index.column() == SoftwareLicenseModel::kSoftwareColumn) {
    const QString url = index.data(SoftwareLicenseModel::kUrlRole).toString();
    if (!url.isEmpty()) {
      emit this->requestOpenUrl(url);
    }
  } else {
    const QString license_url = index.data(SoftwareLicenseModel::kLicenseUrlRole).toString();
    if (!license_url.isEmpty()) {
      emit this->requestOpenUrl(license_url);
    }
  }
}

}  // namespace hebo