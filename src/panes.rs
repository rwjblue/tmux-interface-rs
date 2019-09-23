use crate::Pane;
use crate::TmuxInterface;
use crate::Error;
use crate::pane::{PANE_VARS_REGEX_VEC, PANE_VARS_SEPARATOR};


pub struct Panes {
    //sessions: Vec<Pane>
}


impl Panes {

    pub fn get(target_window: &str) -> Result<Vec<Pane>, Error> {
        let tmux = TmuxInterface::new();
        let lsp_format = PANE_VARS_REGEX_VEC.iter().map(|t| format!("#{{{}}}", t))
            .collect::<Vec<String>>().join(PANE_VARS_SEPARATOR);
        let panes_str = tmux.list_panes(false, false, Some(&lsp_format), Some(target_window))?;
        Panes::parse(&panes_str)
    }

    // TODO: from_str
    pub fn parse(panes_str: &str) -> Result<Vec<Pane>, Error> {
        let mut panes: Vec<Pane> = Vec::new();
        for line in panes_str.lines() {
            panes.push(line.parse()?);
        }
        Ok(panes)
    }

    //pub fn find(&self, id: usize) -> Result<Pane, Error> {
    //}
}

