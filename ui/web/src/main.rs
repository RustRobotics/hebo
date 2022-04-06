// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use yew::prelude::{html, Component, Context, Html};

mod client;

enum Msg {
    AddOne,
    SayHello,
    SayHelloReturns(bool),
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::SayHello => {
                ctx.link().send_future(async move {
                    client::say_hello().await;
                    Msg::SayHelloReturns(true)
                });
                true
            }
            Msg::SayHelloReturns(state) => {
                log::info!("resp state: {}", state);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <div>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                    <p>{ self.value }</p>
                </div>

                <div>
                    <button onclick={link.callback(|_| Msg::SayHello)}>{ "Hello" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
