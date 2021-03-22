use crate::{ClockMode, IfShell, LockServer, RunShell, TmuxCommand, WaitFor};

/// All functions from man tmux "Miscellaneous" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS)
#[cfg(feature = "tmux_1_0")]
pub mod clock_mode;
#[cfg(feature = "tmux_1_0")]
pub mod if_shell;
#[cfg(feature = "tmux_1_0")]
pub mod lock_server;
//#[cfg(feature = "tmux_1_0")]
//pub mod server_info;
//#[cfg(feature = "tmux_1_0")]
//pub mod set_password;
#[cfg(feature = "tmux_1_1")]
pub mod run_shell;
#[cfg(feature = "tmux_1_8")]
pub mod wait_for;

#[cfg(feature = "tmux_1_0")]
pub mod clock_mode_tests;
#[cfg(feature = "tmux_1_0")]
pub mod if_shell_tests;
#[cfg(feature = "tmux_1_0")]
pub mod lock_server_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod server_info_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod set_password_tests;
#[cfg(feature = "tmux_1_1")]
pub mod run_shell_tests;
#[cfg(feature = "tmux_1_8")]
pub mod wait_for_tests;

impl<'a> TmuxCommand<'a> {
    pub fn clock_mode(&self) -> ClockMode<'a> {
        ClockMode::from(self)
    }

    pub fn if_shell(&self) -> IfShell<'a> {
        IfShell::from(self)
    }

    pub fn lock_server(&self) -> LockServer<'a> {
        LockServer::from(self)
    }

    pub fn run_shell(&self) -> RunShell<'a> {
        RunShell::from(self)
    }

    pub fn wait_for(&self) -> WaitFor<'a> {
        WaitFor::from(self)
    }
}
