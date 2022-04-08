use crate::protos::hello_world::greeter_client::GreeterClient;
use crate::protos::hello_world::HelloRequest;

use grpc_web_client::Client;

pub async fn say_hello() {
    let conn = Client::new("http://localhost:50051".to_string());
    let mut client = GreeterClient::new(conn);

    let request = tonic::Request::new(HelloRequest {
        name: "WebTonic".into(),
    });

    let response = client.say_hello(request).await.unwrap().into_inner();
    log::info!("response message: {:?}", response.message);
}
