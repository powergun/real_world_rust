#[allow(unused_imports)]
use std::io::{ErrorKind, Read, Write, Error as IOError};
use std::net::TcpListener;
#[allow(unused_imports)]
use std::sync::mpsc;
#[allow(unused_imports)]
use std::thread;
use std::string::FromUtf8Error;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:36000";
const MSG_SIZE: usize = 8;

#[derive(Debug)]
pub enum ServerError {
    IOError(IOError),
    DecodeError(FromUtf8Error),
}

impl From<IOError> for ServerError {
    fn from(err: IOError) -> Self {
        ServerError::IOError(err)
    }
}

impl From<FromUtf8Error> for ServerError {
    fn from(err: FromUtf8Error) -> Self {
        ServerError::DecodeError(err)
    }
}

fn decode_message(buff: Vec<u8>) -> Option<String> {
    let raw_msg = buff
        //
        .into_iter()
        .take_while(|&x| x != 0)
        .collect::<Vec<_>>();
    if let Ok(msg) = String::from_utf8(raw_msg) {
        Some(msg)
    } else {
        None
    }
}

pub fn some() -> Result<(), ServerError> {
    let server = TcpListener::bind(LOCAL)?;

    // may consider retry in a real world situation; see the function docstring
    server.set_nonblocking(true)?;

    let mut clients = vec![];
    let (tx, _rx) = mpsc::channel::<String>();

    for _ in 1..2 {
        thread::sleep(Duration::from_millis(100));
        if let Ok((mut socket, addr)) = server.accept() {
            println!("client {} connected", addr);
            let tx = tx.clone();
            clients.push(socket.try_clone()?);
            thread::spawn(move || {
                let mut buff = vec![0; MSG_SIZE];
                let opt_msg = match socket.read_exact(&mut buff) {
                    Ok(_) => decode_message(buff),
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => None,
                    _ => None,
                };
                match opt_msg {
                    Some(msg) => {
                        println!("{:?}: {}", addr, msg);
                        if let Ok(_) = tx.send(msg) {} else {
                            println!("failed to send message to rx");
                        }
                    }
                    _ => (),
                }
            });
        }
    }
    Ok(())
}

#[test]
fn test_create_tcp_server() {}


