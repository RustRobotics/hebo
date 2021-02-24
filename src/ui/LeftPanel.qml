import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Rectangle {
  id: root;
  color: "#333844";
  property int currentIndex: 0;

  function setIndex(index) {
    this.currentIndex = index;
    buttonGroup.setIndex(currentIndex);
  }

  ButtonGroup {
    id: buttonGroup;
    exclusive: true;

    onClicked: root.currentIndex = button.index;
    function setIndex(index) {
      for (let i = 0; i < buttons.length; ++i) {
        if (buttons[i].index === index) {
          this.checkedButton = buttons[i];
          break;
        }
      }
    }
  }

  FontLoader {
    id: iconFont;
    source: "fonts/iconfont.ttf";
  }

  Column {
    id: topButtons;
    anchors{
      horizontalCenter: parent.horizontalCenter;
      top: root.top;
      topMargin: 24;
    }
    spacing: 36;

    // Connections
    RoundFontButton {
      checked: true;
      index: 0;
      text: "\ue64d";
    }

    // New connection
    RoundFontButton {
      index: 1;
      text: "\ue64e";
    }

    // Log
    RoundFontButton {
      index: 2;
      text: "\uea07";
    }
  }

  Column {
    id: bottomButtons;
    anchors {
      horizontalCenter: parent.horizontalCenter;
      bottom: root.bottom;
      bottomMargin: 24;
    }
    spacing: 36;

    // Info
    RoundFontButton {
      index: 3;
      text: "\ue64f";
    }

    // Settings
    RoundFontButton {
      index: 4;
      text: "\ue627";
    }
  }

  component RoundFontButton: RoundButton {
    property int index;
    checkable: true;
    font.pixelSize: 16;
    font.family: iconFont.name;
    ButtonGroup.group: buttonGroup;
  }
}
