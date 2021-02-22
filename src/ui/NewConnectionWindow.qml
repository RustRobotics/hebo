import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
  id: root;
  width: 1024;
  height: 754;

  Text {
    id: title;
    text: qsTr("New Connection");
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
    anchors.top: title.bottom;
    spacing: 10;
    width: 680;
    anchors.horizontalCenter: parent.horizontalCenter;

    // General
    HeadLabel {
      text: qsTr("General");
    }

    FormSection {
      width: parent.width;
      height: generalTab.height;

      Column {
        id: generalTab;
        padding: 10;
        width: parent.width;

        RowLayout {
          width: parent.width;

          FormLabel {
            text: qsTr("Name");
            required: true;
          }

          TextField {
            id: nameField;
          }
        }

        RowLayout {
          width: parent.width;

          FormLabel {
            text: qsTr("Client ID");
            required: true;
          }

          TextField {
            id: clientIdField;
          }
        }

        RowLayout {
          width: parent.width;

          FormLabel {
            text: qsTr("Host");
            required: true;
          }

          ComboBox {
            id: hostProtocol;
            model: ["mqtt://", "mqttx://", "ws://", "wss://"];
          }

          TextField {
            id: hostnameField;
          }
        }

        RowLayout {
          width: parent.width;

          FormLabel {
            text: qsTr("Port");
            required: true;
          }

          TextField {
            id: portField;
            focus: true;
            validator: IntValidator {
              top: 2^16;
              bottom: 1;
            }
          }
        }

        RowLayout {
          width: parent.width;

          FormLabel {
            text: qsTr("Username");
          }

          TextField {
            id: usernameField;
          }
        }

        RowLayout {
          width: parent.width;

          FormLabel {
            text: qsTr("Password");
          }

          TextField {
            id: passwordField;
          }
        }

        RowLayout {
          width: parent.width;

          FormLabel {
            text: qsTr("SSL/TLS");
          }

          RadioButton {
            id: enableTlsButton;
            text: "true";
          }

          RadioButton {
            id: disableTlsButton;
            checked: true;
            text: "false";
          }
        }
      }
    }
  }

  // Local components
  component HeadLabel : Text {
    color: "#232422";
    font.pixelSize: 16;
    font.weight: Font.Bold;
  }

  component FormSection: Rectangle {
    color: "#fafafa";
    border {
      color: "#e1e1e1";
      width: 1;
    }
    radius: 4;
  }

  component FormLabel: Text {
    property bool required: false;

    color: "#212121";
    font.pixelSize: 14;
    horizontalAlignment: Text.AlignLeft;
    Layout.minimumWidth: 256;
    Layout.maximumWidth: 256;
    Layout.leftMargin: 24;
    Layout.alignment: Qt.AlignVCenter;
  }
}
