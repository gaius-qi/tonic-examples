use echo::echo_client::EchoClient;
use echo::EchoRequest;

pub mod echo {
    tonic::include_proto!("echo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoClient::connect("http://[::1]:65001").await?;

    let request = tonic::Request::new(EchoRequest {
        name: "Tonic".into(),
    });

    let response = client.unary_echo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
