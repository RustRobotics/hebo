// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import "items" as Hebo

Item {
  id: root;

  Hebo.PageTitle {
    id: title;
    text: qsTr("About");
  }

  FontLoader {
    id: iconFont;
    source: "fonts/iconfont.ttf";
  }

  Column {
    spacing: 10;
    width: 600;
    anchors.top: title.bottom;
    anchors.horizontalCenter: parent.horizontalCenter;

    Image {
      id: logo;
      source: "images/mqttx-light.png";
      anchors.horizontalCenter: parent.horizontalCenter;
    }

    Text {
      text: "v1.5.2";
      color: "#606060";
      font.pixelSize: 18;
      horizontalAlignment: Qt.AlignHCenter;
      anchors.horizontalCenter: parent.horizontalCenter;
    }

    Row {
      anchors.horizontalCenter: parent.horizontalCenter;

      Hebo.TextButton {
        text: qsTr("Check for Update");
        function clickCallback() {
          updateManager.checkUpdate();
        }
      }

      Hebo.TextButton {
        text: qsTr("Releases");
        link: "https://github.com";
      }

      Hebo.TextButton {
        text: qsTr("Support");
        link: "https://github.com";
      }
    }

    Item {
      width: parent.width;
      height: 16;
    }

    Text {
      width: parent.width;
      anchors.topMargin: 124;
      wrapMode: Text.WordWrap;
      textFormat: Text.RichText;
      font.underline: false;
      text: qsTr('To run MQTT Broker locally, <a href="https://biofan.org" style="text-decoration: none; color: #34c388;">EMQ X</a> is recommended. <a href="https://biofan.org" style="text-decoration: none; color: #34c388;">EMQ X</a> is a fully open source, highly scalable, highly available distributed MQTT 5.0 messaging broker for IoT, M2M and mobile applications.');
      onLinkActivated: Qt.openUrlExternally(link);
    }

    Text {
      width: parent.width;
      wrapMode: Text.WordWrap;
      text: qsTr("Install EMQ X by using Docker:");
    }

    TextArea {
      id: codeEdit;
      wrapMode: TextEdit.Wrap;
      width: parent.width;
      padding: 14;
      readOnly: true;
      focus: true;
      selectByMouse: true;
      selectionColor: "#345ec3";
      selectedTextColor: "#fafafa";
      text: "docker run -d --name emqx -p 1883:1883 -p 8083:8083 -p 8883:8883 -p 8084:8084 -p 18083:18083 emqx/emqx";

      background: Rectangle {
        color: "#e7e7e7";
      }

      Keys.onPressed: {
        if (event.modifiers === Qt.ControlModifier && event.key === Qt.Key_C) {
          this.copy();
          this.deselect();
        }
      }

      MouseArea {
        anchors.fill: codeEdit;
        onClicked: codeEdit.selectAll();
      }
    }

    Item {
      width: parent.width;
      height: 24;
    }

    Button {
      id: githubButton;
      anchors.horizontalCenter: parent.horizontalCenter;
      text: qsTr("Follow us on Github");
      padding: 10;

      contentItem: RowLayout {

        Text {
          text: "\ue62a";
          color: "#fff";
          font.pixelSize: 22;
          font.family: iconFont.name;
        }

        Text {
          text: githubButton.text;
          color: "#fff";
          font.pixelSize: 18;
        }
      }

      background: Rectangle {
        color: "#34c388";
        radius: 4;
      }

      MouseArea {
        anchors.fill: parent;
        hoverEnabled: true;
        cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor;
        onClicked: Qt.openUrlExternally("https://github.com");
      }
    }
  }

  Row {
    id: row
    spacing: 6;

    anchors {
      left: root.left;
      leftMargin: 14;
      bottom: root.bottom;
      bottomMargin: 14;
    }

    Image {
      width: 42;
      smooth: true;
      fillMode: Image.PreserveAspectFit;
      source: "images/emqx-logo.png";
    }

    Text {
      anchors.verticalCenter: parent.verticalCenter
      textFormat: Text.RichText;
      font.pixelSize: 14;
      text: 'Copyright © 2021 <a href="https://biofan.org" style="text-decoration: none; color: #34c388;">EMQ X</a>';
      onLinkActivated: Qt.openUrlExternally(link);
    }
  }

  Row {
    spacing: 20;
    anchors {
      right: root.right;
      rightMargin: 14;
      bottom: root.bottom;
      bottomMargin: 14;
    }

    // Twitter
    Hebo.ImageButton {
      text: "\ue6c7";
      link: "https://twitter.com";
    }

    // Slack
    Hebo.ImageButton {
      text: "\ue641";
      link: "https://slack.com";
    }

    // Reddit
    Hebo.ImageButton {
      text: "\ue7e4";
      link: "https://reddit.com";
    }
  }

}
