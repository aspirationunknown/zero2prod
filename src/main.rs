use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:8079").expect("Failed to bind to random port");
    run(listener)?.await
}
