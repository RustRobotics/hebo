// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_ABOUT_WINDOW_H_
#define HEBO_SRC_FRAMES_ABOUT_WINDOW_H_

#include <QWidget>

#include "widgets/text_button.h"

namespace hebo {

class AboutWindow : public QFrame {
  Q_OBJECT
 public:
  explicit AboutWindow(QWidget* parent = nullptr);

 signals:
  void requestUpdate();

 private:
  void initUi();
  void initSignals();

  void openExternalUrl(const QString& url);

  TextButton* update_button_{nullptr};
  TextButton* releases_button_{nullptr};
  TextButton* support_button_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_ABOUT_WINDOW_H_
