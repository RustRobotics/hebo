// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Dialog {
  id: root;
  title: "New Subscription";
  modal: true;
  standardButtons: Dialog.Cancel | Dialog.Ok;

  //property string topic: "";
  //property int qos: 0;
  //property string color: "";

  function reset() {
    topicField.text = "";
    qosField.currentIndex = 0;
    colorField.text = "";
  }

  function fields() {
    return {
      topic: topicField.text,
      qos: qosField.currentIndex,
      color: colorField.text,
    }
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

    TextField {
      id: topicField;
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
}
