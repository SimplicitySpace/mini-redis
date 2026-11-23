use std::io::{self, Read, Write};
use std::net::TcpStream;

use crate::data_store::ActionResponse;
use crate::parser::{Action, Parser};
pub struct TcpConn;

impl TcpConn {
    pub fn convert_stream_to_action(stream: &mut TcpStream) -> Result<Action, io::Error> {
        let mut buffer: [u8; 1024] = [0; 1024];
        loop {
            let n = match stream.read(&mut buffer) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "client closed connection",
                    ));
                }
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            let req = String::from_utf8_lossy(&buffer[..n]);
            let result = match req {
                std::borrow::Cow::Borrowed(owned) => {
                    let _action = Parser::parse(owned);
                    match _action {
                        Ok(ok) => return Ok(ok),
                        Err(_) => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "Unable to make meaning of command",
                            ));
                        }
                    };
                }
                std::borrow::Cow::Owned(_) => {}
            };
            result
        }
    }
    pub fn write_to_client(stream: &mut TcpStream, response: ActionResponse) {
        let reply = match response {
            ActionResponse::Successful => "OK\n".to_string(),
            ActionResponse::NoteFound(data) => format!("{}\n", data.value),
            ActionResponse::Errored(err) => format!("ERR: {:?}\n", err),
        };
        let _ = stream.write_all(reply.as_bytes());
    }
}
