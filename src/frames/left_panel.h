// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBOUI_SRC_FRAMES_LEFT_PANEL_H_
#define HEBOUI_SRC_FRAMES_LEFT_PANEL_H_

#include <QButtonGroup>
#include <QWidget>

namespace hebo {

class LeftPanel : public QWidget {
  Q_OBJECT
  Q_PROPERTY(ButtonId button READ activeButton WRITE setActiveButton NOTIFY activeChanged)
 public:
  enum ButtonId : uint8_t {
    kMessages,
    kBenchmark,
    kBag,
    kLog,
    kAbout,
    kSettings,
  };
  Q_ENUM(ButtonId);

  explicit LeftPanel(QWidget* parent = nullptr);

  [[nodiscard]] ButtonId activeButton() const;

 public slots:
  void setActiveButton(ButtonId id);

 signals:
  void activeChanged(ButtonId id);

 private:
  void initUi();
  void initSignals();

  QButtonGroup* btn_group_{nullptr};
};

}  // namespace hebo

#endif  // HEBOUI_SRC_FRAMES_LEFT_PANEL_H_
