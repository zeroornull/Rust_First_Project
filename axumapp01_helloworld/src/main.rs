use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    //
    let app = Router::new().route("/", get(handler));
    //
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
