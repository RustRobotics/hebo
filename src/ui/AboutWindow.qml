import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
  id: root;
  width: 1024;
  height: 754;

  Text {
    id: title;
    text: qsTr("About");
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

      Button {
        text: qsTr("Check for Updates");

        contentItem: Text {
          color: "#34c388";
          text: parent.text;
        }

        background: Rectangle {
        }

        MouseArea {
          anchors.fill: parent;
          hoverEnabled: true;
          cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor;
        }
      }

      Button {
        text: qsTr("Releases");

        contentItem: Text {
          color: "#34c388";
          text: parent.text;
        }

        background: Rectangle {
        }

        MouseArea {
          anchors.fill: parent;
          hoverEnabled: true;
          cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor;
        }
      }

      Button {
        text: qsTr("Support");

        contentItem: Text {
          color: "#34c388";
          text: parent.text;
        }

        background: Rectangle {
        }

        MouseArea {
          anchors.fill: parent;
          hoverEnabled: true;
          cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor;
        }
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
      textFormat: Text.StyledText;
      linkColor: "#34c388";
      font.underline: false;
      text: qsTr('To run MQTT Broker locally, <a href="https://biofan.org">EMQ X</a> is recommended. <a href="https://biofan.org">EMQ X</a> is a fully open source, highly scalable, highly available distributed MQTT 5.0 messaging broker for IoT, M2M and mobile applications.');
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
      textFormat: Text.StyledText;
      linkColor: "#34c388";
      font.underline: false;
      font.pixelSize: 14;
      text: 'Copyright © 2021 <a href="https://biofan.org">EMQ X</a>';
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
    RoundButton {
      text: "\ue6c7";
      radius: 4;
      font.pixelSize: 20;
      font.family: iconFont.name;

      MouseArea {
        anchors.fill: parent;
        hoverEnabled: true;
        cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor;
      }
    }

    // Slackware
    RoundButton {
      text: "\ue641";
      radius: 4;
      font.pixelSize: 20;
      font.family: iconFont.name;

      MouseArea {
        anchors.fill: parent;
        hoverEnabled: true;
        cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor;
      }
    }

    // Reddit
    RoundButton {
      text: "\ue7e4";
      radius: 4;
      font.pixelSize: 20;
      font.family: iconFont.name;

      MouseArea {
        anchors.fill: parent;
        hoverEnabled: true;
        cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor;
      }
    }
  }

}
