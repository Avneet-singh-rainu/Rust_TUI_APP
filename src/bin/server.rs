use core::panic;
use std::str;
use std::{
    io::{self, stdout, Read, Write},
    net::{TcpListener, TcpStream},
};

fn handleclient(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                eprint!("message received by server of size :{}", n);
                let message = str::from_utf8(&buf[0..n]).unwrap();
                eprint!("{}", message);
                stream.write(message.as_bytes()).unwrap();
            }

            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
fn main() -> io::Result<()> {
    eprint!("listening to 127.0.0.1:8080 ");
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || handleclient(stream));
            }
            Err(e) => {
                eprint!("error: {:?}", e);
            }
        }
    }

    Ok(())
}
