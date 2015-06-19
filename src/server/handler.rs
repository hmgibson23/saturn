use std::net::{TcpListener, TcpStream};
use std::thread;


// TODO: The DB is not thread safe!
/*
 * There some major issues at the moment. The first is that the db is
 * cloned into each client and maintains no state so is completely
 * pointless. The seconds is that a single needs to take
 * responsibility for handling the DB, which has not happened yet. The
 * third is that this is not really a database as yet because you'll
 * update a non-shared data structure and not actually do anything
 * with it.  Future concurrency will be implement in a single thread
 * that is responsible for the DB.
 * Since the current implementation is so naive it gives each client
 * one DB to play with. Ha - now that's concurrency!
 */
use server::command;

use std::io::Read;
use std::io::Write;

use db::db::DB;

fn handle_client(mut stream: TcpStream) {
    let mut buf;

    loop {
        // clear out the buffer so we don't send garbage
        buf = [0; 512];

        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // EOF
                    break;
                }
                m
            },
        };


        let command = command::parse(&buf);
        // use the db to construct a command and execute

        // create a new db for this client
        let mut db = DB::new();

        let result = command::dispatch_command(command, &mut db);

        // more elegant?
        match result {
            Ok(a) => match stream.write(&a.as_bytes()) {
                Err(_) => break,
                Ok(_) => continue
            },
            Err(a) => match stream.write(&a.as_bytes()) {
                Err(_) => break,
                Ok(_) => continue
            }
        }
    }
}


/*
 * Spawn a separate thread to handle the DB
 */
fn db_runner() {
    println!("DD runner running...");
}


/*
 * Each server inits its own DB but when we're distributed
 * one server will need to be master
 */
pub fn run_server() {

    let address = "127.0.0.1:8888";
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}
