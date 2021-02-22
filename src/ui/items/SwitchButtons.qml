// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

RowLayout {
  property bool checked: false;

  RadioButton {
    id: enableCleanSessionButton;
    checked: parent.checked;
    text: "true";
  }

  RadioButton {
    id: disableCleanSessionButton;
    checked: !parent.checked;
    text: "false";
  }
}
