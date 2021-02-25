// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
  id: root;
  property string name;

  ColumnLayout {
    anchors.fill: parent;

    Row {

      Text {
        text: root.name;
        font.pixelSize: 16;
        color: "#4d4d4d";
      }

      Button {
        text: "Connect";
        onClicked: {
          console.log("Do connect client");
          connectManager.requestConnect(root.name);
        }
      }

      Button {
        text: "Edit";
        onClicked: {
          console.log("Edit connection");
        }
      }

      Button {
        text: "NewWindow";
        onClicked: {
          console.log("popup new window");
        }
      }
    }

    TextField {
      id: topicField;
      Layout.fillWidth: true;
      placeholderText: qsTr("Topic");
    }

    TextArea {
      id: payloadField;
      height: 148;
      Layout.fillWidth: true;
      Layout.preferredHeight: height;
      background: Rectangle {
        anchors.fill: parent;
        color: "#a9a9a9";
        opacity: 0.24;
      }

      Button {
        anchors.right: parent.right;
        anchors.bottom: parent.bottom;
        text: "Send";
        onClicked: {
          console.log("publish msg");
        }
      }
    }
  }
}
