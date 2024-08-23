use std::io::{Read, Write};
use std::net::TcpStream;
use std::io::stdin;

pub fn run_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            loop {
                println!("Enter command (find <category> <count> | reserve <category> <zone> <row> <seat> | purchase <category> <zone> <row> <seat> | exit):");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");

                let request = input.trim();
                if request == "exit" {
                    break;
                }

                if let Err(e) = stream.write(request.as_bytes()) {
                    eprintln!("Failed to write to server: {}", e);
                    break;
                }

                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let response = String::from_utf8_lossy(&buffer[..size]);
                        println!("Server response: {}", response);
                    }
                    Err(e) => {
                        eprintln!("Failed to read from server: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to connect to server: {}", e),
    }
}

