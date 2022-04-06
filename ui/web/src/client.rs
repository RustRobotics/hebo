pub mod hello_world {
    tonic::include_proto!("hello_world");
}

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

use grpc_web_client::Client;

pub async fn say_hello() {
    let conn = Client::new("/grpc".to_string());
    let mut client = GreeterClient::new(conn);

    let request = tonic::Request::new(HelloRequest {
        name: "WebTonic".into(),
    });

    let response = client.say_hello(request).await.unwrap().into_inner();
    assert_eq!(response.message, "Hello WebTonic!");
}
