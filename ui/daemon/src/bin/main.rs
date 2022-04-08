// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use tonic::transport::Server;

use hebo_ui_daemon::greeter::MyGreeter;
use protos::hello_world::greeter_server::GreeterServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);
    let config = tonic_web::config().allow_all_origins();

    Server::builder()
        .accept_http1(true)
        .add_service(config.enable(GreeterServer::new(greeter)))
        .serve(addr)
        .await?;

    Ok(())
}
