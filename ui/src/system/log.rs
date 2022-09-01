use bevy::prelude::*;

use crate::resource::*;

pub fn system(mut reader: EventReader<api::LogEvent>, mut log: ResMut<Log>) {
    for api::LogEvent { entry } in reader.iter() {
        log.0.push(entry.to_string());
    }
}
