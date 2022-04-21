// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use yew::prelude::*;

pub enum NewConnectionMsg {
    RefreshClientId,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ProtocolType {
    Mqtt,
    SecureMqtt,
    WebSocket,
    SecureWebSocket,
    Quic,
}

impl ToString for ProtocolType {
    fn to_string(&self) -> String {
        match self {
            ProtocolType::Mqtt => "mqtt".to_string(),
            ProtocolType::SecureMqtt => "mqtts".to_string(),
            ProtocolType::WebSocket => "ws".to_string(),
            ProtocolType::SecureWebSocket => "wss".to_string(),
            ProtocolType::Quic => "quic".to_string(),
        }
    }
}

pub struct NewConnectionComponent {
    name: String,
    client_id: String,
    protocol: ProtocolType,
    host: String,
    port: u16,
    username: String,
    password: String,
    with_ssl: bool,
}

impl NewConnectionComponent {
    pub fn random_client_id() -> String {
        let rng = rand::thread_rng();
        let s = String::from_utf8(rng.sample_iter(&Alphanumeric).take(8).collect::<Vec<u8>>())
            .expect("Invalid random string");

        format!("hebo_{}", s.to_lowercase())
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
            host: "localhost".to_string(),
            port: 1883,
            username: String::new(),
            password: String::new(),
            with_ssl: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NewConnectionMsg::RefreshClientId => {
                self.client_id = Self::random_client_id();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div>
                <h1>{ "NewConnection" }</h1>

                <div>
                    <h2>{ "General" }</h2>
                    <div>
                        <label>{ "Name" }</label>
                        <input type="text" value={ self.name.clone() } />
                    </div>
                    <div>
                        <label>{ "Client Id" }</label>
                        <input type="text" value={ self.client_id.clone() } />
                        <button type="button"
                            onclick={ link.callback(|_event| NewConnectionMsg::RefreshClientId) }
                        >
                            { "Refresh" }
                        </button>
                    </div>
                    <div>
                        <select value={ self.protocol.to_string() }>
                            <option value={ "mqtt" }>{ "mqtt" }</option>
                            <option value={ "mqtts" }>{ "mqtts" }</option>
                            <option value={ "ws" }>{ "ws" }</option>
                            <option value={ "wss" }>{ "wss" }</option>
                            <option value={ "quic" }>{ "quic" }</option>
                        </select>
                        <label>{"Host"}</label>
                        <input type="text" value={self.host.clone()} />
                    </div>
                    <div>
                        <label>{"Port"}</label>
                        <input type="number" value={self.port.to_string()} />
                    </div>
                    <div>
                        <label>{"Username"}</label>
                        <input type="text" value={self.username.clone()} />
                    </div>
                    <div>
                        <label>{"Password"}</label>
                        <input type="text" value={self.password.clone()} />
                    </div>
                    <div>
                        <label>{"SSL/TLS"}</label>
                        <input type="checkbox" checked={self.with_ssl} />
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
