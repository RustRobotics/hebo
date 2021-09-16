// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Acl cmd handler.

use super::Listener;
use crate::commands::AclToListenerCmd;
use crate::error::Error;

impl Listener {
    pub(super) async fn handle_acl_cmd(&mut self, cmd: AclToListenerCmd) -> Result<(), Error> {
        log::info!("Handle acl cmd: {:?}", cmd);
        Ok(())
    }
}
