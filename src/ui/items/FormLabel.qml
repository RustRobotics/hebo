// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Text {
  id: root;
  property bool required: false;

  color: "#212121";
  font.pixelSize: 14;
  horizontalAlignment: Text.AlignRight;
  Layout.rightMargin: 0;
  Layout.alignment: Qt.AlignVCenter | Qt.AlignRight;

  Text {
    text: root.required ? "*" : "";
    anchors.right: root.left;
    width: 12;
    Layout.preferredWidth: width;
    color: "red";
  }
}
