// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_SOFTWARE_LICENSE_WINDOW_H_
#define HEBO_SRC_FRAMES_SOFTWARE_LICENSE_WINDOW_H_

#include <QDialog>
#include <QPushButton>
#include <QTableView>

#include "frames/models/software_license_model.h"

namespace hebo {

class SoftwareLicenseWindow : public QDialog {
  Q_OBJECT
 public:
  explicit SoftwareLicenseWindow(QWidget* parent = nullptr);

 signals:
  void requestOpenUrl(const QString& url);

 private slots:
  void onItemClicked(const QModelIndex& index);

 private:
  void initUi();
  void initSignals();

  QTableView* table_view_{nullptr};
  SoftwareLicenseModel* model_{nullptr};
  QPushButton* close_button_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_SOFTWARE_LICENSE_WINDOW_H_
