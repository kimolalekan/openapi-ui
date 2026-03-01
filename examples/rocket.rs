#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;
use openapi_ui::{generate_docs, ThemeMode};

#[get("/docs")]
fn show_docs() -> RawHtml<String> {
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
