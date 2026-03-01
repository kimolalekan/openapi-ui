use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use openapi_ui::{generate_docs, ThemeMode};

// In a real app, generate your OpenAPI JSON using utoipa:
//
// ```
// use utoipa::OpenApi;
//
// #[derive(OpenApi)]
// #[openapi(paths(show_data), components(schemas(DataItem)))]
// struct ApiDoc;
//
// let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
// ```

#[get("/docs")]
async fn show_docs() -> impl Responder {
    // Replace with utoipa-generated JSON:
    // let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
    let openapi_json = r#"{
        "openapi": "3.0.0",
        "info": {
            "title": "Actix API",
            "version": "1.0.0"
        },
        "paths": {
            "/api/data": {
                "get": {
                    "responses": {
                        "200": { "description": "Success" }
                    }
                }
            }
        }
    }"#;

    let html = generate_docs(openapi_json, ThemeMode::System, None, None).unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| App::new().service(show_docs))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
