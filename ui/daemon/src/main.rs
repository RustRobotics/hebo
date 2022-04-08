use tonic::{transport::Server, Request, Response, Status};

use protos::hello_world::greeter_server::{Greeter, GreeterServer};
use protos::hello_world::{HelloReply, HelloRequest};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

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
