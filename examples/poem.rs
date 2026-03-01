use poem::{get, handler, listener::TcpListener, Route, Server};
use openapi_ui::{generate_docs, ThemeMode};

#[handler]
fn show_docs() -> String {
    let openapi_json = r#"{
        "openapi": "3.0.0",
        "info": {
            "title": "Poem API",
            "version": "1.0.0"
        },
        "paths": {
            "/api/v1": {
                "get": {
                    "responses": {
                        "200": { "description": "Success" }
                    }
                }
            }
        }
    }"#;

    generate_docs(openapi_json, ThemeMode::System, None, None).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/docs", get(show_docs));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
