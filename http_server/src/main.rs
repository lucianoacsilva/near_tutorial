fn main() {
    let get = HTTP_method::GET("abcd".to_string());
    let post = HTTP_method::POST;
    let put = HTTP_method::PUT;
    let delete = HTTP_method::DELETE(100);

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
    query_string: String,
    method: HTTP_method
}

enum HTTP_method {
    GET(String),
    POST,
    PUT,
    DELETE(u64),
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}


