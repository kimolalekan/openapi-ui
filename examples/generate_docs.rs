use openapi_ui::{generate_docs, ThemeMode};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the sample OpenAPI spec
    let sample_spec = include_str!("../src/sample_data.json");

    // Generate HTML documentation with sample data embedded
    let html = generate_docs(sample_spec, ThemeMode::System, None, None)?;

    // Write to docs.html in the project root
    let output_path = Path::new("docs.html");
    fs::write(output_path, &html)?;

    println!("✓ Generated docs.html successfully!");
    println!("  File size: {} bytes", html.len());
    println!("  Output: {}", output_path.display());

    // Verify sample data is included
    if html.contains("uploadImage") && html.contains("findByStatus") {
        println!("✓ Sample API endpoints are embedded in the HTML");
    }

    // Verify custom features work
    if html.contains("data-theme=\"system\"") {
        println!("✓ System theme switching is enabled");
    }

    Ok(())
}
