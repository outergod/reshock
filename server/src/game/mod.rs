use std::{collections::VecDeque, fs, path::Path};

use anyhow::Result;
use bevy_ecs::prelude::*;
use bevy_ecs::system::{BoxedSystem, SystemState};
use glam::IVec2;
use thiserror::Error;

mod behavior;
mod bundle;
mod component;
mod effect;
mod resource;

const LEVEL01_PATH: &'static str = "rooms/level01.room";

type BoxedBehavior = Box<dyn System<In = (), Out = Status>>;

type State = SystemState<(
    Query<
        'static,
        'static,
        (
            Entity,
            &'static component::Sight,
            &'static component::Memory,
        ),
        With<component::Player>,
    >,
    Query<
        'static,
        'static,
        (
            Entity,
            &'static component::Position,
            &'static component::Renderable,
            &'static component::Ordering,
            Option<&'static component::Door>,
        ),
    >,
)>;

pub struct Game {
    world: World,
    behaviors: Vec<BoxedBehavior>,
    effects: Vec<BoxedSystem>,
    state: State,
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::new();

        let room: behavior::Room = fs::read_to_string(Path::new("assets").join(LEVEL01_PATH))
            .unwrap()
            .into();

        behavior::room(&mut world, room);
        behavior::radial_lines(&mut world);

        world.init_resource::<ActiveAction>();
        world.init_resource::<Reactions>();
        world.init_resource::<FollowUps>();

        let mut behaviors = vec![
            Box::new(IntoSystem::into_system(behavior::dwim)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::r#move)) as BoxedBehavior,
            Box::new(IntoSystem::into_system(behavior::door)) as BoxedBehavior,
        ];
        for behavior in behaviors.iter_mut() {
            (*behavior).initialize(&mut world);
        }

        let mut effects = vec![
            Box::new(IntoSystem::into_system(effect::r#move)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::door)) as BoxedSystem,
            Box::new(IntoSystem::into_system(effect::sight)) as BoxedSystem,
        ];
        for effect in effects.iter_mut() {
            (*effect).initialize(&mut world);
            (*effect).run((), &mut world);
            (*effect).apply_buffers(&mut world);
        }

        let state = State::new(&mut world);

        Self {
            world,
            behaviors,
            effects,
            state,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    EndTurn,
    Dwim(DwimAction),
    Move(MoveAction),
    OpenDoor(OpenDoorAction),
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
}

#[derive(Debug, Clone)]
pub struct MoveAction {
    entity: Entity,
    position: IVec2,
}

#[derive(Debug, Clone)]
pub struct OpenDoorAction {
    pub actor: Entity,
    pub entity: Entity,
}

#[derive(Default)]
pub struct ActiveAction(pub Option<Action>);
#[derive(Default)]
pub struct Reactions(pub Vec<Action>);
#[derive(Default)]
pub struct FollowUps(pub Vec<Action>);

pub enum Status {
    Accept,
    Continue,
    Reject,
}

#[derive(Debug, Error)]
#[error("No player found")]
pub struct NoPlayer;

impl Game {
    pub fn input(&mut self, action: Action) -> Vec<api::Event> {
        let mut actions = VecDeque::from([action]);
        let events = Vec::new();

        loop {
            log::debug!("Current action queue is {:?}", actions);

            let action = match actions.pop_front() {
                Some(it) => it,
                None => break,
            };

            self.world.resource_mut::<ActiveAction>().0 = Some(action.clone());

            let mut accepted = false;

            for behavior in &mut self.behaviors {
                match behavior.run((), &mut self.world) {
                    Status::Accept => {
                        accepted = true;
                        break;
                    }
                    Status::Continue => {}
                    Status::Reject => break,
                }
            }

            if !accepted {
                log::debug!("Action {:?} rejected", action);
                self.world.resource_mut::<Reactions>().0.clear();
                self.world.resource_mut::<FollowUps>().0.clear();
                continue;
            }

            log::debug!("Action {:?} accepted", action);

            for effect in &mut self.effects {
                effect.run((), &mut self.world);
                effect.apply_buffers(&mut self.world);
            }

            for action in self.world.resource_mut::<Reactions>().0.drain(..) {
                log::debug!("Queueing reaction {:?}", action);
                actions.push_front(action);
            }
            for action in self.world.resource_mut::<FollowUps>().0.drain(..) {
                log::debug!("Queueing followup {:?}", action);
                actions.push_back(action);
            }
        }

        events
    }

    pub fn state(&mut self) -> Result<api::StateDumpResponse> {
        let (player, entities) = self.state.get(&self.world);

        let (player, sight, memory) = player.get_single().map_err(|_| NoPlayer)?;

        let view = {
            let entities = entities
                .iter()
                .filter_map(|(entity, position, renderable, ordering, door)| {
                    sight.seeing.contains(&entity).then_some((
                        entity.id(),
                        api::Components {
                            position: Some(position.into()),
                            renderable: Some(renderable.into()),
                            ordering: Some(ordering.into()),
                            door: door.map(|it| it.into()),
                        },
                    ))
                })
                .collect();

            api::State { entities }
        };

        let memory = api::State {
            entities: memory.0.iter().map(|(e, cs)| (e.id(), cs.into())).collect(),
        };

        Ok(api::StateDumpResponse {
            player: player.id(),
            view: Some(view),
            memory: Some(memory),
        })
    }
}
