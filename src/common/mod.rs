use crate::Error;
use std::fmt;
use std::str::FromStr;

pub mod pane_options;
pub mod server_options;
pub mod session_options;
pub mod window_options;

pub mod pane_options_tests;
pub mod server_options_tests;
pub mod session_options_tests;
pub mod window_options_tests;

use crate::common::pane_options::PaneOptions;
use crate::common::server_options::ServerOptions;
use crate::common::session_options::SessionOptions;
use crate::common::window_options::WindowOptions;

pub struct Options {
    pub server_options: ServerOptions,
    pub session_options: SessionOptions,
    pub window_options: WindowOptions,
    pub pane_options: PaneOptions,
}

#[derive(PartialEq, Clone, Debug)]
pub enum StatusKeys {
    Vi,
    Emacs,
}

impl FromStr for StatusKeys {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "vi" => Ok(Self::Vi),
            "emacs" => Ok(Self::Emacs),
            _ => Err(Error::ParseStatusKeys),
        }
    }
}

impl fmt::Display for StatusKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Vi => write!(f, "vi"),
            Self::Emacs => write!(f, "emacs"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Switch {
    On,
    Off,
}

impl FromStr for Switch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(Error::ParseSwitch),
        }
    }
}

impl fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
        }
    }
}
