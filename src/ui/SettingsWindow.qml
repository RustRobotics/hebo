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

      GridLayout {
        id: generalTab;
        width: parent.width;
        columns: 2;
        columnSpacing: 15;
        rowSpacing: 10;

        Hebo.FormLabel {
          text: qsTr("Language");
        }

        ComboBox {
          id: languageField;
          editable: false;
          model: settingsManager.localeNames;
          currentIndex: settingsManager.localeIndex;

          Binding {
            target: settingsManager;
            property: "localeIndex";
            value: languageField.currentIndex;
          }
        }

        Hebo.FormLabel {
          text: qsTr("Auto check update");
        }

        Switch {
          checked: settingsManager.autoUpdate;
          onToggled: {
            settingsManager.autoUpdate = this.checked;
          }
        }

        Hebo.FormLabel {
          text: qsTr("Max retry connection");
        }

        SpinBox {
          id: maxRetryField;
          from: 0;
          to: 1000;
          value: settingsManager.retryConnections;

          Binding {
            target: settingsManager;
            property: "retryConnections";
            value: maxRetryField.value;
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

      GridLayout {
        id: appearanceTab;
        width: parent.width;
        columns: 2;
        columnSpacing: 15;
        rowSpacing: 10;

        Hebo.FormLabel {
          text: qsTr("Theme");
        }

        ComboBox {
          id: themeField;
          editable: false;
          model: settingsManager.themeNames;
          currentIndex: settingsManager.themeIndex;

          Binding {
            target: settingsManager;
            property: "themeIndex";
            value: themeField.currentIndex;
          }
        }
      }
    }
  }
}
