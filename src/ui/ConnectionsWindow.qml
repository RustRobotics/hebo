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
    keyNavigationEnabled: true;

    delegate: Item {
      width: connectionList.width;
      height: 60;

      MouseArea {
        anchors.fill: parent;
        onClicked: connectionList.currentIndex = index;
      }

      Rectangle {
        anchors.fill: rowItem;
        color: "#b0f9aa";
        visible: connectionList.currentIndex == index;
      }

      Row {
        id: rowItem;
        anchors.fill: parent;
        leftPadding: 14;
        spacing: 8;

        Rectangle {
          color: connectionList.model[index].state === 2 ? "#39d12d" : "#606060";
          width: 8;
          height: 8;
          radius: 4;
          anchors.verticalCenter: parent.verticalCenter;
        }

        Text {
          font.pixelSize: 14;
          anchors.verticalCenter: parent.verticalCenter;
          color: "#000";
          text: connectionList.model[index].description;
        }
      }
    }

  }

  ScrollView {
  }
}
