// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use yew::{html, Component, Context, Html};

pub enum NewConnectionMsg {}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ProtocolType {
    Mqtt,
    SecureMqtt,
    WebSocket,
    SecureWebSocket,
}

pub struct NewConnectionComponent {
    name: String,
    client_id: String,
    protocol: ProtocolType,
    port: u16,
    username: String,
    password: String,
    with_ssl: bool,
}

impl NewConnectionComponent {
    pub fn random_client_id() -> String {
        let mut rng = rand::thread_rng();
        let s = String::from_utf8(rng.sample_iter(&Alphanumeric).take(8).collect::<Vec<u8>>())
            .expect("Invalid random string");

        format!("hebo_{}", s)
    }
}

impl Component for NewConnectionComponent {
    type Message = NewConnectionMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: String::new(),
            client_id: Self::random_client_id(),
            protocol: ProtocolType::Mqtt,
            port: 1883,
            username: String::new(),
            password: String::new(),
            with_ssl: false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"NewConnection"}</h1>

                <div>
                    <h2>{"General"}</h2>
                    <div>
                        <label>{"Name"}</label>
                        <input type="text" />
                    </div>
                </div>

                <div>
                    <h2>{"Advanced"}</h2>
                </div>

                <div>
                    <h2>{"Last Will"}</h2>
                </div>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_client_id() {
        let client_id = NewConnectionComponent::random_client_id();
        assert_eq!(client_id.len(), 13);
    }
}
