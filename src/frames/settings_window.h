// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_
#define HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_

#include <QComboBox>
#include <QSpinBox>
#include <QWidget>
#include <rusty/widgets/switch_button.h>

#include "widgets/integer_line_edit.h"
#include "widgets/spin_box.h"

namespace hebo {

class SettingsWindow : public QWidget {
  Q_OBJECT
 public:
  explicit SettingsWindow(QWidget* parent = nullptr);

 public slots:
  void setLocale(const QString& locale);
  void setAutoUpdate(bool auto_update);
  void setRetryConnection(int retry);
  void setNightMode(bool night_mode);

 signals:
  void localeChanged(const QString& locale);
  void autoUpdateChanged(bool auto_update);
  void retryConnectionChanged(int retry);
  void nightModeChanged(bool night_mode);

 private:
  void initUi();
  void initSignals();

  QComboBox* locale_box_{nullptr};
  rusty::SwitchButton* auto_update_button_{nullptr};
  SpinBox* retry_connection_box_{nullptr};
  QComboBox* theme_box_{nullptr};

  QStringList locales_{};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_
