extern crate saturn;

use saturn::server::command;

#[test]
fn should_parse_commands() {
    let test = "create database".as_bytes();
    let command = command::parse(test);
    let res = match command {
        command::Command::Create(a) => true,
        _ => false
    };
    assert!(res == true)
}

#[test]
fn should_not_parse_wrong_commands() {
    let test = "something random".as_bytes();
    let command = command::parse(test);
    let res = match command {
        command::Command::ErrNotRecognised => true,
        _ => false
    };
    assert!(res == true)
}
