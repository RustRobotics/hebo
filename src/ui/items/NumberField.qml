// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Row {
  id: root;
  property int maxNumber;
  property int minNumber;
  property int number;

  width: 168;

  function plusNumber() {
    if (number < maxNumber) {
      number += 1;
    }
  }

  function minusNumber() {
    if (number > minNumber) {
      number -= 1;
    }
  }

  Button {
    text: "-";
    width: 24;
    height: root.height;
    onClicked: root.minusNumber();
  }

  TextField {
    width: root.width - 24 * 2;

    text: root.number;
    validator: IntValidator {
      top: root.maxNumber;
      bottom: root.minNumber;
    }

    Keys.onUpPressed: root.plusNumber();
    Keys.onDownPressed: root.minusNumber();
  }

  Button {
    text: "+";
    width: 24;
    height: root.height;
    onClicked: root.plusNumber();
  }
}
