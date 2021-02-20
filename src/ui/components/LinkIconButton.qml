import QtQuick 2.15
import QtQuick.Controls 2.15

RoundButton {
  property string link: "";

  radius: 4;
  font.pixelSize: 20;
  font.family: iconFont.name;

  MouseArea {
    anchors.fill: parent;
    hoverEnabled: true;
    cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor;
    onClicked: Qt.openUrlExternally(parent.link);
  }
}
