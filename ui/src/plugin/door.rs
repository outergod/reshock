use std::time::Duration;

use bevy::{math::ivec2, prelude::*, utils::HashSet};
use bevy_kira_audio::Audio;
use bevy_tweening::*;

use crate::{
    component::*,
    resource::{Deltas, ReshockEvents, TransitionState},
};

const VDOOR: char = '║';
const HDOOR: char = '═';
// const VDOOR: char = '╎';
// const HDOOR: char = '╌';
const EMPTY: char = ' ';

const DOOR_OPEN_SOUND: &'static str = "sshock/sounds/00206.wav";

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(event).add_system(render);
    }
}

pub fn event(
    mut reader: EventReader<api::DoorEvent>,
    doors: Query<(Entity, &ReshockEntity, &Door)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut commands: Commands,
) {
    for api::DoorEvent {
        actor: _,
        door: entity,
        open,
    } in reader.iter()
    {
        if let Some((e, _, door)) = doors.iter().find(|(_, id, _)| entity == &id.0) {
            let color = if *open {
                ColorLens {
                    start: door.close_color,
                    end: door.open_color,
                }
            } else {
                ColorLens {
                    start: door.open_color,
                    end: door.close_color,
                }
            };

            let mut tween = Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_secs_f32(1.5),
                color,
            );

            tween.set_completed_event(0);
            commands.entity(e).insert(Animator::new(tween));
            audio.play(asset_server.load(DOOR_OPEN_SOUND));
        }
    }
}

pub fn render(
    changed: Query<(), Or<(Changed<Door>, Changed<Wall>)>>,
    mut set: ParamSet<(
        Query<(&mut Renderable, &Position, &Door, Option<&Memory>)>,
        Query<&Position, Or<(With<Door>, With<Wall>)>>,
    )>,
    deltas: Res<Deltas>,
) {
    if changed.is_empty() {
        return;
    }

    let index: HashSet<_> = set.p1().iter().map(|p| p.0).collect();
    let deltas = &deltas.0;

    let vdoor: HashSet<_> = [ivec2(0, -1), ivec2(0, 1)].into_iter().collect();
    let hdoor: HashSet<_> = [ivec2(1, 0), ivec2(1, 0)].into_iter().collect();

    for (mut renderable, position, door, memory) in set.p0().iter_mut() {
        let neighbors: HashSet<_> = deltas
            .iter()
            .cloned()
            .filter(|n| index.contains(&(position.0 + *n)))
            .collect();

        if vdoor.is_subset(&neighbors) {
            renderable.char = VDOOR;
        } else if hdoor.is_subset(&neighbors) {
            renderable.char = HDOOR;
        } else {
            renderable.char = EMPTY;
        }

        renderable.color = if door.open || memory.is_some() {
            Color::DARK_GRAY
        } else {
            Color::WHITE
        }
    }
}
