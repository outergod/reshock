use std::fmt::Display;
use std::time::Instant;
use std::{collections::VecDeque, fs, path::Path};

use anyhow::Result;
use bevy_ecs::prelude::*;
use bevy_ecs::system::BoxedSystem;
use glam::IVec2;
use itertools::Itertools;
use rand::prelude::*;
use thiserror::Error;

mod behavior;
mod bundle;
mod component;
mod effect;
mod pathfinding;
mod resource;

const LEVEL01_PATH: &'static str = "rooms/level01.room";

type BoxedBehavior = BoxedSystem<(), Status>;

pub struct Game {
    world: World,
    behaviors: Vec<BoxedBehavior>,
    effects: Vec<BoxedSystem>,
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::new();

        let room: resource::Room = fs::read_to_string(Path::new("assets").join(LEVEL01_PATH))
            .unwrap()
            .into();

        behavior::room(&mut world, room);
        behavior::radial_lines(&mut world);

        world.init_resource::<ActiveAction>();
        world.init_resource::<Reactions>();
        world.init_resource::<FollowUps>();
        world.init_resource::<Events>();
        world.init_resource::<resource::Deltas>();
        world.init_resource::<resource::SpatialHash>();
        world.init_resource::<resource::Log>();
        world.init_resource::<api::State>();

