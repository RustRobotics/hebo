// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_
#define HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_

#include <QComboBox>
#include <QFrame>
#include <QSpinBox>

#include "widgets/integer_line_edit.h"
#include "widgets/spin_box.h"
#include "widgets/switch_button.h"

namespace hebo {

class SettingsWindow : public QFrame {
  Q_OBJECT
 public:
  explicit SettingsWindow(QWidget* parent = nullptr);

 public slots:
  void setLocale(const QString& locale);
  void setAutoUpdate(bool auto_update);
  void setRetryConnection(int retry);
  void setTheme(const QString& theme);

 signals:
  void localeChanged(const QString& locale);
  void autoUpdateChanged(bool auto_update);
  void retryConnectionChanged(int retry);
  void themeChanged(const QString& theme);

 private:
  void initUi();
  void initSignals();

  QComboBox* locale_box_{nullptr};
  SwitchButton* auto_update_button_{nullptr};
  SpinBox* retry_connection_box_{nullptr};
  QComboBox* theme_box_{nullptr};

  QStringList locales_{};
  QStringList themes_{};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_
