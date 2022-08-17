use pb::echo_service_server::{EchoService, EchoServiceServer};
use tonic::{transport::Server, Response, Status};
mod pb {
    include!("pb.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("echo_descriptor");
}

#[derive(Default)]
pub struct EchoServiceImpl {}

#[tonic::async_trait]
impl EchoService for EchoServiceImpl {
    async fn echo(
        &self,
        request: tonic::Request<pb::Request>,
    ) -> Result<Response<pb::Response>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let response = pb::Response {
            msg: request.get_ref().msg.to_owned(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let echo = EchoServiceImpl::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("EchoService server listening on {}", addr);

    Server::builder()
        .add_service(EchoServiceServer::new(echo))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
