mod color;
mod rainbow;
#[cfg(feature = "clap")]
mod rainbow_cmd;

pub use rainbow::*;
pub use color::*;

#[cfg(feature = "clap")]
pub use rainbow_cmd::*;
