// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use mqtt::ToNetPacket;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    let addr = "127.0.0.1:1883";
    let mut stream = TcpStream::connect(addr).unwrap();
    let mut connect_packet = mqtt::ConnectPacket::new();
    connect_packet.set_client_id(b"test").unwrap();
    let mut buf = Vec::new();
    connect_packet.to_net(&mut buf).unwrap();
    println!("{:x?}", buf);
    let n_recv = stream.write(&buf).unwrap();
    println!("n_recv: {:?}", n_recv);

    let mut msg_packet = mqtt::PublishMessage::new(b"hello");
    msg_packet.set_message(b"Hello, world").unwrap();
    buf.clear();
    msg_packet.to_net(&mut buf).unwrap();
    println!("buf: {:x?}", buf);
    let n_recv = stream.write(&buf).unwrap();
    println!("n_recv: {:?}", n_recv);
}
