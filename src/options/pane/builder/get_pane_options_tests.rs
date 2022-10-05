#[test]
fn get_pane_options() {
    use crate::{GetPaneOptions, GetPaneOptionsTrait, GetUserOption, GetUserOptions, Switch};

    let cmd = "show -p";

    let mut v = Vec::new();

    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {}", cmd, "allow-rename"));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {}", cmd, "alternate-screen"));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {}", cmd, "remain-on-exit"));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {}", cmd, "window-active-style"));
    #[cfg(feature = "tmux_3_0")]
    v.push(format!("{} {}", cmd, "window-style"));
    #[cfg(feature = "tmux_3_2")]
    v.push(format!("{} {}", cmd, "synchronize-panes"));
    v.push(format!("{} {}", cmd, "@user-option-name"));

    let origin = v.join(" ; ");

    //dbg!(&options);

    let options = GetPaneOptions::new();

    #[cfg(feature = "tmux_3_0")]
    let options = options.allow_rename();
    #[cfg(feature = "tmux_3_0")]
    let options = options.alternate_screen();
    #[cfg(feature = "tmux_3_0")]
    let options = options.remain_on_exit();
    #[cfg(feature = "tmux_3_0")]
    let options = options.window_active_style();
    #[cfg(feature = "tmux_3_0")]
    let options = options.window_style();
    #[cfg(feature = "tmux_3_2")]
    let options = options.synchronize_panes();
    let options = options.user_option("user-option-name");
    let options = options.build().to_string();

    assert_eq!(options, origin);
}
