fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    address: String
}

impl Server {
    fn new(address: String) -> Self {
        Self {
            address
        }
    }

    fn run(self) {
        println!("Server listening on {}!", self.address);
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: HTTP_method
}

enum HTTP_method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}


