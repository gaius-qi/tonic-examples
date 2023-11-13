use echo::echo_client::EchoClient;
use echo::EchoRequest;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::Request;

pub mod echo {
    tonic::include_proto!("echo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoClient::connect("http://[::1]:65001").await?;

    // let request = tonic::Request::new(EchoRequest {
    // name: "Tonic".into(),
    // });

    // let response = client.unary_echo(request).await?;

    // println!("RESPONSE={:?}", response);

    let (tx, rx) = mpsc::channel(128);

    tx.send(EchoRequest {
        name: "Tonic".into(),
    })
    .await
    .unwrap();

    let in_stream = ReceiverStream::new(rx);
    let response = client.stream_echo(Request::new(in_stream)).await?;

    let mut count = 0;
    let out_stream = response.into_inner().timeout(Duration::from_secs(8));
    tokio::pin!(out_stream);

    while let Some(message) = out_stream.try_next().await? {
        if count == 5 {
            drop(tx);
            return Ok(());
        }

        println!("RESPONSE={:?}", message);
        tx.send(EchoRequest {
            name: "Tonic".into(),
        })
        .await
        .unwrap();

        count += 1;
        tokio::time::sleep(Duration::from_secs(2)).await
    }

    Ok(())
}
