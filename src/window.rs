use crate::Error;
use crate::Layout;
use crate::WindowFlag;
use std::time::Duration;

pub const WINDOW_VARS_SEPARATOR: &str = "'";
// FIXME: regex name can be anything, and other keys should be checked better
pub const WINDOW_VARS_REGEX_VEC: [(&str, usize); 24] = [
    ("window_active", WINDOW_ACTIVE),
    ("window_activity", WINDOW_ACTIVITY),
    ("window_activity_flag", WINDOW_ACTIVITY_FLAG),
    ("window_bell_flag", WINDOW_BELL_FLAG),
    ("window_bigger", WINDOW_BIGGER),
    ("window_end_flag", WINDOW_END_FLAG),
    ("window_flags", WINDOW_FLAGS),
    ("window_format", WINDOW_FORMAT),
    ("window_height", WINDOW_HEIGHT),
    ("window_id", WINDOW_ID),
    ("window_index", WINDOW_INDEX),
    ("window_last_flag", WINDOW_LAST_FLAG),
    ("window_layout", WINDOW_LAYOUT),
    ("window_linked", WINDOW_LINKED),
    ("window_name", WINDOW_NAME),
    ("window_offset_x", WINDOW_OFFSET_X),
    ("window_offset_y", WINDOW_OFFSET_Y),
    ("window_panes", WINDOW_PANES),
    ("window_silence_flag", WINDOW_SILENCE_FLAG),
    ("window_stack_index", WINDOW_STACK_INDEX),
    ("window_start_flag", WINDOW_START_FLAG),
    ("window_visible_layout", WINDOW_VISIBLE_LAYOUT),
    ("window_width", WINDOW_WIDTH),
    ("window_zoomed_flag", WINDOW_ZOOMED_FLAG),
];

// accordingly to tmux.h: Formats
// XXX: check all types, optionality
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Window {
    /// 1 if window active
    pub active: Option<bool>,
    /// Time of window last activity
    pub activity: Option<Duration>,
    /// 1 if window has activity
    pub activity_flag: Option<bool>,
    /// 1 if window has bell
    pub bell_flag: Option<bool>,
    /// 1 if window is larger than client
    pub bigger: Option<bool>,
    /// 1 if window has the highest index
    pub end_flag: Option<bool>,
    /// #F Window flags
    pub flags: Option<WindowFlag>,
    /// 1 if format is for a window (not assuming the current)
    pub format: Option<bool>,
    /// Height of window
    pub height: Option<usize>,
    /// Unique window ID
    pub id: Option<usize>,
    /// #I Index of window
    pub index: Option<usize>,
    /// 1 if window is the last used
    pub last_flag: Option<bool>,
    /// Window layout description, ignoring zoomed window panes
    pub layout: Option<Layout>,
    /// 1 if window is linked across sessions
    pub linked: Option<bool>,
    /// #W Name of window
    pub name: Option<String>,
    /// X offset into window if larger than client
    pub offset_x: Option<usize>,
    /// Y offset into window if larger than client
    pub offset_y: Option<usize>,
    /// Number of panes in window
    pub panes: Option<usize>,
    /// 1 if window has silence alert
    pub silence_flag: Option<bool>,
    /// Index in session most recent stack
    pub stack_index: Option<usize>,
    /// 1 if window has the lowest index
    pub start_flag: Option<bool>,
    /// Window layout description, respecting zoomed window panes
    pub visible_layout: Option<Layout>,
    /// Width of window
    pub width: Option<usize>,
    /// 1 if window is zoomed
    pub zoomed_flag: Option<bool>,
}

// NOTE: u32 mb not enough!
pub const WINDOW_ACTIVE: usize = 1 << 0;
pub const WINDOW_ACTIVITY: usize = 1 << 1;
pub const WINDOW_ACTIVITY_FLAG: usize = 1 << 2;
pub const WINDOW_BELL_FLAG: usize = 1 << 3;
pub const WINDOW_BIGGER: usize = 1 << 4;
pub const WINDOW_END_FLAG: usize = 1 << 5;
pub const WINDOW_FLAGS: usize = 1 << 6;
pub const WINDOW_FORMAT: usize = 1 << 7;
pub const WINDOW_HEIGHT: usize = 1 << 8;
pub const WINDOW_ID: usize = 1 << 9;
pub const WINDOW_INDEX: usize = 1 << 10;
pub const WINDOW_LAST_FLAG: usize = 1 << 11;
pub const WINDOW_LAYOUT: usize = 1 << 12;
pub const WINDOW_LINKED: usize = 1 << 13;
pub const WINDOW_NAME: usize = 1 << 14;
pub const WINDOW_OFFSET_X: usize = 1 << 15;
pub const WINDOW_OFFSET_Y: usize = 1 << 16;
pub const WINDOW_PANES: usize = 1 << 17;
pub const WINDOW_SILENCE_FLAG: usize = 1 << 18;
pub const WINDOW_STACK_INDEX: usize = 1 << 19;
pub const WINDOW_START_FLAG: usize = 1 << 20;
pub const WINDOW_VISIBLE_LAYOUT: usize = 1 << 21;
pub const WINDOW_WIDTH: usize = 1 << 22;
pub const WINDOW_ZOOMED_FLAG: usize = 1 << 23;

