use server::Server;
use http::Method;
use http::Request;

mod server;
mod http;

fn main() {
    let address = String::from("127.0.0.1:8080");
    let _port = &address[10..]; //index 10 to end

    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    dbg!(get);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
