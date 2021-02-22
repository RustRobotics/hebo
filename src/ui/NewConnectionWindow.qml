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

    Column {
      spacing: 10;
      width: 580;
      anchors.centerIn: parent;

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

            SwitchButtons {
              id: tlsButton;
              checked: true;
            }
          }
        }
      }  // generalTab

      // Advanced
      HeadLabel {
        text: qsTr("Advanced");
      }

      FormSection {
        width: parent.width;
        height: advancedTab.height;

        Column {
          id: advancedTab;
          padding: 10;
          width: parent.width;

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("Connection Timeout(s)");
              required: true;
            }

            TextField {
              id: connectionTimeoutField;
            }
          }

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("Keep Alive(s)");
            }

            TextField {
              id: keepAliveField;
            }
          }

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("Clean Session");
            }

            SwitchButtons {
              id: cleanSessionButton;
              checked: true;
            }
          }

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("Auto Reconnect");
            }

            SwitchButtons {
              id: autoReconnectButton;
            }
          }

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("MQTT Version");
            }

            ComboBox {
              id: mqttVersionBox;
              model: ["3.1.1", "5.0",];
            }
          }
        }
      }  // advancedTab


      // Last Will and Testament
      HeadLabel {
        text: qsTr("Last Will and Testament");
      }

      FormSection {
        width: parent.width;
        height: lastWillTab.height;

        Column {
          id: lastWillTab;
          padding: 10;
          width: parent.width;

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("Last-Will Topic");
            }

            TextField {
              id: lastWillField;
            }
          }

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("Last-Will QoS");
            }

            RowLayout {
              property int qos: 0;

              RadioButton {
                checked: parent.qos === 0;
                text: "0";
              }

              RadioButton {
                checked: parent.qos === 1;
                text: "1";
              }

              RadioButton {
                checked: parent.qos === 2;
                text: "2";
              }
            }
          }

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("Last-Will Payload");
            }

            ColumnLayout {
              TextArea {
                width: parent.width;
                id: lastWillPayloadField;
              }

              RowLayout {
                id: lastWillPayloadFormat;
                property string format: "text";

                RadioButton {
                  checked: parent.format === "json";
                  text: "JSON";
                }

                RadioButton {
                  checked: parent.format === "text";
                  text: "text";
                }
              }
            }
          }

        }  // lastWillTab
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

  component SwitchButtons: RowLayout {
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
}
