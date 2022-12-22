use tonic::{transport::Server, Request, Response, Status};

use echo::echo_server::{Echo, EchoServer};
use echo::{EchoResponse, EchoRequest};

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
