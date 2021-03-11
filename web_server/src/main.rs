use crate::thread_pool::ThreadPool;
use crate::connection_handler::handle_connection;
use std::net::TcpListener;

mod connection_handler;
mod thread_pool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
