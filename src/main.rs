mod error;
mod handler;
mod parser;
use std::error::Error;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
mod data_store;
mod tcp_conn;
use crate::data_store::Datastore;
use crate::handler::handle_stream;
fn main() -> Result<(), Box<dyn Error>> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080")?;
    let store = Arc::new(Mutex::new(Datastore::new()));
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let i_store = Arc::clone(&store);
                std::thread::spawn(move || handle_stream(s, i_store));
            }
            Err(e) => eprintln!("{e}"),
        }
    }
    Ok(())
}
