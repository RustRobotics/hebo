// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Handles commands and new connections

use codec::{
    ConnectAckPacket, ConnectPacket, ConnectReturnCode, PublishPacket, SubscribeAck,
    SubscribeAckPacket, SubscribePacket, SubscribedTopic, UnsubscribePacket,
};
use tokio::sync::mpsc;

use super::Listener;
use super::Pipeline;
use super::CHANNEL_CAPACITY;
use crate::commands::{
    AclToListenerCmd, AuthToListenerCmd, DispatcherToListenerCmd, ListenerToAuthCmd,
    ListenerToDispatcherCmd, ListenerToSessionCmd, SessionToListenerCmd,
};
use crate::error::Error;
use crate::session::Session;
use crate::stream::Stream;
use crate::types::SessionId;

// TODO(Shaohua): Move to dispatcher app.
fn topic_match(topics: &[SubscribedTopic], topic_str: &str) -> bool {
    for topic in topics {
        if topic.topic().is_match(topic_str) {
            return true;
        }
    }
    false
}

impl Listener {
    pub async fn run_loop(&mut self) -> ! {
        // Take ownership of mpsc receiver or else tokio select will raise error.
        let mut session_receiver = self
            .session_receiver
            .take()
            .expect("Invalid session receiver");

        let mut dispatcher_receiver = self
            .dispatcher_receiver
            .take()
            .expect("Invalid dispatcher receiver");
        let mut auth_receiver = self.auth_receiver.take().expect("Invalid auth receiver");
        let mut acl_receiver = self.acl_receiver.take().expect("Invalid acl receiver");

        loop {
            tokio::select! {
                Ok(stream) = self.accept() => {
                    self.new_connection(stream).await;
                },

                Some(cmd) = session_receiver.recv() => {
                    if let Err(err) = self.handle_session_cmd(cmd).await {
                        log::error!("handle session cmd failed: {:?}", err);
                    }
                },

                Some(cmd) = dispatcher_receiver.recv() => {
                    self.handle_dispatcher_cmd(cmd).await;
                }

                Some(cmd) = auth_receiver.recv() => {
                    if let Err(err) = self.handle_auth_cmd(cmd).await {
                        log::error!("handle auth cmd failed: {:?}", err);
                    }
                }

                Some(cmd) = acl_receiver.recv() => {
                    if let Err(err) = self.handle_acl_cmd(cmd).await {
                        log::error!("handle acl cmd failed: {:?}", err);
                    }
                }
            }
        }
    }

