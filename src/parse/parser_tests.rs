use crate::parse::*;
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
    assert!(action.parse_text("ls", "label").is_ok());
    assert_eq!(
        action,
        CommandAction::RunProgram("ls".to_string(), vec![], None, None, None)
    );
    assert!("\"target".starts_with('"'));
}
