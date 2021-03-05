import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

ApplicationWindow {
  id: appWindow;
  visible: true;
  width: 1024;
  height: 754;
  title: qsTr("Hebo UI");

  LeftPanel {
    id: leftPanel;
    anchors {
      left: parent.left;
      top: parent.top;
      bottom: parent.bottom;
    }
    width: 88;
  }

  StackLayout {
    id: stackLayout;
    currentIndex: leftPanel.currentIndex;

    onCurrentIndexChanged: {
      if (currentIndex == 1) {
        newConnectionWindow.resetForm();
      }
    }

    anchors {
      left: leftPanel.right;
      right: parent.right;
      top: parent.top;
      bottom: parent.bottom;
    }

    ConnectionsWindow {
    }

    NewConnectionWindow {
      id: newConnectionWindow;
      onConnectClicked: {
        leftPanel.setIndex(0);
      }
    }

    LogWindow {
    }

    AboutWindow {
    }

    SettingsWindow {
    }
  }
}
