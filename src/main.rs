extern crate saturn;

use saturn::server::handler;

fn main() {
    println!("Launching saturn server...");
    handler::run_server();
}
