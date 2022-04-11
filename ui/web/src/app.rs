// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::{html, Component, Context, Html};

use crate::client;

pub enum AppMsg {
    SayHello,
    SayHelloReturns(bool),
}

pub struct AppModel {}

impl Component for AppModel {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::SayHello => {
                ctx.link().send_future(async move {
                    client::say_hello().await;
                    AppMsg::SayHelloReturns(true)
                });
                true
            }
            AppMsg::SayHelloReturns(state) => {
                log::info!("resp state: {}", state);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| AppMsg::SayHello)}>{ "Hello" }</button>
            </div>
        }
    }
}
