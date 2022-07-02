//! The [`variables`][`crate::variables`] module contains functions for getting variables from tmux
//!
//! Getting variables
//! * session
//! * window
//! * pane
//! * layout
//! * client
//!
//! # See Also
//! * [Formats][`crate::formats`]
//! * [Tmux Manual -> Formats](https://man7.org/linux/man-pages/man1/tmux.1.html#FORMATS)
pub mod layout;
pub mod pane;
pub mod session;
pub mod window;

pub use layout::*;
pub use pane::*;
pub use session::*;
pub use window::*;
