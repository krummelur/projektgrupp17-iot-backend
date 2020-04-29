/**
 * Gets the current API version / checks if api is alive
 */
#[get("/")]
pub fn default() -> &'static str {
    "IoT server v1.0.0"
}