// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_BENCHMARK_WINDOW_H_
#define HEBO_SRC_FRAMES_BENCHMARK_WINDOW_H_

#include <QFrame>

namespace hebo {

class BenchmarkWindow : public QFrame {
  Q_OBJECT
 public:
  explicit BenchmarkWindow(QWidget* parent = nullptr);

 private:
  void initUi();
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_BENCHMARK_WINDOW_H_
