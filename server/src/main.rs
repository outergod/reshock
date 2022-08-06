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

// impl From<action_request::Action> for dyn game::Action {
//     fn from(action: action_request::Action) -> Self {
//         match action {
//             action_request::Action::UpLeft => game::DwimAction::UpLeft,
//             action_request::Action::Up => game::DwimAction::Up,
//             action_request::Action::UpRight => game::DwimAction::UpRight,
//             action_request::Action::Right => game::DwimAction::Right,
//             action_request::Action::DownRight => game::DwimAction::DownRight,
//             action_request::Action::Down => game::DwimAction::Down,
//             action_request::Action::DownLeft => game::DwimAction::DownLeft,
//             action_request::Action::Left => game::DwimAction::Left,
//         }
//     }
// }

// impl From<&game::StateQueryItem<'_>> for api::Entity {
//     fn from(item: &game::StateQueryItem<'_>) -> Self {
//         Self {
//             entity: item.entity.id(),
//             player: item.player.map(|_| api::PlayerComponent {}),
//             wall: item.wall.map(|_| api::WallComponent {}),
//             room: item.room.map(|_| api::RoomComponent {}),
//             door: item.door.map(|door| api::DoorComponent { open: door.open }),
//             renderable: item.renderable.map(|renderable| api::RenderableComponent {
//                 renderable: *renderable as i32,
//             }),
//             ordering: item.ordering.map(|ordering| api::OrderingComponent {
//                 ordering: *ordering as i32,
//             }),
//             position: item.position.map(|position| api::PositionComponent {
//                 x: position.0.x,
//                 y: position.0.y,
//             }),
//             sight: item.sight.map(|sight| api::SightComponent {
//                 seeing: sight.seeing.iter().map(|e| e.id()).collect(),
//             }),
//             memory: item.memory.map(|memory| api::MemoryComponent {
//                 entities: memory.clone().into(),
//             }),
//         }
//     }
// }

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
        let game = self.game.lock().await;
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
