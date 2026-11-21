use std::io::{Read, Write};
use std::net::TcpStream;

use crate::parser::Parser;
pub struct TcpConn;

impl TcpConn {
    pub fn handle_client(mut stream: TcpStream) {
        let mut buffer: [u8; 1024] = [0; 1024];
        loop {
            let n = match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
            };
            let req = String::from_utf8_lossy(&buffer[..n]);
            match req {
                std::borrow::Cow::Borrowed(owned) => {
                    let _action = Parser::parse(owned);
                    println!("Here is my action {:?}", _action);
                }
                std::borrow::Cow::Owned(_) => {}
            }
            stream
                .write("Hello Client".as_bytes())
                .expect("Unable to write to client");
        }
    }
}
