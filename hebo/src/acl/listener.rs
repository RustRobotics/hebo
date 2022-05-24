// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{v3, v5};

use super::AclApp;
use crate::commands::{AclToListenerCmd, ListenerToAclCmd};
use crate::error::Error;
use crate::types::SessionGid;

impl AclApp {
    pub(super) async fn handle_listener_cmd(&mut self, cmd: ListenerToAclCmd) -> Result<(), Error> {
        match cmd {
            ListenerToAclCmd::Publish(session_gid, packet) => {
                self.on_listener_publish(session_gid, packet).await
            }
            ListenerToAclCmd::PublishV5(session_gid, packet) => {
                self.on_listener_publish_v5(session_gid, packet).await
            }
            ListenerToAclCmd::Subscribe(session_gid, packet) => {
                self.on_listener_subscribe(session_gid, packet).await
            }
            ListenerToAclCmd::SubscribeV5(session_gid, packet) => {
                self.on_listener_subscribe_v5(session_gid, packet).await
            }
        }
    }

    async fn on_listener_publish(
        &mut self,
        session_gid: SessionGid,
        packet: v3::PublishPacket,
    ) -> Result<(), Error> {
        // TODO(Shaohua): Read acl list from config.
        let accepted = true;
        if let Some(listener_sender) = self.listener_senders.get(&session_gid.listener_id()) {
            let cmd = AclToListenerCmd::PublishAck(session_gid.session_id(), packet, accepted);
            if let Err(err) = listener_sender.send(cmd).await {
                log::error!(
                    "acl: Failed to send publish ack to listener: {:?}, err: {:?}",
                    session_gid,
                    err
                );
            }
        } else {
            log::error!(
                "acl: Failed to find listener sender with id: {}",
                session_gid.listener_id()
            );
        }
        // TODO(Shaohua): Return errors
        Ok(())
    }

    async fn on_listener_publish_v5(
        &mut self,
        session_gid: SessionGid,
        packet: v5::PublishPacket,
    ) -> Result<(), Error> {
        // TODO(Shaohua): Read acl list from config.
        let accepted = true;
        if let Some(listener_sender) = self.listener_senders.get(&session_gid.listener_id()) {
            let cmd = AclToListenerCmd::PublishAckV5(session_gid.session_id(), packet, accepted);
            if let Err(err) = listener_sender.send(cmd).await {
                log::error!(
                    "acl: Failed to send publish ack to listener: {:?}, err: {:?}",
                    session_gid,
                    err
                );
            }
        } else {
            log::error!(
                "acl: Failed to find listener sender with id: {}",
                session_gid.listener_id()
            );
        }
        // TODO(Shaohua): Return errors
        Ok(())
    }

    async fn on_listener_subscribe(
        &mut self,
        session_gid: SessionGid,
        packet: v3::SubscribePacket,
    ) -> Result<(), Error> {
        // TODO(Shaohua): Read acl list from config.
        let accepted = true;
        let mut acks = Vec::with_capacity(packet.topics().len());
        for topic in packet.topics() {
            // TODO(Shaohua): Check topic patterns.
            acks.push(v3::SubscribeAck::QoS(topic.qos()));
        }

        if let Some(listener_sender) = self.listener_senders.get(&session_gid.listener_id()) {
            let cmd =
                AclToListenerCmd::SubscribeAck(session_gid.session_id(), packet, acks, accepted);
            if let Err(err) = listener_sender.send(cmd).await {
                log::error!(
                    "acl: Failed to send subscribe ack to listener: {:?}, err: {:?}",
                    session_gid,
                    err
                );
            }
        } else {
            log::error!(
                "acl: Failed to find listener sender with id: {}",
                session_gid.listener_id()
            );
        }
        // TODO(Shaohua): Return errors
        Ok(())
    }

    async fn on_listener_subscribe_v5(
        &mut self,
        session_gid: SessionGid,
        packet: v5::SubscribePacket,
    ) -> Result<(), Error> {
        // TODO(Shaohua): Read acl list from config.
        let accepted = true;
        let mut reasons = Vec::with_capacity(packet.topics().len());
        for _topic in packet.topics() {
            // TODO(Shaohua): Check topic patterns.
            reasons.push(v5::ReasonCode::Success);
        }

        if let Some(listener_sender) = self.listener_senders.get(&session_gid.listener_id()) {
            let cmd = AclToListenerCmd::SubscribeAckV5(
                session_gid.session_id(),
                packet,
                reasons,
                accepted,
            );
            if let Err(err) = listener_sender.send(cmd).await {
                log::error!(
                    "acl: Failed to send subscribe ack to listener: {:?}, err: {:?}",
                    session_gid,
                    err
                );
            }
        } else {
            log::error!(
                "acl: Failed to find listener sender with id: {}",
                session_gid.listener_id()
            );
        }
        // TODO(Shaohua): Return errors
        Ok(())
    }
}
