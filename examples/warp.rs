use warp::Filter;
use openapi_ui::{generate_docs, ThemeMode};

// In a real app, generate your OpenAPI JSON using utoipa:
//
// ```
// use utoipa::OpenApi;
//
// #[derive(OpenApi)]
// #[openapi(paths(get_api))]
// struct ApiDoc;
//
// let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
// ```

#[tokio::main]
async fn main() {
    let docs = warp::path("docs")
        .map(|| {
            // Replace with utoipa-generated JSON:
            // let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
            let openapi_json = r#"{
                "openapi": "3.0.0",
                "info": {
                    "title": "Warp API",
                    "version": "1.0.0"
                },
                "paths": {
                    "/api": {
                        "get": {
                            "responses": {
                                "200": { "description": "OK" }
                            }
                        }
                    }
                }
            }"#;

            let html = generate_docs(openapi_json, ThemeMode::System, None, None).unwrap();
            warp::reply::html(html)
        });

    println!("Starting server at http://127.0.0.1:3030");
    warp::serve(docs).run(([127, 0, 0, 1], 3030)).await;
}
