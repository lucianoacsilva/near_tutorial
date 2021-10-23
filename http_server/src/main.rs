mod server;
mod http;

use http::HTTP_method;
use http::Request;
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
