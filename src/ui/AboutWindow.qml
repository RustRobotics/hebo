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
}
