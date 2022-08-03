use bevy_ecs::prelude::*;

pub struct Game {
    world: World,
    schedule: Schedule,
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game").field("world", &self.world).finish()
    }
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

        let mut schedule = Schedule::default();

        schedule.add_stage("core", SystemStage::single_threaded().with_system(test));
        schedule.add_system_to_stage("core", Events::<Command>::update_system);

        Self { world, schedule }
    }
}

impl Game {
    pub fn input(&mut self, command: Command) -> Vec<Command> {
        self.world.send_event(command);
        self.schedule.run(&mut self.world);
        self.world.get_resource::<GameEvents>().unwrap().0.clone()
    }
}
