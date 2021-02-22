import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
  id: root;

  Text {
    id: title;
    text: qsTr("New Connection");
    padding: 14;
    font {
      pixelSize: 18;
      weight: Font.Bold;
    }

    anchors {
      left: parent.left;
      top: parent.top;
    }
  }

  FontLoader {
    id: iconFont;
    source: "fonts/iconfont.ttf";
  }

  Column {
    id: generalTab;
    anchors.top: title.bottom;
    width: parent.width;

    // General
    HeadLabel {
      text: qsTr("General");
    }

    Grid {
      columns: 2;
      spacing: 10;

      FormLabel {
        text: "Name";
        required: true;
      }
    }
  }

  // Local components
  component HeadLabel : Text {
    color: "#232422";
    font.pixelSize: 16;
    font.weight: Font.Bold;
  }

  component FormLabel: Text {
    property bool required: false;
    color: "#212121";
    font.pixelSize: 14;
    horizontalAlignment: Text.AlignRight;
  }
}
