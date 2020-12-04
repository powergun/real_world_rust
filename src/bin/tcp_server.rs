use std::net::{TcpListener, TcpStream, SocketAddr};
use clap::Clap;
use std::string::FromUtf8Error;
use std::io::{Error as IOError, Read};

#[derive(Clap)]
#[clap(version = "0.1", author = "...")]
struct Opts {
    #[clap(short, long, default_value = "127.0.0.1:36000")]
    address: String,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "0.1", author = "...")]
    ST(SingleThread),
    MT(MultiThread),
}

#[derive(Clap)]
struct SingleThread {}

#[derive(Clap)]
struct MultiThread {}

const MSG_SIZE: usize = 32;

#[derive(Debug)]
enum ServerError {
    IOError(IOError),
    DecodeError(FromUtf8Error),
    ReadSocketError(String),
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

fn decode_message(buff: Vec<u8>) -> Result<String, ServerError> {
    let raw_msg = buff
        //
        .into_iter()
        .take_while(|&x| x != 0)
        .collect::<Vec<_>>();
    let s = String::from_utf8(raw_msg)?;
    Ok(s)
}

fn read_socket(mut socket: &TcpStream) -> Result<String, ServerError> {
    let mut buff = vec![0; MSG_SIZE];
    // see the difference between read_exact() and read()
    // https://rust-lang.github.io/rfcs/0980-read-exact.html
    match socket.read(&mut buff) {
        Ok(_num_read) => decode_message(buff),
        Err(ref err) => Err(ServerError::ReadSocketError(err.to_string())),
    }
}

fn serve_single_thread(mut socket: TcpStream, addr: SocketAddr) -> Result<(), ServerError> {
    let s = read_socket(&socket)?;
    println!("[{:?}] {}", addr, s);
    Ok(())
}

fn _main(opts: &Opts) -> Result<(), ServerError> {
    let address = opts.address.to_string();
    println!("serving at {}", address);
    let server = TcpListener::bind(address)?;
    let (mut socket, addr) = server.accept()?;
    match opts.subcmd {
        SubCommand::ST(_) => serve_single_thread(socket, addr),
        SubCommand::MT(_) => Ok(()),
    }?;
    Ok(())
}

fn main() {
    let opts: Opts = Opts::parse();
    match _main(&opts) {
        Ok(_) => (),
        Err(err) => panic!("{:?}", err),
    };
}
