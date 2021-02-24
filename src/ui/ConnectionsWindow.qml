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
      text: "Hello";
    }

    Rectangle {
      anchors.fill: parent;
      color: "red";
      opacity: 0.24;
    }

  }

  ScrollView {
  }
}
