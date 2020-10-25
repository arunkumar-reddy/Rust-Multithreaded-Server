mod server_core;
mod static_file_server;

use server_core::start_server;
use static_file_server::handle_connection;

fn main() {
    start_server(String::from("0.0.0.0"), String::from("3000"), handle_connection);
}
