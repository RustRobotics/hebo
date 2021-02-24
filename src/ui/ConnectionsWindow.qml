// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import "items" as Hebo

Item {
  id: root;

  Hebo.PageTitle {
    id: title;
    text: qsTr("Connections");
    z: 1;
  }


  ListView {
    id: connectionList;
    anchors.top: title.bottom;
    anchors.left: root.left;
    anchors.bottom: root.bottom;
    width: 240;
    model: connectManager.connList;
    delegate: Text {
      font.pixelSize: 14;
      padding: 20;
      verticalAlignment: Text.AlignVCenter;
      width: connectionList.width;
      text: connectionList.model[index].description;

      Rectangle {
        anchors.fill: parent;
        color: "#c3e6c8";
        opacity: 0.25;
      }
    }

  }

  ScrollView {
  }
}
