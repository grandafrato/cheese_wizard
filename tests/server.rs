use cheese_wizard::server;
use std::net::{SocketAddr, TcpListener};

async fn start_server() -> SocketAddr {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(server::app().into_make_service())
            .await
            .unwrap();
    });

    addr
}
