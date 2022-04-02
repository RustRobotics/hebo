// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use crate::commands::DashboardToServerContexCmd;
use tokio::sync::mpsc::Sender;

pub type DashboardSender = Sender<DashboardToServerContexCmd>;
