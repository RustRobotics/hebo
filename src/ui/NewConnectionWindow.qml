// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import "items"

Item {
  id: root;

  PageTitle {
    id: title;
    text: qsTr("New Connection");
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

      // General
      SectionTitle {
        text: qsTr("General");
      }

      FormSection {
        width: parent.width;
        height: generalTab.height;

        Column {
          id: generalTab;
          padding: 10;
          spacing: 10;
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
              width: 94;
              Layout.preferredWidth: 94;
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

            NumberField {
              id: portField;
              maxNumber: 65535;
              minNumber: 1;
              number: 1883;
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
              echoMode: TextInput.Password;
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
      SectionTitle {
        text: qsTr("Advanced");
      }

      FormSection {
        width: parent.width;
        height: advancedTab.height;

        Column {
          id: advancedTab;
          padding: 10;
          spacing: 10;
          width: parent.width;

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("Connection Timeout(s)");
              required: true;
            }

            NumberField {
              id: connectionTimeoutField;
              minNumber: 0;
              maxNumber: 3600;
              number: 20;
            }
          }

          RowLayout {
            width: parent.width;

            FormLabel {
              text: qsTr("Keep Alive(s)");
            }

            NumberField {
              id: keepAliveField;
              minNumber: 10;
              maxNumber: 2^30;
              number: 60;
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
      SectionTitle {
        text: qsTr("Last Will and Testament");
      }

      FormSection {
        width: parent.width;
        height: lastWillTab.height;

        Column {
          id: lastWillTab;
          padding: 10;
          spacing: 10;
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
              Layout.alignment: Qt.AlignTop;
              topPadding: 14;
            }

            ColumnLayout {
              width: parent.width;

              ScrollView {
                width: 390;
                Layout.preferredWidth: 390;
                height: 124;
                Layout.preferredHeight: 124;

                TextArea {
                  id: lastWillPayloadField;
                  readOnly: false;
                  selectByMouse: true;
                  selectByKeyboard: true;
                  wrapMode: TextEdit.WrapAnywhere;
                  text: "Hello, world";

                  background: Rectangle {
                    anchors.fill: parent;
                    border {
                      width: 1;
                      color: "#e1e1e1";
                    }
                  }
                }
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
}
