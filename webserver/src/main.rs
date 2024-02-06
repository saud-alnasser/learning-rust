use std::net::TcpListener;
use webserver::HttpClient;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let client = HttpClient::with_threads(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        client.handle(stream);
    }
}
