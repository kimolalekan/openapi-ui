use openapi_ui::{generate_docs, ThemeMode};
use salvo::prelude::*;

// In a real app, generate your OpenAPI JSON using utoipa:
//
// ```
// use utoipa::OpenApi;
//
// #[derive(OpenApi)]
// #[openapi(paths(get_users))]
// struct ApiDoc;
//
// let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
// ```

#[handler]
async fn show_docs(res: &mut Response) {
    // Replace with utoipa-generated JSON:
    // let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
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
