use std::fs;
use std::path::Path;

use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;

mod bundle;
mod component;
mod room;

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

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game").field("world", &self.world).finish()
    }
}

#[derive(StageLabel)]
enum Stage {
    Core,
}

#[derive(Debug, Clone)]
pub enum Command {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

#[derive(Default)]
struct GameEvents(Vec<Command>);

fn test(mut commands: EventReader<Command>, mut events: ResMut<GameEvents>) {
    events.0.append(&mut commands.iter().cloned().collect());
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::new();

        world.insert_resource(GameEvents::default());
        world.init_resource::<Events<Command>>();

        let room = fs::read_to_string(Path::new("assets").join(LEVEL01_PATH)).unwrap();
        room::setup(&mut world, room.into());

        let mut schedule = Schedule::default();

        schedule.add_stage(
            Stage::Core,
            SystemStage::single_threaded().with_system(test),
        );
        schedule.add_system_to_stage(Stage::Core, Events::<Command>::update_system);

        let state = world.query::<StateQuery>();

        Self {
            world,
            schedule,
            state,
        }
    }
}

impl Game {
    pub fn input(&mut self, command: Command) -> Vec<Command> {
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
