//! Prints the server's OpenAPI spec; `mise run gen` writes it to `server/openapi.json` (ADR-0010).

use utoipa::OpenApi;

fn main() {
    println!(
        "{}",
        quincy_server::ApiDoc::openapi()
            .to_pretty_json()
            .expect("spec serialises")
    );
}
