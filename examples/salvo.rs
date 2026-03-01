use salvo::prelude::*;
use openapi_ui::{generate_docs, ThemeMode};

#[handler]
async fn show_docs(res: &mut Response) {
    let openapi_json = r#"{
        "openapi": "3.0.0",
        "info": {
            "title": "Salvo API",
            "version": "1.0.0"
        },
        "paths": {
            "/api/users": {
                "get": {
                    "responses": {
                        "200": { "description": "Success" }
                    }
                }
            }
        }
    }"#;

    let html = generate_docs(openapi_json, ThemeMode::System, None, None).unwrap();
    res.render(Text::Html(html));
}

#[tokio::main]
async fn main() {
    let router = Router::with_path("docs").get(show_docs);
    let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
    Server::new(acceptor).serve(router).await;
}
