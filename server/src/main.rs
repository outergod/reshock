use std::sync::Arc;

use env_logger::Env;
use game::Game;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::api::reshock_server::*;
use crate::api::*;
use crate::config::Config;

mod api {
    tonic::include_proto!("reshock");

    pub(crate) const FILE_DESCRIPTOR_SET: &'static [u8] =
        tonic::include_file_descriptor_set!("reshock_descriptor");
}
mod config;
mod game;

impl From<command_request::Command> for game::Command {
    fn from(command: command_request::Command) -> Self {
        match command {
            command_request::Command::UpLeft => game::Command::UpLeft,
            command_request::Command::Up => game::Command::Up,
            command_request::Command::UpRight => game::Command::UpRight,
            command_request::Command::Right => game::Command::Right,
            command_request::Command::DownRight => game::Command::DownRight,
            command_request::Command::Down => game::Command::Down,
            command_request::Command::DownLeft => game::Command::DownLeft,
            command_request::Command::Left => game::Command::Left,
        }
    }
}

struct ReshockService {
    game: Arc<Mutex<Game>>,
}

impl ReshockService {
    fn new(game: Game) -> Self {
        Self {
            game: Arc::new(Mutex::new(game)),
        }
    }
}

#[tonic::async_trait]
impl Reshock for ReshockService {
    async fn dump_state(&self, request: Request<Empty>) -> Result<Response<DumpResponse>, Status> {
        log::info!("Reshock::dump_state {:?}", request.get_ref());
        Ok(Response::new(DumpResponse {
            value: format!("{:#?}", self.game),
        }))
    }

    async fn process_command(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        log::info!("Reshock::process_command {:?}", request.get_ref());
        let mut game = self.game.lock().await;

        let events = game.input(request.into_inner().command().into());

        Ok(Response::new(EventsResponse {
            value: format!("{:?}", events),
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

    let game = Game::default();

    log::info!("Running on {}", address);

    Server::builder()
        .add_service(ReshockServer::new(ReshockService::new(game)))
        .add_service(reflector)
        .serve(address)
        .await?;

    Ok(())
}
