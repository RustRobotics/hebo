// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import "items" as Hebo

Item {
  id: root;

  Hebo.PageTitle {
    id: title;
    text: qsTr("Settings");
  }

  ColumnLayout {
    spacing: 20;
    anchors.top: title.bottom;
    width: 580;
    anchors.horizontalCenter: parent.horizontalCenter;

    Hebo.SectionTitle {
      text: qsTr("General");
    }

    Hebo.FormSection {
      width: parent.width;
      height: generalTab.height;

      Column {
        id: generalTab;
        width: parent.width;
        padding: 10;

        RowLayout {
          Hebo.FormLabel {
            text: qsTr("Language");
          }

          ComboBox {
            editable: false;
            model: ["English", "简体中文",];
          }
        }

        RowLayout {
          Hebo.FormLabel {
            text: qsTr("Auto check update");
          }

          Switch {
            checked: settingsManager.autoUpdate;
            onToggled: {
              settingsManager.autoUpdate = this.checked;
            }
          }
        }

        RowLayout {
          Hebo.FormLabel {
            text: qsTr("Max retry connection");
          }

          Hebo.NumberField {
            id: maxRetryField;
            minNumber: 0;
            maxNumber: 1000;
            number: settingsManager.retryConnections;

            Binding {
              target: settingsManager;
              property: "retryConnections";
              value: maxRetryField.number;
            }
          }
        }
      }
    }

    Hebo.SectionTitle {
      text: qsTr("Appearance");
    }

    // Appearance
    Hebo.FormSection {
      width: parent.width;
      implicitHeight: appearanceTab.implicitHeight;

      Column {
        id: appearanceTab;
        width: parent.width;
        padding: 10;

        RowLayout {
          Hebo.FormLabel {
            text: qsTr("Theme");
          }

          ComboBox {
            id: themeField;
            editable: false;
            model: settingsManager.themeNames;
            currentIndex: settingsManager.themeId;

            Binding {
              target: settingsManager;
              property: "themeId";
              value: themeField.currentIndex;
            }
          }
        }
      }
    }
  }
}
