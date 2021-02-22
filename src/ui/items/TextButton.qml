// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15

Button {
  property string link;

  contentItem: Text {
    color: "#34c388";
    text: parent.text;
  }

  background: Rectangle {
  }

  MouseArea {
    anchors.fill: parent;
    hoverEnabled: true;
    cursorShape: containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor;
    onClicked: {
      if (!!parent.link) {
        Qt.openUrlExternally(parent.link);
      }
    }
  }
}
