extern crate saturn;

use saturn::server::command;

#[test]
fn should_parse_commands() {
    let test = "create database";
    let command = command::parse(&test);
    let res = match command {
        command::Command::Create(_) => true,
        _ => false
    };
    assert!(res == true)
}
