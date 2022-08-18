#![feature(downcast_unchecked)]

use std::sync::Arc;

use api::reshock_server::*;
use api::*;
use env_logger::Env;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::config::Config;
use crate::game::Game;

mod config;
mod game;

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
    async fn dump_state(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<StateDumpResponse>, Status> {
        log::info!("Reshock::dump_state {:?}", request.get_ref());
        let mut game = self.game.lock().await;
        let response = game.state().map_err(|_| Status::internal("Bla"))?;

        Ok(Response::new(response))
    }

    async fn process_action(
        &self,
        request: Request<ActionRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        use action_request::{Action, DwimAction};

        log::info!("Reshock::process_action {:?}", request.get_ref());
        let mut game = self.game.lock().await;

        let action = match request.into_inner().action {
            Some(Action::Dwim(direction)) => match DwimAction::from_i32(direction) {
                Some(DwimAction::UpLeft) => game::Action::Dwim(game::DwimAction::UpLeft),
                Some(DwimAction::Up) => game::Action::Dwim(game::DwimAction::Up),
                Some(DwimAction::UpRight) => game::Action::Dwim(game::DwimAction::UpRight),
                Some(DwimAction::Right) => game::Action::Dwim(game::DwimAction::Right),
                Some(DwimAction::DownRight) => game::Action::Dwim(game::DwimAction::DownRight),
                Some(DwimAction::Down) => game::Action::Dwim(game::DwimAction::Down),
                Some(DwimAction::DownLeft) => game::Action::Dwim(game::DwimAction::DownLeft),
                Some(DwimAction::Left) => game::Action::Dwim(game::DwimAction::Left),
                None => return Err(Status::invalid_argument("Dwim index out of bounds")),
            },
            Some(Action::God(_)) => game::Action::GodMode(None),
            None => return Err(Status::invalid_argument("Action not set")),
        };

        let events = game.input(action);

        Ok(Response::new(EventsResponse { events }))
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
