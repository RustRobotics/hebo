import QtQuick 2.0
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Rectangle {
  color: "#333844";
  property int currentIndex: 0;

  ButtonGroup {
    id: buttonGroup;
    buttons: column.children;
    exclusive: true;

    onClicked: function(button) {
      for (let i = 0; i < this.buttons.length; ++i) {
        if (button === this.buttons[i]) {
          parent.currentIndex = i;
          console.log("currentIndex:", parent.currentIndex);
          break;
        }
      }
    }
  }

  Column {
    id: column;
    anchors.horizontalCenter: parent.horizontalCenter;
    spacing: 24;

    FontLoader {
      id: iconFont;
      source: "fonts/iconfont.ttf";
    }

    // Connections
    RoundButton {
      checked: true;
      text: "\ue64d";
      font.pixelSize: 16;
      font.family: iconFont.name;
    }

    // New connection
    RoundButton {
      checkable: true;
      text: "\ue64e";
      font.pixelSize: 16;
      font.family: iconFont.name;
    }

    // Log
    RoundButton {
      checkable: true;
      text: "\uea07";
      font.pixelSize: 16;
      font.family: iconFont.name;
    }

    // Info
    RoundButton {
      checkable: true;
      text: "\ue64f";
      font.pixelSize: 16;
      font.family: iconFont.name;
    }

    // Settings
    RoundButton {
      checkable: true;
      text: "\ue627";
      font.pixelSize: 16;
      font.family: iconFont.name;
    }

  }
}