        let mut behaviors = vec![
            Box::new(IntoSystem::into_system(behavior::dwim_move)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::dwim_close)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::dwim_shoot)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::ai)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::god_mode)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::r#move)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::door)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::view)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::view_all)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::spot)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::memorize)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::melee_intent)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::melee_attack)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::shoot_intent)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::shoot_projectile)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::dispatch_projectile)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::combat_damage)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::combat_hit)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::death)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::state)) as BoxedBehavior,
        ];
        for behavior in behaviors.iter_mut() {
            (*behavior).initialize(&mut world);
        }

        let mut effects = vec![
            Box::new(IntoSystem::into_system(effect::r#move)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::god_mode)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::door_open)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::door_close)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::melee)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::shoot)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::health)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::death)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::render)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::spatial)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::view)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::spot)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::memorize)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::state)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::log)) as BoxedSystem,
        ];
        for effect in effects.iter_mut() {
            (*effect).initialize(&mut world);
            (*effect).run((), &mut world);
            (*effect).apply_buffers(&mut world);
        }

        let mut game = Self {
            world,
            behaviors,
            effects,
        };

        game.input(Action::View(None));

        game
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Dwim(DwimAction),
    AI(Entity),
    EndTurn(Entity),
    GodMode(Option<GodModeAction>),
    Move(MoveAction),
    OpenDoor(OpenDoorAction),
    CloseDoor(CloseDoorAction),
    View(Option<ViewAction>),
    Memorize(MemorizeAction),
    Spot(SpotAction),
    Log(String),
    Melee(MeleeAttackAction),
    Shoot(ShootAction),
    Hit(HitAction),
    Damage(DamageAction),
    HealthLoss(HealthLossAction),
    Death(DeathAction),
    State(Option<api::State>),
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Action::Dwim(_) => "Dwim",
            Action::AI(_) => "AI",
            Action::EndTurn(_) => "EndTurn",
            Action::GodMode(_) => "GodMode",
            Action::Move(_) => "Move",
            Action::OpenDoor(_) => "OpenDoor",
            Action::CloseDoor(_) => "CloseDoor",
            Action::View(_) => "View",
            Action::Memorize(_) => "Memorize",
            Action::Spot(_) => "Spot",
            Action::Log(_) => "Log",
            Action::Melee(_) => "Melee",
            Action::Shoot(_) => "Shoot",
            Action::Hit(_) => "Hit",
            Action::Damage(_) => "Damage",
            Action::HealthLoss(_) => "HealthLoss",
            Action::Death(_) => "Death",
            Action::State(_) => "State",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct ViewAction {
    actor: Entity,
    sight: component::Sight,
}

#[derive(Debug, Clone)]
pub struct MemorizeAction {
    actor: Entity,
    memory: component::Memory,
}

#[derive(Debug, Clone)]
pub struct DeathAction {
    actor: Entity,
    kind: Option<component::Alive>,
}

#[derive(Debug, Clone, Copy)]
pub struct HealthLossAction {
    actor: Entity,
    amount: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct HitAction {
    actor: Entity,
    target: Entity,
    weapon: Entity,
    damage: component::Damage,
}

#[derive(Debug, Clone, Copy)]
pub struct DamageAction {
    actor: Entity,
    target: Entity,
    weapon: Entity,
    damage: component::Damage,
}

#[derive(Debug, Clone, Copy)]
pub enum MeleeAttackAction {
    Intent {
        actor: Entity,
        target: Entity,
    },
    Attack {
        actor: Entity,
        target: Entity,
        weapon: Entity,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ShootAction {
    Intent {
        actor: Entity,
        target: Entity,
    },
    ProjectileGun {
        actor: Entity,
        target: Entity,
        weapon: Entity,
    },
    DispatchProjectile {
        actor: Entity,
        target: Entity,
        weapon: Entity,
        magazine: Entity,
    },
}

#[derive(Debug, Clone)]
pub enum DwimAction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    Close,
    Shoot,
}

#[derive(Debug, Clone, Copy)]
pub struct GodModeAction {
    actor: Entity,
    activate: bool,
}

#[derive(Debug, Clone)]
pub struct MoveAction {
    actor: Entity,
    position: IVec2,
}

#[derive(Debug, Clone)]
pub struct OpenDoorAction {
    actor: Entity,
    target: Entity,
}

#[derive(Debug, Clone)]
pub struct CloseDoorAction {
    actor: Entity,
    target: Entity,
}

#[derive(Debug, Clone)]
pub struct SpotAction {
    actor: Entity,
    sound: api::spot_event::SpotSound,
}

#[derive(Default)]
pub struct ActiveAction(pub Option<Action>);
#[derive(Default)]
pub struct Reactions(pub Vec<Action>);
#[derive(Default)]
pub struct FollowUps(pub Vec<Action>);
#[derive(Default)]
pub struct Events(pub Vec<api::Event>);

pub enum Status {
    Continue,
    Reject(Option<Action>),
}

#[derive(Debug, Error)]
#[error("No player found")]
pub struct NoPlayer;

impl Game {
    pub fn input(&mut self, action: Action) -> Vec<api::Event> {
        let mut actions = VecDeque::from([action]);
        let mut events = Vec::new();

        let now = Instant::now();

        loop {
            log::debug!(
                "Current action queue is [{}]",
                actions.iter().map(|a| a.to_string()).join(" -> ")
            );

            let action = match actions.pop_front() {
                Some(it) => it,
                None => break,
            };

            self.world.resource_mut::<ActiveAction>().0 = Some(action.clone());

            let mut accepted = true;

            for behavior in &mut self.behaviors {
                match behavior.run((), &mut self.world) {
                    Status::Continue => {}
                    Status::Reject(action) => {
                        accepted = false;

                        if let Some(action) = action {
                            log::debug!("Queueing reject followup {:?}", action);
                            actions.push_back(action);
                        }
                        break;
                    }
                }
            }

            let action = self
                .world
                .resource::<ActiveAction>()
                .0
                .as_ref()
                .expect("Action is not None");

            if !accepted {
                log::debug!("Action {} rejected", action);
                self.world.resource_mut::<Reactions>().0.clear();
                self.world.resource_mut::<FollowUps>().0.clear();
                continue;
            }

            log::debug!("Action {} accepted", action);

            for effect in &mut self.effects {
                effect.run((), &mut self.world);
                effect.apply_buffers(&mut self.world);
            }

            for action in self.world.resource_mut::<Reactions>().0.drain(..).rev() {
                log::debug!("Queueing reaction {}", action);
                actions.push_front(action);
            }
            for action in self.world.resource_mut::<FollowUps>().0.drain(..) {
                log::debug!("Queueing followup {}", action);
                actions.push_back(action);
            }
            for event in self.world.resource_mut::<Events>().0.drain(..) {
                log::debug!("Queueing event {}", event);
                events.push(event);
            }
        }

        let duration = Instant::now() - now;
        log::debug!("Time taken: {}µs", duration.as_micros());

        // Deduplicate state events, last one wins; TODO move to impl
        events.into_iter().fold(vec![], |mut acc, ev| {
            match (acc.last(), &ev) {
                (
                    Some(api::Event {
                        event: Some(api::event::Event::State(_)),
                    }),
                    api::Event {
                        event: Some(api::event::Event::State(_)),
                    },
                ) => *acc.last_mut().unwrap() = ev,
                _ => acc.push(ev),
            }
            acc
        })
    }

    pub fn state(&mut self) -> Result<api::StateDumpResponse> {
        let state = self.world.resource::<api::State>().clone();
        let player = self
            .world
            .query_filtered::<Entity, With<component::Player>>()
            .get_single(&self.world)
            .unwrap();

        let (x, y) = self
            .world
            .query_filtered::<&component::Position, With<component::Renderable>>()
            .iter(&self.world)
            .fold(
                (0, 0),
                |(max_x, max_y), component::Position(IVec2 { x, y })| {
                    (max_x.max(*x), max_y.max(*y))
                },
            );

        let dimensions = api::Dimensions { x, y };

        let log = self
            .world
            .resource::<resource::Log>()
            .read()
            .cloned()
            .collect();

        Ok(api::StateDumpResponse {
            player: player.id(),
            dimensions: Some(dimensions),
            state: Some(state),
            log: Some(api::Log { entries: log }),
        })
    }
}
