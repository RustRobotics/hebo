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
    Loader {
      sourceComponent: headLabel;

      onLoaded: {
        item.text = qsTr("General");
      }
    }

    Grid {
      columns: 2;
      spacing: 10;

      Loader {
        sourceComponent: formLabel;
        onLoaded: {
          item.text = "Name";
          item.required = true;
        }
      }
    }
  }

  // Local components
  Component {
    id: headLabel;

    Text {
      color: "#232422";
      font.pixelSize: 16;
      font.weight: Font.Bold;
    }
  }

  Component {
    id: formLabel;

    Text {
      property bool required: false;

      color: "#212121";
      font.pixelSize: 14;
      horizontalAlignment: Text.AlignRight;
    }
  }
}
