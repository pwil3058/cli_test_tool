use crate::parse::*;
use lalr1::Parser;

#[test]
fn test_command_parse() {
    let mut command = Command::default();
    assert!(command.parse_text("\"nothing\" something", "label").is_ok());
    assert!(command.parse_text("PATH=/usr/bin:/bin", "label").is_ok());
    assert!(command.parse_text("unset WHATEVER", "label").is_ok());
    assert!("\"target".starts_with('"'));
}
