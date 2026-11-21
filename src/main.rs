mod error;
mod parser;

use crate::parser::Parser;
use std::net::{TcpListener, TcpStream};
use std::{env, error::Error};
mod tcp_conn;
use tcp_conn::TcpConn;
fn main() -> Result<(), Box<dyn Error>> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                std::thread::spawn(|| TcpConn::handle_client(s));
            }
            Err(e) => eprintln!("{e}"),
        }
    }
 
    Ok(())
}
