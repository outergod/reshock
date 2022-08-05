use std::fs;
use std::path::Path;

use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use bevy_ecs::schedule::ShouldRun;

pub use event::Command;
use system::*;

mod bundle;
mod component;
mod event;
mod resource;
mod system {
    pub mod command;
    pub mod door;
    pub mod radial_lines;
    pub mod room;
    pub mod sight;
}

const LEVEL01_PATH: &'static str = "rooms/level01.room";

#[derive(WorldQuery)]
pub struct StateQuery {
    pub entity: Entity,
    pub player: Option<&'static component::Player>,
    pub wall: Option<&'static component::Wall>,
    pub room: Option<&'static component::Room>,
    pub door: Option<&'static component::Door>,
    pub renderable: Option<&'static component::Renderable>,
    pub obstacle: Option<&'static component::Obstacle>,
    pub ordering: Option<&'static component::Ordering>,
    pub position: Option<&'static component::Position>,
    pub sight: Option<&'static component::Sight>,
    pub memory: Option<&'static component::Memory>,
    pub opaque: Option<&'static component::Opaque>,
    pub ai: Option<&'static component::AI>,
}

pub struct Game {
    world: World,
    schedule: Schedule,
    state: QueryState<StateQuery>,
}

#[derive(Default)]
pub struct GameEvents(Vec<api::Event>);

#[derive(StageLabel)]
enum Stage {
    Init,
    Cleanup,
    Event,
    Core,
}

#[derive(StageLabel)]
struct InitStage;

fn cleanup(mut events: ResMut<GameEvents>) {
    log::debug!("cleanup {:?}", events.0);
    events.0.clear();
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::new();

        let room: room::Room = fs::read_to_string(Path::new("assets").join(LEVEL01_PATH))
            .unwrap()
            .into();

        world.insert_resource(GameEvents::default());
        world.init_resource::<Events<event::Command>>();
        world.init_resource::<Events<event::ToggleDoor>>();
        world.insert_resource(room);

        let mut schedule = Schedule::default()
            .with_stage(
                Stage::Init,
                Schedule::default()
                    .with_run_criteria(ShouldRun::once)
                    .with_stage(InitStage, SystemStage::parallel())
                    .with_system_in_stage(InitStage, room::setup)
                    .with_system_in_stage(InitStage, radial_lines::setup),
            )
            .with_stage(Stage::Cleanup, SystemStage::single_threaded())
            .with_system_in_stage(Stage::Cleanup, cleanup)
            .with_stage(Stage::Event, SystemStage::parallel())
            .with_system_in_stage(Stage::Event, Events::<event::Command>::update_system)
            .with_system_in_stage(Stage::Event, Events::<event::ToggleDoor>::update_system)
            .with_stage(Stage::Core, SystemStage::single_threaded())
            .with_system_in_stage(Stage::Core, command::system)
            .with_system_in_stage(Stage::Core, sight::system)
            .with_system_in_stage(Stage::Core, door::toggle)
            .with_system_in_stage(Stage::Core, door::open);

        schedule.run_once(&mut world);

        let state = world.query::<StateQuery>();

        Self {
            world,
            schedule,
            state,
        }
    }
}

impl Game {
    pub fn input(&mut self, command: Command) -> Vec<api::Event> {
        self.world.send_event(command);
        self.schedule.run_once(&mut self.world);
        self.world.get_resource::<GameEvents>().unwrap().0.clone()
    }

    pub fn state(&mut self) -> Vec<StateQueryItem> {
        self.state
            .iter(&mut self.world)
            .map(|item| item as StateQueryItem)
            .collect()
    }
}
