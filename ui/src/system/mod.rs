mod client;
mod input;
mod log;
mod r#move;
mod spot;
mod view;
mod wall;

pub use client::setup as client_setup;
pub use input::system as input;
pub use log::system as log;
pub use r#move::system as r#move;
pub use spot::system as spot;
pub use view::system as view;
pub use wall::system as wall;
