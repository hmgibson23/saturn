use std::str;

use db::db::DB;


// contains all the possible commands
// used by both client and server to create a database
#[derive (Debug, Clone)]
pub enum Command {
    Create(String),
    Show(String),
    ErrNotRecognised
}


pub fn parse(cmd: &[u8]) -> Command {

    let command: &str = str::from_utf8(cmd).unwrap();

    // I can't be bothered to write a full parser so for now
    // commands are in form: CMD_NAME ARGS
    let mut pieces = command.split(" ");
    let (first, arg) = (pieces.nth(0), pieces.last());

    match (first, arg) {
        (Some("create"), Some(a)) => Command::Create(a.to_string()),
        (Some("show"), Some(a)) => Command::Show(a.to_string()),
        (_, _) => Command::ErrNotRecognised
    }
}


/**
 * Execute the give command
 */
pub fn dispatch_command(command: Command, db: &mut DB) -> Result<String, String> {
    match command {
        Command::Create(a) => DB::create(db, &a),
        Command::Show(_) => DB::show(db),
        Command::ErrNotRecognised => Err("Unrecognised command".to_string())
    }
}
