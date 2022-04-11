// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use hebo_web::app::AppModel;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<AppModel>();
}
