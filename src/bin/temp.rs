use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn receive_messages(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Connection closed
                    break;
                }

                let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received from server: {}", received_message);
            }
            Err(e) => {
                eprintln!("Error reading from server: {}", e);
                break;
            }
        }
    }
}

fn send_messages(mut stream: TcpStream) {
    loop {
        let mut user_input = String::new();

        // Get user input
        io::stdin().read_line(&mut user_input).expect("Failed to read user input");

        // Send user input to the server
        stream.write_all(user_input.trim().as_bytes()).expect("Failed to write to stream");

        // Optional: add a delay between messages
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    // Create separate threads for receiving and sending messages
    let receive_stream = stream.try_clone().expect("Failed to clone stream");
    let receive_handle = thread::spawn(move || {
        receive_messages(receive_stream);
    });

    let send_handle = thread::spawn(move || {
        send_messages(stream);
    });

    // Wait for both threads to finish
    receive_handle.join().expect("Receive thread panicked");
    send_handle.join().expect("Send thread panicked");
}
