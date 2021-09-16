// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::collections::{BTreeMap, HashMap};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    AclToListenerCmd, AuthToListenerCmd, DispatcherToListenerCmd, ListenerToAclCmd,
    ListenerToAuthCmd, ListenerToDispatcherCmd, SessionToListenerCmd,
};
use crate::config;
use crate::types::{ListenerId, SessionId};

mod handlers;
mod init;
mod pipeline;
mod protocol;

use pipeline::Pipeline;
use protocol::Protocol;

const CHANNEL_CAPACITY: usize = 16;

#[derive(Debug)]
pub struct Listener {
    id: ListenerId,
    protocol: Protocol,
    listener_config: config::Listener,
    current_session_id: SessionId,

    // TODO(Shaohua): Move subscription to dispatcher layer to support global
    // subscriptions and clean sessions.
    pipelines: HashMap<SessionId, Pipeline>,
    session_ids: HashMap<SessionId, String>,
    client_ids: BTreeMap<String, SessionId>,

    session_sender: Sender<SessionToListenerCmd>,
    session_receiver: Option<Receiver<SessionToListenerCmd>>,

    dispatcher_sender: Sender<ListenerToDispatcherCmd>,
    dispatcher_receiver: Option<Receiver<DispatcherToListenerCmd>>,

    auth_sender: Sender<ListenerToAuthCmd>,
    auth_receiver: Option<Receiver<AuthToListenerCmd>>,

    acl_sender: Sender<ListenerToAclCmd>,
    acl_receiver: Option<Receiver<AclToListenerCmd>>,
}
