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

  Button {
    text: "-";
    width: 24;
    height: root.height;
    onClicked: {
      if (root.number > root.minNumber) {
        root.number -= 1;
      }
    }
  }

  TextField {
    text: root.number;
    validator: IntValidator {
      top: root.maxNumber;
      bottom: root.minNumber;
    }
  }

  Button {
    text: "+";
    width: 24;
    height: root.height;
    onClicked: {
      if (root.number < root.maxNumber) {
        root.number += 1;
      }
    }
  }
}
