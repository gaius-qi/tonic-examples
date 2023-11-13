use std::pin::Pin;
use std::time::Duration;
use tokio_stream::Stream;
use tonic::{transport::Server, Request, Response, Status};

use echo::echo_server::{Echo, EchoServer};
use echo::{EchoRequest, EchoResponse};

pub mod echo {
    tonic::include_proto!("echo");
}

#[derive(Default)]
pub struct MyEcho {}

#[tonic::async_trait]
impl Echo for MyEcho {
    async fn unary_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = echo::EchoResponse {
            message: format!("Echo {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    type StreamEchoStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send>>;

    async fn stream_echo(
        &self,
        request: Request<tonic::Streaming<EchoRequest>>,
    ) -> Result<Response<Self::StreamEchoStream>, Status> {
        println!("Got first request from {:?}", request.remote_addr());

        let mut in_stream = request.into_inner();
        let output_stream = async_stream::try_stream! {
            while let Some(req) = in_stream.message().await? {
                tokio::time::sleep(Duration::from_secs(4)).await;
                println!("Got a request from {:?}", req.name);
                yield EchoResponse {
                    message: format!("Echo {}!", req.name),
                };
            }
        };

        Ok(Response::new(
            Box::pin(output_stream) as Self::StreamEchoStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:65001".parse().unwrap();
    let echo = MyEcho::default();

    println!("EchoServer listening on {}", addr);

    Server::builder()
        .add_service(EchoServer::new(echo))
        .serve(addr)
        .await?;

    Ok(())
}
