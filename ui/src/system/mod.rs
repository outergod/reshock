mod client;
mod input;
mod r#move;
mod spot;
mod view;
mod wall;

pub use client::setup as client_setup;
pub use input::system as input;
pub use r#move::system as r#move;
pub use spot::system as spot;
pub use view::system as view;
pub use wall::system as wall;