    async fn new_connection(&mut self, stream: Stream) {
        let (sender, receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let session_id = self.next_session_id();
        let pipeline = Pipeline::new(sender, session_id);
        self.pipelines.insert(session_id, pipeline);
        let session = Session::new(session_id, stream, self.session_sender.clone(), receiver);
        tokio::spawn(session.run_loop());

        if let Err(err) = self
            .dispatcher_sender
            .send(ListenerToDispatcherCmd::SessionAdded(self.id))
            .await
        {
            log::error!("Failed to send NewSession cmd: {:?}", err);
        }
    }

    async fn handle_session_cmd(&mut self, cmd: SessionToListenerCmd) -> Result<(), Error> {
        log::info!("Listener::handle_session_cmd()");
        match cmd {
            SessionToListenerCmd::Connect(session_id, packet) => {
                self.on_session_connect(session_id, packet).await
            }
            SessionToListenerCmd::Publish(packet) => self.on_session_publish(packet).await,
            SessionToListenerCmd::Subscribe(session_id, packet) => {
                self.on_session_subscribe(session_id, packet).await
            }
            SessionToListenerCmd::Unsubscribe(session_id, packet) => {
                self.on_session_unsubscribe(session_id, packet).await
            }
            SessionToListenerCmd::Disconnect(session_id) => {
                self.on_session_disconnect(session_id).await
            }
        }
    }

    fn next_session_id(&mut self) -> SessionId {
        self.current_session_id += 1;
        self.current_session_id
    }

    async fn on_session_connect(
        &mut self,
        session_id: SessionId,
        packet: ConnectPacket,
    ) -> Result<(), Error> {
        log::info!("Listener::on_session_connect()");
        // If client id already exists, notify session to send disconnect packet.
        if self.client_ids.get(packet.client_id()).is_some() {
            let ack_packet = ConnectAckPacket::new(false, ConnectReturnCode::IdentifierRejected);
            let cmd = ListenerToSessionCmd::ConnectAck(ack_packet);
            if let Some(pipeline) = self.pipelines.get(&session_id) {
                return pipeline.sender.send(cmd).await.map_err(Into::into);
            } else {
                return Err(Error::session_error(session_id));
            }
        }

        self.session_ids
            .insert(session_id, packet.client_id().to_string());

        // Send request to auth app.
        self.auth_sender
            .send(ListenerToAuthCmd::RequestAuth(
                self.id,
                session_id,
                packet.username().to_string(),
                packet.password().to_vec(),
            ))
            .await
            .map_err(Into::into)
    }

    async fn on_session_disconnect(&mut self, session_id: SessionId) -> Result<(), Error> {
        log::info!("Listener::on_session_disconnect()");
        // Delete session info
        if self.pipelines.remove(&session_id).is_none() {
            log::error!("Failed to remove pipeline with session id: {}", session_id);
        }
        if let Some(client_id) = self.session_ids.remove(&session_id) {
            if self.client_ids.remove(&client_id).is_none() {
                log::error!("Failed to remove client id: {}", client_id);
            }
        } else {
            log::error!("Failed to remove session id: {}", session_id);
        }

        self.dispatcher_sender
            .send(ListenerToDispatcherCmd::SessionRemoved(self.id))
            .await
            .map_err(Into::into)
    }

    async fn on_session_subscribe(
        &mut self,
        session_id: SessionId,
        packet: SubscribePacket,
    ) -> Result<(), Error> {
        log::info!("Listener::on_session_subscribe()");

        // TODO(Shaohua): Check acl.

        let packet_id = packet.packet_id();
        if let Some(pipeline) = self.pipelines.get_mut(&session_id) {
            let mut ack_vec = vec![];
            for topic in packet.mut_topics() {
                // Update sub tree
                ack_vec.push(SubscribeAck::QoS(topic.qos()));
                pipeline.topics.push(topic);
            }

            // Send subscribe ack to session.
            let ack_packet = SubscribeAckPacket::with_vec(ack_vec, packet_id);
            pipeline
                .sender
                .send(ListenerToSessionCmd::SubscribeAck(ack_packet))
                .await?;
        } else {
            return Err(Error::session_error(session_id));
        }

        // TODO(Shaohua): Send notify to dispatcher.
        Ok(())
    }

    async fn on_session_unsubscribe(
        &mut self,
        session_id: SessionId,
        packet: UnsubscribePacket,
    ) -> Result<(), Error> {
        // Remove topic from sub tree.
        for (_, pipeline) in self.pipelines.iter_mut() {
            if pipeline.session_id == session_id {
                pipeline
                    .topics
                    .retain(|ref topic| !packet.topics().any(|t| t == topic.topic().topic()));
            }
            break;
        }

        // Send subRemoved to dispatcher.
        self.dispatcher_sender
            .send(ListenerToDispatcherCmd::SubscriptionsRemoved(self.id))
            .await
            .map_err(Into::into)
    }

    async fn on_session_publish(&mut self, packet: PublishPacket) -> Result<(), Error> {
        let cmd = ListenerToDispatcherCmd::Publish(packet.clone());
        self.dispatcher_sender.send(cmd).await.map_err(Into::into)
    }

    async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToListenerCmd) {
        match cmd {
            DispatcherToListenerCmd::Publish(packet) => self.on_dispatcher_publish(packet).await,
        }
    }

    async fn on_dispatcher_publish(&mut self, packet: PublishPacket) {
        let cmd = ListenerToSessionCmd::Publish(packet.clone());
        // TODO(Shaohua): Replace with a trie tree and a hash table.

        // TODO(Shaohua): Handle errors
        for (_, pipeline) in self.pipelines.iter_mut() {
            if topic_match(&pipeline.topics, packet.topic()) {
                if let Err(err) = pipeline.sender.send(cmd.clone()).await {
                    log::warn!(
                        "Failed to send publish packet from listener to session: {:?}",
                        err
                    );
                }
            }
        }
    }

    async fn handle_auth_cmd(&mut self, cmd: AuthToListenerCmd) -> Result<(), Error> {
        match cmd {
            AuthToListenerCmd::ResponseAuth(session_id, access_granted) => {
                self.on_auth_response(session_id, access_granted).await
            }
        }
    }

    async fn on_auth_response(
        &mut self,
        session_id: SessionId,
        access_granted: bool,
    ) -> Result<(), Error> {
        let ack_packet = if access_granted {
            ConnectAckPacket::new(true, ConnectReturnCode::Accepted)
        } else {
            ConnectAckPacket::new(false, ConnectReturnCode::Unauthorized)
        };
        let cmd = ListenerToSessionCmd::ConnectAck(ack_packet);

        if access_granted {
            // Add client id to cache.
            if let Some(client_id) = self.session_ids.get(&session_id) {
                self.client_ids.insert(client_id.to_string(), session_id);
            } else {
                log::error!(
                    "listener: Failed to find client id with session: {}",
                    session_id
                );
            }
        }

        if let Some(pipeline) = self.pipelines.get(&session_id) {
            pipeline.sender.send(cmd).await.map_err(Into::into)
        } else {
            Err(Error::session_error(session_id))
        }
    }

    /// Acl cmd handler.
    async fn handle_acl_cmd(&mut self, cmd: AclToListenerCmd) -> Result<(), Error> {
        log::info!("Handle acl cmd: {:?}", cmd);
        Ok(())
    }
}