pub const WINDOW_NONE: usize = 0;
//pub const WINDOW_DEFAULT: usize = WINDOW_ID | WINDOW_NAME;
pub const WINDOW_ALL: usize = WINDOW_ACTIVE
    | WINDOW_ACTIVITY
    | WINDOW_ACTIVITY_FLAG
    | WINDOW_BELL_FLAG
    | WINDOW_BIGGER
    | WINDOW_END_FLAG
    | WINDOW_FLAGS
    | WINDOW_FORMAT
    | WINDOW_HEIGHT
    | WINDOW_ID
    | WINDOW_INDEX
    | WINDOW_LAST_FLAG
    | WINDOW_LAYOUT
    | WINDOW_LINKED
    | WINDOW_NAME
    | WINDOW_OFFSET_X
    | WINDOW_OFFSET_Y
    | WINDOW_PANES
    | WINDOW_SILENCE_FLAG
    | WINDOW_STACK_INDEX
    | WINDOW_START_FLAG
    | WINDOW_VISIBLE_LAYOUT
    | WINDOW_WIDTH
    | WINDOW_ZOOMED_FLAG;

impl Window {
    pub fn new() -> Self {
        Default::default()
    }

    // XXX: mb deserialize like serde something?
    pub fn from_str(s: &str, bitflags: usize) -> Result<Self, Error> {
        let wv: Vec<&str> = s.split(WINDOW_VARS_SEPARATOR).collect();
        let mut wv = wv.iter();
        // XXX: optimize?
        let mut w = Window::new();
        // for all bitflags
        for var in WINDOW_VARS_REGEX_VEC.iter() {
            // is current bitflag given?
            if bitflags & var.1 == var.1 {
                // does vector element exist?
                if let Some(part) = wv.next() {
                    // is vector element not empty
                    if !part.is_empty() {
                        // decode it and save as struct field
                        match bitflags & var.1 {
                            WINDOW_ACTIVE => w.active = part.parse::<usize>().map(|i| i == 1).ok(),
                            WINDOW_ACTIVITY => {
                                w.activity = part.parse().ok().map(Duration::from_millis)
                            }
                            WINDOW_ACTIVITY_FLAG => {
                                w.activity_flag = part.parse::<usize>().map(|i| i == 1).ok()
                            }
                            WINDOW_BELL_FLAG => {
                                w.bell_flag = part.parse::<usize>().map(|i| i == 1).ok()
                            }
                            WINDOW_BIGGER => w.bigger = part.parse::<usize>().map(|i| i == 1).ok(),
                            WINDOW_END_FLAG => {
                                w.end_flag = part.parse::<usize>().map(|i| i == 1).ok()
                            }
                            WINDOW_FLAGS => w.flags = part.parse().ok(),
                            WINDOW_FORMAT => w.format = part.parse::<usize>().map(|i| i == 1).ok(),
                            WINDOW_HEIGHT => w.height = part.parse().ok(),
                            WINDOW_ID => w.id = part[1..].parse().ok(),
                            WINDOW_INDEX => w.index = part.parse().ok(),
                            WINDOW_LAST_FLAG => {
                                w.last_flag = part.parse::<usize>().map(|i| i == 1).ok()
                            }
                            WINDOW_LAYOUT => w.layout = part.parse().ok(),
                            WINDOW_LINKED => w.linked = part.parse::<usize>().map(|i| i == 1).ok(),
                            WINDOW_NAME => w.name = part.parse().ok(),
                            WINDOW_OFFSET_X => w.offset_x = part.parse().ok(),
                            WINDOW_OFFSET_Y => w.offset_y = part.parse().ok(),
                            WINDOW_PANES => w.panes = part.parse().ok(),
                            WINDOW_SILENCE_FLAG => {
                                w.silence_flag = part.parse::<usize>().map(|i| i == 1).ok()
                            }
                            WINDOW_STACK_INDEX => w.stack_index = part.parse().ok(),
                            WINDOW_START_FLAG => {
                                w.start_flag = part.parse::<usize>().map(|i| i == 1).ok()
                            }
                            WINDOW_VISIBLE_LAYOUT => w.visible_layout = part.parse().ok(),
                            WINDOW_WIDTH => w.width = part.parse().ok(),
                            WINDOW_ZOOMED_FLAG => {
                                w.zoomed_flag = part.parse::<usize>().map(|i| i == 1).ok()
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
        Ok(w)
    }
}
