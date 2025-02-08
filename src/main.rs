use server::Server;

mod serializer;
mod server;

fn main() {
    Server::new(String::from("127.0.0.1"), String::from("3000")).expect("Aron").start();
}