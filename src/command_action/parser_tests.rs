use crate::command_action::*;
use lalr1::Parser;

#[test]
fn test_command_parse() {
    let mut action = CommandAction::default();
    assert!(action.parse_text("PATH=/usr/bin:/bin", "label").is_ok());
    assert_eq!(
        action,
        CommandAction::SetEnvVar("PATH".to_string(), "/usr/bin:/bin".to_string())
    );

    assert!(action.parse_text("unset WHATEVER", "label").is_ok());
    assert_eq!(action, CommandAction::UnsetEnvVar("WHATEVER".to_string()));

    assert!(action.parse_text("cd WHATEVER", "label").is_ok());
    assert_eq!(action, CommandAction::ChangeDir("WHATEVER".to_string()));

    assert!(action.parse_text("ls", "label").is_ok());
    assert_eq!(
        action,
        CommandAction::RunProgram("ls".to_string(), vec![], None, None, None)
    );

    assert!(action.parse_text("echo hello world", "label").is_ok());
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

    assert!(action
        .parse_text("echo hello world < something > else", "label")
        .is_ok());
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

    assert!(action
        .parse_text("echo hello world < something >> else 2> error", "label")
        .is_ok());
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
}
