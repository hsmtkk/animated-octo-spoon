use log::{info, error};
use std::io::{BufReader, BufWriter};
use std::io::{BufRead, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    env_logger::init();
    info!("start");

    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer = stream.peer_addr().unwrap();
                info!("peer {}", peer);        
                std::thread::spawn(move || {
                    handle_client(stream);
                });        
            },
            Err(e) => {
                error!("{}", e);
            },
        }
    }

    info!("finish");
}

fn handle_client(stream: TcpStream){
    let mut buf_reader = BufReader::new(&stream);
    let mut buf_writer = BufWriter::new(&stream);
    let mut line = String::new();
    match buf_reader.read_line(&mut line){
        Ok(_size) => {
            dbg!(&line);
            let resp = line.to_uppercase();
            match buf_writer.write_all(resp.as_bytes()) {
                Ok(_) => {},
                Err(e) => {error!("{}", e)},
            }
        },
        Err(e) => {
            error!("{}", e);
        },
    }
}