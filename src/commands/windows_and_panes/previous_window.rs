use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const PREVIOUS_WINDOW: &'static str = "previous-window";
    #[cfg(feature = "use_cmd_alias")]
    const PREVIOUS_WINDOW: &'static str = "prev";

    /// Move to the previous window in the session
    ///
    /// # Manual
    ///
    /// tmux ^0.9:
    /// ```text
    /// tmux previous-window [-a] [-t target-session]
    /// (alias: prev)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux previous-window [-t target-session]
    /// (alias: prev)
    /// ```
    pub fn previous_window(
        &mut self,
        alert: Option<bool>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if alert.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.command(TmuxInterface::PREVIOUS_WINDOW, &args)?;
        Ok(output)
    }
}
