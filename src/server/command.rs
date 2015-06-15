use std::str;

// contains all the possible commands
// used by both client and server to create a database
pub enum Command {
    Create { db_name: Box<String> },
    ErrNotRecognised
}




pub fn parse(cmd: &[u8]) -> Command {

    let command: &str = str::from_utf8(cmd).unwrap();

    // I can't be bothered to write a full parser so for now
    // commands are in form: CMD_NAME ARGS
    let mut pieces = command.split(" ");
    let first = pieces.nth(0).unwrap();
    let arg = pieces.nth(1).unwrap();

    match first {
        "create" => Command::Create {db_name: Box::new(arg.to_string())},
        _ => Command::ErrNotRecognised
    }
}
