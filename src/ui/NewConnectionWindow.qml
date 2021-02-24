// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import "items" as Hebo

Item {
  id: root;
  signal connectClicked();

  Hebo.PageTitle {
    id: title;
    text: qsTr("New Connection");
    z: 1;
  }

  Button {
    text: "Connect";
    anchors.top: title.top;
    anchors.topMargin: 10;
    anchors.left: title.right;
    anchors.leftMargin: 24;
    z: 1;

    onClicked: {
      // TODO(Shaohua): Check conn name is unique
      connectManager.addConnection(
        nameField.text,
        clientIdField.text,
        hostProtocol.currentText,
        hostnameField.text,
        portField.number,
        qosField.qos,
        cleanSessionButton.checked
      );
      connectManager.requestConnect(nameField.text);

      root.connectClicked();
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
    topInset: 24;
    rightInset: 14;
    bottomInset: 10;
    leftInset: 14;
    topPadding: 2;
    rightPadding: 18;
    bottomPadding: 18;
    leftPadding: 18;
    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff;

    ColumnLayout {
      spacing: 10;
      width: 580;

      // General
      Hebo.SectionTitle {
        text: qsTr("General");
      }

      Hebo.FormSection {
        width: parent.width;
        height: generalTab.height;

        GridLayout {
          id: generalTab;
          width: parent.width;
          columns: 2;
          columnSpacing: 15;
          rowSpacing: 10;

          Hebo.FormLabel {
            text: qsTr("Name");
            required: true;
          }

          TextField {
            id: nameField;
          }

          Hebo.FormLabel {
            text: qsTr("Client ID");
            required: true;
          }

          TextField {
            id: clientIdField;
          }

          Hebo.FormLabel {
            text: qsTr("Host");
            required: true;
          }

          RowLayout {
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

          Hebo.FormLabel {
            text: qsTr("Port");
            required: true;
          }

          Hebo.NumberField {
            id: portField;
            maxNumber: 65535;
            minNumber: 1;
            number: 1883;
          }

          Hebo.FormLabel {
            text: qsTr("Username");
          }

          TextField {
            id: usernameField;
          }

          Hebo.FormLabel {
            text: qsTr("Password");
          }

          TextField {
            id: passwordField;
            echoMode: TextInput.Password;
          }

          Hebo.FormLabel {
            text: qsTr("SSL/TLS");
          }

          Hebo.SwitchButtons {
            id: tlsButton;
            checked: false;
          }
        }
      }  // generalTab

      // Advanced
      Hebo.SectionTitle {
        text: qsTr("Advanced");
      }

      Hebo.FormSection {
        width: parent.width;
        height: advancedTab.height;

        GridLayout {
          id: advancedTab;
          width: parent.width;
          columns: 2;
          columnSpacing: 15;
          rowSpacing: 10;

          Hebo.FormLabel {
            text: qsTr("Connection Timeout(s)");
            required: true;
          }

          Hebo.NumberField {
            id: connectionTimeoutField;
            minNumber: 0;
            maxNumber: 3600;
            number: 20;
          }

          Hebo.FormLabel {
            text: qsTr("Keep Alive(s)");
          }

          Hebo.NumberField {
            id: keepAliveField;
            minNumber: 10;
            maxNumber: 2^30;
            number: 60;
          }

          Hebo.FormLabel {
            text: qsTr("Clean Session");
          }

          Hebo.SwitchButtons {
            id: cleanSessionButton;
            checked: true;
          }

          Hebo.FormLabel {
            text: qsTr("Auto Reconnect");
          }

          Hebo.SwitchButtons {
            id: autoReconnectButton;
          }

          Hebo.FormLabel {
            text: qsTr("MQTT Version");
          }

          ComboBox {
            id: mqttVersionBox;
            model: ["3.1.1", "5.0",];
          }
        }
      }  // advancedTab


      // Last Will and Testament
      Hebo.SectionTitle {
        text: qsTr("Last Will and Testament");
      }

      Hebo.FormSection {
        width: parent.width;
        height: lastWillTab.height;

        GridLayout {
          id: lastWillTab;
          width: parent.width;
          columns: 2;
          columnSpacing: 15;
          rowSpacing: 10;

          Hebo.FormLabel {
            text: qsTr("Last-Will Topic");
          }

          TextField {
            id: lastWillField;
          }

          Hebo.FormLabel {
            text: qsTr("Last-Will QoS");
          }

          RowLayout {
            id: qosField;
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

          Hebo.FormLabel {
            text: qsTr("Last-Will Payload");
            Layout.alignment: Qt.AlignTop | Qt.AlignRight;
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
        }  // lastWillTab
      }
    }
  }
}
