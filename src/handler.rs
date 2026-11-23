use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{data_store::Datastore, tcp_conn::TcpConn};

pub fn handle_stream(mut stream: TcpStream, store: Arc<Mutex<Datastore>>) {
    loop {
        let action = match TcpConn::convert_stream_to_action(&mut stream) {
            Ok(a) => a,
            Err(_) => break,
        };

        let response = store.lock().unwrap().perform_action(action);
        TcpConn::write_to_client(&mut stream, response);
    }
}
