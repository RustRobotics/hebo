// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import org.biofan.hebo 1.0
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
    model: connectManager.model;
    keyNavigationEnabled: true;

    onCurrentIndexChanged: {
      console.log("current index:", this.currentIndex);
      const config = connectManager.row(this.currentIndex);
      console.log("config:", JSON.stringify(config));
      stackView.switchClient(config.name);
    }

    delegate: Item {
      width: connectionList.width;
      height: 60;

      MouseArea {
        anchors.fill: parent;
        onClicked: {
          console.log("model:", model);
          connectionList.currentIndex = index;
        }
      }

      Rectangle {
        anchors.fill: rowItem;
        color: "#b0f9aa";
        visible: connectionList.currentIndex === index;
      }

      Row {
        id: rowItem;
        anchors.fill: parent;
        leftPadding: 14;
        spacing: 8;

        Rectangle {
          color: model.state === MqttClient.ConnectionConnected ? "#39d12d" : "#606060";
          width: 8;
          height: 8;
          radius: 4;
          anchors.verticalCenter: parent.verticalCenter;
        }

        Text {
          font.pixelSize: 14;
          anchors.verticalCenter: parent.verticalCenter;
          color: "#000";
          text: model.description;
        }
      }
    }
  }

  // Right panel
  StackLayout {
    id: stackView;

    anchors {
      left: connectionList.right;
      right: root.right;
      top: title.top;
      bottom: root.bottom;
    }

    function switchClient(name) {
      for (let index = 0; index < this.children.length; ++index) {
        if (this.children[index].name === name) {
          this.currentIndex = index;
          return;
        }
      }

      const newItem = clientControl.createObject(null, {name: name});
      this.children.push(newItem);
      this.currentIndex = this.count - 1;
    }
  }

  Component.onCompleted: {
  }

  Component {
    id: clientControl;

    ClientControl {
    }
  }
}
