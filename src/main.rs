use axum::{Server, Router};
use axum::routing::get;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:6969";
    let router = Router::new().route("/", get(root_get));
    let server = Server::bind(&addr.parse().unwrap()).serve(router.into_make_service());

    println!("listening on {}", server.local_addr());
    server.await.unwrap();
}

async fn root_get() -> &'static str {
    "hello world"
}
