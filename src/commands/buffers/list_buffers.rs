use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LIST_BUFFERS: &'static str = "list-buffers";
    /// List the global buffers.
    ///
    /// # Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux list-buffers [-F format]
    /// (alias: lsb)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux list-buffers
    /// (alias: lsb)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux list-buffers [-t target-session]
    /// (alias: lsb)
    /// ```
    pub fn list_buffers(&mut self, format: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        let output = self.command(TmuxInterface::LIST_BUFFERS, &args)?;
        Ok(output)
    }
}
