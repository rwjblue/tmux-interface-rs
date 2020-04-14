#[test]
fn attach_session() {
    use crate::{AttachSession, AttachSessionBuilder, Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux attach-session [-dErx] [-c working-directory] [-t target-session]
        // (alias: attach)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["attach-session", "-d", "-E", "-r", "-x", "-c", "1", "-t", "2"]"#
        );
        Err(Error::Hook)
    }));

    let attach_session = AttachSession {
        detach_other: Some(true),
        not_update_env: Some(true),
        read_only: Some(true),
        parent_sighup: Some(true),
        cwd: Some("1"),
        target_session: Some(&TargetSession::Raw("2")),
    };
    tmux.attach_session(Some(&attach_session)).unwrap_err();

    let attach_session = AttachSessionBuilder::new()
        .detach_other()
        .not_update_env()
        .read_only()
        .parent_sighup()
        .cwd("1")
        .target_session(&TargetSession::Raw("2"))
        .build();
    tmux.attach_session(Some(&attach_session)).unwrap_err();
}
