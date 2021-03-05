// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Dialog {
  id: root;
  title: qsTr("New Subscription");
  modal: true;

  property string topic: topicField.text;
  property int qos: qosField.currentIndex;
  property string color: colorField.text;

  function reset() {
    topicField.reset();
    qosField.currentIndex = 0;
    colorField.text = "";
  }

  GridLayout {
    width: parent.width;
    columns: 2;
    columnSpacing: 15;
    rowSpacing: 10;

    FormLabel {
      text: qsTr("Topic");
      required: true;
    }

    FormField {
      id: topicField;
      isValid: text.length > 0;
    }

    FormLabel {
      text: qsTr("QoS");
    }

    ComboBox {
      id: qosField;
      model: ["0", "1", "2"];
    }

    FormLabel {
      text: qsTr("Color");
    }

    TextField {
      id: colorField;
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
