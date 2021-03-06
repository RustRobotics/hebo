// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Dialog {
  id: root;
  title: qsTr("Edit Subscription");
  modal: true;

  GridLayout {
    width: root.width;
    columns: 2;
    columnSpacing: 15;
    rowSpacing: 10;

    FormLabel {
      text: qsTr("Name");
      required: true;
    }

    FormField {
      id: nameField;
      isValid: text.length > 0;
    }

    FormLabel {
      text: qsTr("Client ID");
    }

    Row {
      spacing: 24;

      Hebo.FormField {
        id: clientIdField;
        isValid: this.text.length > 0;
      }

      RoundButton {
        text: "\ue6d0";
        ToolTip.text: qsTr("Generate random client id");
        font.pixelSize: 14;
        font.family: iconFont.name;

        onClicked: {
          const clientId = connectManager.newClientId();
          clientIdField.text = clientId;
        }
      }
    }

    FormLabel {
      text: qsTr("Username");
    }

    TextField {
      id: usernameField;
    }

    FormLabel {
      text: qsTr("Password");
    }

    TextField {
      id: passwordField;
      echoMode: TextInput.Password;
    }

    FormLabel {
      text: qsTr("Keep Alive");
    }

    SpinBox {
      id: keepAliveField;
      from: 10;
      to: 2 ^ 30;
      value: 60;
      editable: true;
    }

    Hebo.FormLabel {
      text: qsTr("Clean Session");
    }

    Hebo.SwitchButtons {
      id: cleanSessionButton;
      checked: true;
    }
  }

  footer: DialogButtonBox {
    Button {
      text: qsTr("Cancel");
      DialogButtonBox.buttonRole: DialogButtonBox.RejectRole;
    }

    Button {
      text: qsTr("Subscribe");
      DialogButtonBox.buttonRole: DialogButtonBox.AcceptRole;

      MouseArea {
        anchors.fill: parent;
        onClicked: {
          topicField.runValidate();
          if (topicField.isValid) {
            root.accept();
          }
        }
      }
    }
  }
}
