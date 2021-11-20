use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Server listening on {}!", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(Request) => {},
                                Err(e) => println!("Failed to parse received request: {}", e)
                            }

                        },
                        Err(e) => println!("Failed to read from connecdtion: {}", e)
                    }
                },
                Err(err) => println!("Failed to establish connection: {}", err)
            }
        }

    }
}
