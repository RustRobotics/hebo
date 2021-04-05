// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_
#define HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_

#include <QComboBox>
#include <QFrame>
#include <QSpinBox>

namespace hebo {

class SettingsWindow : public QFrame {
  Q_OBJECT
 public:
  explicit SettingsWindow(QWidget* parent = nullptr);

 private:
  void initUi();
  void initSignals();

  QComboBox* languages_box_{nullptr};

  QSpinBox* retry_connections_box_{nullptr};
  QComboBox* theme_box_{nullptr};

  QStringList locale_names_{};
  QStringList locales_{};
  QStringList theme_names_{};
  QStringList themes_{};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_
