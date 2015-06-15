extern crate saturn;

use saturn::server::command;

#[test]
fn should_parse_commands() {
    let test = "create database";
    let command = command::parse(&test);
}
