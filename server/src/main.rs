use env_logger::Env;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::api::reshock_server::{Reshock, ReshockServer};
use crate::api::{Dump, Empty};
use crate::config::Config;

mod api {
    tonic::include_proto!("reshock");

    pub(crate) const FILE_DESCRIPTOR_SET: &'static [u8] =
        tonic::include_file_descriptor_set!("reshock_descriptor");
}

mod config;

struct ReshockService;

#[tonic::async_trait]
impl Reshock for ReshockService {
    async fn dump_state(&self, request: Request<Empty>) -> Result<Response<Dump>, Status> {
        log::info!("Reshock::dump_state {:?}", request.get_ref());
        Ok(Response::new(Dump {
            value: "".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config = Config::new()?;

    let address = config.listen_address.parse()?;

    let reflector = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(api::FILE_DESCRIPTOR_SET)
        .build()?;

    log::info!("Running on {}", address);

    Server::builder()
        .add_service(ReshockServer::new(ReshockService))
        .add_service(reflector)
        .serve(address)
        .await?;

    Ok(())
}
