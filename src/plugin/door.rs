use std::time::Duration;

use bevy::{math::ivec2, prelude::*, utils::HashSet};
use bevy_kira_audio::Audio;
use bevy_tweening::*;

use crate::component::*;

// const VDOOR: char = '║';
// const HDOOR: char = '═';
const VDOOR: char = '╎';
const HDOOR: char = '╌';
const DOOR: char = '+';

const DOOR_OPEN_SOUND: &'static str = "sshock/sounds/00206.wav";

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render)
            .add_system(toggle)
            .add_system(open)
            .add_system(event);
    }
}

fn render(
    mut set: ParamSet<(
        Query<(&mut Renderable, &Position), With<Door>>,
        Query<&Position, With<Room>>,
    )>,
) {
    let index: HashSet<_> = set.p1().iter().map(|p| p.0).collect();

    let deltas: HashSet<_> = (-1..=1)
        .flat_map(|x| {
            (-1..=1).filter_map(move |y| {
                if x == 0 && y == 0 {
                    None
                } else {
                    Some(ivec2(x, y))
                }
            })
        })
        .collect();

    let vdoor: HashSet<_> = [ivec2(0, -1), ivec2(0, 1)].into_iter().collect();
    let hdoor: HashSet<_> = [ivec2(1, 0), ivec2(1, 0)].into_iter().collect();

    for (mut renderable, position) in set.p0().iter_mut() {
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
            renderable.char = DOOR;
        }
    }
}

fn open(mut doors: Query<(&Door, &mut Opaque, &mut Obstacle)>) {
    for (door, mut opaque, mut obstacle) in doors.iter_mut() {
        if door.open {
            opaque.0 = false;
            obstacle.0 = false;
        } else {
            opaque.0 = true;
            obstacle.0 = true;
        }
    }
}

fn event(mut reader: EventReader<TweenCompleted>, mut doors: Query<(Entity, &mut Door)>) {
    let entities: HashSet<_> = reader.iter().map(|e| e.entity).collect();

    for (entity, mut door) in doors.iter_mut() {
        if entities.contains(&entity) {
            door.open = !door.open;
        }
    }
}

fn toggle(
    mut doors: Query<(Entity, &mut Door)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut commands: Commands,
) {
    for (entity, mut door) in doors.iter_mut() {
        if door.toggle {
            door.toggle = false;

            let color = if door.open {
                ColorLens {
                    start: Color::DARK_GRAY,
                    end: Color::WHITE,
                }
            } else {
                ColorLens {
                    start: Color::WHITE,
                    end: Color::DARK_GRAY,
                }
            };

            let mut tween = Tween::new(
                EaseMethod::Linear,
                TweeningType::Once,
                Duration::from_secs_f32(1.5),
                color,
            );

            tween.set_completed_event(0);
            commands.entity(entity).insert(Animator::new(tween));
            audio.play(asset_server.load(DOOR_OPEN_SOUND));
        }
    }
}
