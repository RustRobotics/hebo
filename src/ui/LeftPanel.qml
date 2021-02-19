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

    // Connections
    RoundButton {
      checked: true;
      text: "\u2713";
    }

    // New connection
    RoundButton {
      checkable: true;
      text: "\u2713";
    }

    // Log
    RoundButton {
      checkable: true;
      text: "\u2713";
    }

    // Info
    RoundButton {
      checkable: true;
      text: "\uf6c8";
    }

    // Settings
    RoundButton {
      checkable: true;
      text: "\u2713";
    }

  }
}
