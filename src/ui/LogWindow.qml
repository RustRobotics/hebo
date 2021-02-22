// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Rectangle {
  id: root;
  color: "#f9fafd";

  Text {
    id: title;
    text: qsTr("Log");
    padding: 14;
    font {
      pixelSize: 18;
      weight: Font.Bold;
    }

    anchors {
      left: parent.left;
      top: parent.top;
    }
  }

  ScrollView {
    anchors {
      top: title.bottom;
      bottom: root.bottom;
    }
    width: root.width;
    topInset: 2;
    rightInset: 14;
    bottomInset: 10;
    leftInset: 14;
    padding: 18;
    background: Rectangle {
      antialiasing: true;
      color: "#fff";
      radius: 4;
      border {
        width: 1;
        color: "#e1e1e1";
      }
    }

    RowLayout {
      anchors.fill: parent;

      ListView {
        width: 30;
        Layout.preferredWidth: 30;
        Layout.fillHeight: true;
        model: logText.text.split(/\n/g);
        delegate: Text {
          text: index + 1;
          horizontalAlignment: Text.AlignRight;
        }
      }

      TextEdit {
        id: logText;
        readOnly: true;
        Layout.fillWidth: true;
        Layout.fillHeight: true;
        focus: true;
        selectByMouse: true;
        selectByKeyboard: true;
        textFormat: TextEdit.PlainText;

        text: "[2021-02-21 07:34:39] [INFO] APP init\n[2021-02-21 07:34:39] [INFO] APP init\n";
      }
    }

  }
}
