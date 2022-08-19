use crate::options::*;
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;
use std::fmt;

// TODO: all options exist in get/set?

pub struct GetServerOption;

pub struct GetGlobalServerOption(GetServerOption);

impl std::ops::Deref for GetGlobalServerOption {
    type Target = GetServerOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Getter for GetGlobalServerOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new()
            .server()
            .global()
            .option(name.into())
            .value()
            .build()
    }
}

pub trait Getter {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().server().option(name).value().build()
    }
}

impl Getter for GetServerOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new()
            .server()
            .option(name.into())
            .value()
            .build()
    }
}

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
impl GetServerOption {
    //pub fn get<T: Into<Cow<'a, str>>>(&self, name: T) -> TmuxCommand<'a> {
    //(self.getter)(name.into())
    //}

    //pub fn gets<'a>(names: ServerOptionB) -> TmuxCommands<'a> {
    //let mut cmds = TmuxCommands::new();
    //for name in names.0 {
    //cmds.push(Self::get(name));
    //}
    //cmds
    //}

    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace<'a>() -> TmuxCommand<'a> {
        Self::get(BACKSPACE)
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit<'a>() -> TmuxCommand<'a> {
        Self::get(BUFFER_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias<'a>() -> TmuxCommand<'a> {
        Self::get(COMMAND_ALIAS)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn copy_command<'a>() -> TmuxCommand<'a> {
        Self::get(COPY_COMMAND)
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal<'a>() -> TmuxCommand<'a> {
        Self::get(DEFAULT_TERMINAL)
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time<'a>() -> TmuxCommand<'a> {
        Self::get(ESCAPE_TIME)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn editor<'a>() -> TmuxCommand<'a> {
        Self::get(EDITOR)
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty<'a>() -> TmuxCommand<'a> {
        Self::get(EXIT_EMPTY)
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached<'a>() -> TmuxCommand<'a> {
        Self::get(EXIT_UNATTACHED)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys<'a>() -> TmuxCommand<'a> {
        Self::get(EXTENDED_KEYS)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events<'a>() -> TmuxCommand<'a> {
        Self::get(FOCUS_EVENTS)
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn history_file<'a>() -> TmuxCommand<'a> {
        Self::get(HISTORY_FILE)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_history_limit<'a>() -> TmuxCommand<'a> {
        Self::get(PROMPT_HISTORY_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    ///set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard<'a>() -> TmuxCommand<'a> {
        Self::get(SET_CLIPBOARD)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn terminal_features<'a>() -> TmuxCommand<'a> {
        Self::get(TERMINAL_FEATURES)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides<'a>() -> TmuxCommand<'a> {
        Self::get(TERMINAL_OVERRIDES)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys<'a>() -> TmuxCommand<'a> {
        Self::get(USER_KEYS)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn quiet<'a>() -> TmuxCommand<'a> {
        Self::get(QUIET)
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn detach_on_destroy<'a>() -> TmuxCommand<'a> {
        Self::get(DETACH_ON_DESTROY)
    }

    /// ### Manual
    ///
    /// ```text
    /// user option
    /// ```
    pub fn user_option<'a, S: fmt::Display>(name: S) -> TmuxCommand<'a> {
        Self::get(format!("{}{}", USER_OPTION_MARKER, name))
    }
}

#[test]
fn parse_server_option() {
    use crate::options::get_server_option::{GetServerOption, TmuxServerOptionOutput};
    use crate::Tmux;

    #[cfg(feature = "tmux_3_1")]
    {
        let origin = "C-?";
        let output = Tmux::new()
            .command(GetServerOption::backspace())
            .output()
            .unwrap();
        let value = TmuxServerOptionOutput::from(output).backspace().unwrap();
        assert_eq!(origin, value);
    }

    #[cfg(feature = "tmux_1_5")]
    {
        let origin = 50;
        let output = Tmux::new()
            .command(GetServerOption::buffer_limit())
            .output()
            .unwrap();
        let value = TmuxServerOptionOutput::from(output).buffer_limit().unwrap();
        assert_eq!(origin, value);
    }
}

#[test]
fn get_server_option_c() {
    use crate::Tmux;

    let cmd = Tmux::new()
        .command(GetServerOption::get(BUFFER_LIMIT))
        .output()
        .unwrap();
    let cmd = Tmux::new()
        .command(GetServerOption::buffer_limit())
        .command(GetServerOption::set_clipboard())
        .output()
        .unwrap();
    dbg!(&cmd);
    let cmd = TmuxServerOptionOutput::from(cmd).buffer_limit();
    dbg!(&cmd);

    let cmd = Tmux::new()
        .command(GetServerOption::command_alias())
        .output()
        .unwrap();
    let cmd = TmuxServerOptionOutput::from(cmd).command_alias();
    dbg!(&cmd);

    let cmds = SetServerOption::command_alias(Some(vec!["asdf".to_string(), "a".to_string()]));
    dbg!(&cmds);
}
