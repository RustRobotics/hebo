// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use rustls::Session;

struct DummyClientAuth {
    mandatory: bool,
}

impl rustls::ClientCertVerifier for DummyClientAuth {
    fn offer_client_auth(&self) -> bool { true }

    fn client_auth_mandatory(&self, _sni: Option<&webpki::DNSName>) -> Option<bool> { Some(self.mandatory) }

    fn client_auth_root_subjects(&self, _sni: Option<&webpki::DNSName>) -> Option<rustls::DistinguishedNames> {
        Some(rustls::DistinguishedNames::new())
    }

    fn verify_client_cert(&self,
                          _certs: &[rustls::Certificate], _sni: Option<&webpki::DNSName>) -> Result<rustls::ClientCertVerified, rustls::TLSError> {
        Ok(rustls::ClientCertVerified::assertion())
    }
}

fn main() {
    let mut config = rustls::ClientConfig::new();
    // config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    let root_ca_file = env::args().nth(1).unwrap();
    let root_ca_fd = File::open(root_ca_file).unwrap();
    let mut root_ca_buf = BufReader::new(root_ca_fd);
    let ret = config.root_store.add_pem_file(&mut root_ca_buf).unwrap();
    println!("ret: {:?}", ret);
    println!("root len: {}", config.root_store.len());

    let rc_config = Arc::new(config);
    let hostname = webpki::DNSNameRef::try_from_ascii_str("example.org").unwrap();
    println!("blog hostname: {:?}", hostname);
    let mut client = rustls::ClientSession::new(&rc_config, hostname);

    client.write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut have_written = false;
    let mut socket = TcpStream::connect("127.0.0.1:8883").unwrap();
    loop {
        if !have_written && client.wants_write() {
            println!("write request!");
            have_written = true;
            client.write_tls(&mut socket).unwrap();
        }
        if client.wants_read() {
            println!("read socket!");
            client.read_tls(&mut socket).unwrap();
            client.process_new_packets().unwrap();

            let mut buffer = Vec::new();
            let n_read = client.read_to_end(&mut buffer).unwrap();
            println!("n_raed: {}", n_read);
            io::stdout().write(&buffer).unwrap();
        }
    }
}