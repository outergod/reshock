use std::fmt::Display;

tonic::include_proto!("reshock");

pub const FILE_DESCRIPTOR_SET: &'static [u8] =
    tonic::include_file_descriptor_set!("reshock_descriptor");

impl Display for event::Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            event::Event::State(_) => "State",
            event::Event::Move(_) => "Move",
            event::Event::Door(_) => "Door",
            event::Event::Spot(_) => "Spot",
            event::Event::Log(_) => "Log",
            event::Event::Hit(_) => "Hit",
            event::Event::Death(_) => "Death",
            event::Event::Shoot(_) => "Shoot",
        };

        write!(f, "{}", name)
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.event {
            Some(event) => write!(f, "{}", event),
            None => write!(f, "(empty)"),
        }
    }
}
