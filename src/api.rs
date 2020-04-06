
#[derive(Debug)]
pub struct Client {
}

#[derive(Debug, Default)]
pub struct ConnectOptions {
    pub username: String,
    pub password: String,
}

impl Client {
    pub fn connect(addr, extra_option: &ConnectOptions) -> Result<Client> {
    }

    pub fn publish(&mut self, topic: &str, qos: QosLevel, data: &[u8]) {
    }

    pub fn disconnect(&mut self) {
    }

    pub fn on_connect(&mut self) {
    }

    pub fn on_disconnect(&mut self) {
    }

    pub fn on_message(&mut self) {
    }
}
