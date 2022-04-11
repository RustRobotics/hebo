// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{html, Component, Context, Html};

pub enum NewConnectionMsg {}

pub struct NewConnectionComponent {}

impl Component for NewConnectionComponent {
    type Message = NewConnectionMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"NewConnection"}</h1>
            </div>
        }
    }
}
