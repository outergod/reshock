use std::sync::Arc;

use api::reshock_server::*;
use api::*;
use env_logger::Env;
use game::Game;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::config::Config;

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

impl From<&game::StateQueryItem<'_>> for api::Entity {
    fn from(item: &game::StateQueryItem<'_>) -> Self {
        Self {
            entity: item.entity.id(),
            player: item.player.map(|_| api::PlayerComponent {}),
            wall: item.wall.map(|_| api::WallComponent {}),
            room: item.room.map(|_| api::RoomComponent {}),
            door: item.door.map(|door| api::DoorComponent { open: door.open }),
            renderable: item.renderable.map(|renderable| api::RenderableComponent {
                renderable: *renderable as i32,
            }),
            obstacle: item.obstacle.map(|obstacle| api::ObstacleComponent {
                obstacle: obstacle.0,
            }),
            ordering: item.ordering.map(|ordering| api::OrderingComponent {
                ordering: *ordering as i32,
            }),
            position: item.position.map(|position| api::PositionComponent {
                x: position.0.x,
                y: position.0.y,
            }),
            sight: item.sight.map(|sight| api::SightComponent {
                kind: sight.kind as i32,
                seeing: sight.seeing.iter().map(|e| e.id()).collect(),
            }),
            memory: item.memory.map(|memory| api::MemoryComponent {
                entities: memory
                    .entities
                    .iter()
                    .map(|(entity, components)| api::RememberedEntity {
                        entity: entity.id(),
                        renderable: Some(api::RenderableComponent {
                            renderable: components.renderable as i32,
                        }),
                        position: Some(api::PositionComponent {
                            x: components.position.0.x,
                            y: components.position.0.y,
                        }),
                        ordering: Some(api::OrderingComponent {
                            ordering: components.ordering as i32,
                        }),
                    })
                    .collect(),
            }),
            opaque: item
                .opaque
                .map(|opaque| api::OpaqueComponent { opaque: opaque.0 }),
            ai: item.ai.map(|ai| api::AiComponent { ai: *ai as i32 }),
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
    async fn dump_state(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<StateDumpResponse>, Status> {
        log::info!("Reshock::dump_state {:?}", request.get_ref());
        let mut game = self.game.lock().await;

        Ok(Response::new(StateDumpResponse {
            entities: game.state().iter().map(|entity| entity.into()).collect(),
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
