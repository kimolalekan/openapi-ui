use axum::{response::Html, routing::get, Router};
use openapi_ui::{generate_docs, ThemeMode};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/docs", get(show_docs));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn show_docs() -> Html<String> {
    let openapi_json = r#"{
        "openapi": "3.0.0",
        "info": {
            "title": "Axum API",
            "version": "1.0.0"
        },
        "paths": {
            "/hello": {
                "get": {
                    "responses": {
                        "200": { "description": "OK" }
                    }
                }
            }
        }
    }"#;

    let html = generate_docs(openapi_json, ThemeMode::System, None, None).unwrap();
    Html(html)
}
