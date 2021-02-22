// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import "items"

Item {
  id: root;

  PageTitle {
    id: title;
    text: qsTr("Settings");
  }

  ColumnLayout {
    spacing: 20;
    anchors.top: title.bottom;
    width: 580;
    anchors.horizontalCenter: parent.horizontalCenter;

    SectionTitle {
      text: qsTr("General");
    }

    FormSection {
      width: parent.width;
      height: generalTab.height;

      Column {
        id: generalTab;
        width: parent.width;
        padding: 10;

        RowLayout {
          FormLabel {
            text: qsTr("Language");
          }

          ComboBox {
            editable: false;
            model: ["English", "简体中文",];
          }
        }

        RowLayout {
          FormLabel {
            text: qsTr("Auto check update");
          }

          Switch {
          }
        }

        RowLayout {
          FormLabel {
            text: qsTr("Max retry connection");
          }

          TextField {
            focus: true;
            validator: IntValidator {
              top: 2^20;
              bottom: 1;
            }
            text: "20";
          }
        }
      }
    }

    SectionTitle {
      text: qsTr("Appearance");
    }

    // Appearance
    FormSection {
      width: parent.width;
      implicitHeight: appearanceTab.implicitHeight;

      Column {
        id: appearanceTab;
        width: parent.width;
        padding: 10;

        RowLayout {
          FormLabel {
            text: qsTr("Theme");
          }

          ComboBox {
            editable: false;
            model: ["Light", "Dark", "Night",];
          }
        }
      }
    }
  }
}
