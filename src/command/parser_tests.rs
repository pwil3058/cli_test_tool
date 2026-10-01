// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::command_action::*;
use lalr1::Parser;

#[test]
fn test_command_parse() {
    let mut action = CommandAction::default();
    assert!(action.parse_text("PATH=/usr/bin:/bin\n", "label").is_ok());
    assert_eq!(
        action,
        CommandAction::SetEnvVar("PATH".to_string(), "/usr/bin:/bin".to_string())
    );

    assert!(action.parse_text("unset WHATEVER\n", "label").is_ok());
    assert_eq!(action, CommandAction::UnsetEnvVar("WHATEVER".to_string()));

    assert!(action.parse_text("cd WHATEVER\n", "label").is_ok());
    assert_eq!(action, CommandAction::ChangeDir("WHATEVER".to_string()));

    assert!(action.parse_text("ls\n", "label").is_ok());
    assert_eq!(
        action,
        CommandAction::RunProgram("ls".to_string(), vec![], None, None, None)
    );

    assert!(action.parse_text("echo hello world\n", "label").is_ok());
    assert_eq!(
        action,
        CommandAction::RunProgram(
            "echo".to_string(),
            vec!["hello".to_string(), "world".to_string()],
            None,
            None,
            None
        )
    );

    assert!(
        action
            .parse_text("echo hello world < something > else\n", "label")
            .is_ok()
    );
    assert_eq!(
        action,
        CommandAction::RunProgram(
            "echo".to_string(),
            vec!["hello".to_string(), "world".to_string()],
            Some("something".to_string()),
            Some(("else".to_string(), true)),
            None
        )
    );

    assert!(
        action
            .parse_text("echo hello world < something >> else 2> error\n", "label")
            .is_ok()
    );
    assert_eq!(
        action,
        CommandAction::RunProgram(
            "echo".to_string(),
            vec!["hello".to_string(), "world".to_string()],
            Some("something".to_string()),
            Some(("else".to_string(), false)),
            Some(("error".to_string(), true))
        )
    );

    assert!(action.parse_text("ls > /dev/null\n", "label").is_ok());
}
