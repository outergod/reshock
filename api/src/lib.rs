use std::fmt::Display;

tonic::include_proto!("reshock");

pub const FILE_DESCRIPTOR_SET: &'static [u8] =
    tonic::include_file_descriptor_set!("reshock_descriptor");

impl Display for event::Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            event::Event::View(_) => "View",
            event::Event::Move(_) => "Move",
            event::Event::Door(_) => "Door",
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
