// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import org.biofan.hebo 1.0
import "items" as Hebo

Item {
  id: root;
  property string name;

  property MqttClient client;

  ColumnLayout {
    anchors.fill: parent;

    Row {

      Text {
        text: root.name;
        font.pixelSize: 16;
        color: "#4d4d4d";
      }

      Text {
        width: 36;
        font.pixelSize: 16;
        color: "red";
        text: root.client.state;
      }

      Button {
        text: "Connect";
        //visible: !root.client || root.client.state === MqttClient.Disconnected || root.client.state === MqttClient.Connected;
        onClicked: {
          console.log("Do connect client");
          console.log("client state:", root.client.state);
          root.client.requestConnect();
        }
      }

      Button {
        text: "Disconnect";
        //visible: root.client && root.client.state !== MqttClient.Disconnected && root.client.state !== MqttClient.Connected;
        onClicked: {
          root.client.requestDisconnect();
        }
      }

      Button {
        text: "Edit";
        onClicked: {
          console.log("Edit connection");
        }
      }

      Button {
        text: "NewWindow";
        onClicked: {
          console.log("popup new window");
        }
      }
    }

    RowLayout {

      ColumnLayout {
        id: topicLayout;
        Layout.preferredWidth: 202;
        spacing: 12;

        Button {
          text: qsTr("New Subscription");
          onClicked: {
            console.log("Show new subscription window");
            if (root.client.state === MqttClient.ConnectionConnected) {
              newSubscriptionDialog.reset();
              newSubscriptionDialog.open();
            } else {
              console.warn("Invalid connection state");
            }
          }
        }

        ScrollView {
          Layout.fillHeight: true;

          ListView {
            id: subscriptionsList;
            model: root.client.subscriptions;
            spacing: 9;

            delegate: Rectangle {
              color: "#eaeaea";
              radius: 4;
              width: topicLayout.Layout.preferredWidth;
              height: topicLabel.height + 24;

              MouseArea {
                id: unsubscribeMA;
                anchors.fill: parent;
                hoverEnabled: true;
                onClicked: {
                  console.log("clicked, filter topic");
                }
              }

              Button {
                id: unsubscribeButton;
                visible: unsubscribeMA.containsMouse;
                anchors.right: parent.right;
                anchors.top: parent.top;
                text: "X";

                background: Rectangle {
                  color: "red";
                  opacity: 1;
                  width: 24;
                  height: 24;
                  radius: 12;
                }

                onClicked: {
                  // TODO(Shaohua): Check connection state.
                  root.client.requestUnsubscribe(model.topic);
                }
              }

              RowLayout {
                anchors.fill: parent;
                anchors.leftMargin: 8;
                anchors.rightMargin: 8;
                spacing: 8;

                Rectangle {
                  width: 16;
                  height: 16;
                  radius: 4;
                  color: model.color;
                }

                Text {
                  id: topicLabel;
                  text: model.topic;
                }

                Text {
                  Layout.alignment: Qt.AlignRight;
                  horizontalAlignment: Text.AlignRight;
                  color: "#313131";
                  text: "QoS " + model.qos;
                }
              }
            }
          }
        }

      }

      ColumnLayout {

        ScrollView {
          Layout.fillWidth: true;
          Layout.fillHeight: true;

          TextEdit {
            id: messagesField;
            anchors.fill: parent;
          }
        }

        TextField {
          id: topicField;
          Layout.fillWidth: true;
          placeholderText: qsTr("Topic");
        }

        TextArea {
          id: payloadField;
          height: 148;
          Layout.fillWidth: true;
          Layout.preferredHeight: height;
          background: Rectangle {
            anchors.fill: parent;
            color: "#a9a9a9";
            opacity: 0.24;
          }

          Button {
            anchors.right: parent.right;
            anchors.bottom: parent.bottom;
            text: "Send";
            onClicked: {
              console.log("publish msg");
              root.client.requestPublish(topicField.text, HeboNs.AtMostOnce, payloadField.text);
            }
          }
        }
      }
    }
  }

  Hebo.NewSubscriptionDialog {
    id: newSubscriptionDialog;

    onAccepted: {
      const fields = this.fields();
      console.log("fields:", fields);
      root.client.requestSubscribe(fields.topic, fields.qos, fields.color);
    }
  }

  Component.onCompleted: {
    this.client = connectManager.client(this.name);
    console.log("client:", client);
    console.log("QoS:", HeboNs.AtMostOnce);
    console.log("state:", this.client.state);
  }
}
