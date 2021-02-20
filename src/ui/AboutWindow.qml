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
      horizontalAlignment: Qt.AlignHCenter;
      anchors.horizontalCenter: parent.horizontalCenter;
    }

    Row {
      anchors.horizontalCenter: parent.horizontalCenter;

      Button {
        text: qsTr("Check for Updates");
      }

      Button {
        text: qsTr("Releases");
      }

      Button {
        text: qsTr("Support");
      }
    }

    Text {
      width: parent.width;
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

    Button {
      id: githubButton;
      anchors.horizontalCenter: parent.horizontalCenter;
      text: qsTr("Follow us on Github");
      padding: 10;

      contentItem: RowLayout {

        Text {
          text: "\ue62a";
          color: "#fafafa";
          font.pixelSize: 18;
          font.family: iconFont.name;
        }

        Text {
          text: githubButton.text;
          color: "#fafafa";
          font.pixelSize: 16;
        }
      }

      background: Rectangle {
        color: "#34c388";
        radius: 4;
      }
    }
  }

  // Reddit
  RoundButton {
    id: redditButton;
    anchors {
      right: root.right;
      rightMargin: 14;
      bottom: root.bottom;
      bottomMargin: 14;
    }

    text: "\ue7e4";
    radius: 4;
    font.pixelSize: 20;
    font.family: iconFont.name;
  }

  // Slackware
  RoundButton {
    id: slackButton;
    anchors {
      bottom: redditButton.bottom;
      right: redditButton.left;
      rightMargin: 14;
    }

    text: "\ue641";
    radius: 4;
    font.pixelSize: 20;
    font.family: iconFont.name;
  }

  // Twitter
  RoundButton {
    anchors {
      bottom: redditButton.bottom;
      right: slackButton.left;
      rightMargin: 14;
    }

    text: "\ue6c7";
    radius: 4;
    font.pixelSize: 20;
    font.family: iconFont.name;
  }
}
