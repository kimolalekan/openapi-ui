#[macro_use]
extern crate rocket;
use openapi_ui::{generate_docs, ThemeMode};
use rocket::response::content::RawHtml;

// In a real app, generate your OpenAPI JSON using utoipa:
//
// ```
// use utoipa::OpenApi;
//
// #[derive(OpenApi)]
// #[openapi(paths(show_items))]
// struct ApiDoc;
//
// let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
// ```

#[get("/docs")]
fn show_docs() -> RawHtml<String> {
    // Replace with utoipa-generated JSON:
    // let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
    let openapi_json = r#"{
        "openapi": "3.0.0",
        "info": {
            "title": "Rocket API",
            "version": "1.0.0"
        },
        "paths": {
            "/api/items": {
                "get": {
                    "responses": {
                        "200": { "description": "OK" }
                    }
                }
            }
        }
    }"#;

    let html = generate_docs(openapi_json, ThemeMode::System, None, None).unwrap();
    RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![show_docs])
}
