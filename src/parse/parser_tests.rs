use crate::parse::*;
use lalr1::Parser;

#[test]
fn test_command_parse() {
    let mut command = Command::default();
    assert!(command.parse_text("PATH=/usr/bin:/bin", "label").is_ok());
    assert_eq!(
        command.action,
        CommandAction::SetEnvVar("PATH".to_string(), "/usr/bin:/bin".to_string())
    );
    assert!(command.parse_text("unset WHATEVER", "label").is_ok());
    assert_eq!(
        command.action,
        CommandAction::UnsetEnvVar("WHATEVER".to_string())
    );
    assert!(command.parse_text("ls", "label").is_ok());
    assert_eq!(
        command.action,
        CommandAction::RunProgram("ls".to_string(), vec![], None, None, None)
    );
    assert!("\"target".starts_with('"'));
}
