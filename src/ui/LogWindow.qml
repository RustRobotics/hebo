import QtQuick 2.15

Item {
  id: root;

  Text {
    id: title;
    text: qsTr("Log");
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

}
