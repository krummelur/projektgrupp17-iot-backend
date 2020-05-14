use rocket_contrib::json::JsonValue;
use serde_json::json;

/**
 * 404 override
 */
#[catch(404)]
pub fn not_found() -> JsonValue {
    JsonValue(json!({
        "status": "error",
        "reason": "not found"
    }))
}

/**
 * 400 orverride
 */
#[catch(400)]
pub fn bad_request() -> JsonValue {
    JsonValue(json!({
        "status": "error",
        "message": "request could not be fullfilled. Check request headers and body format"
    }))
}

/**
 * 422 override
 */
#[catch(422)]
pub fn unproc_request() -> JsonValue {
    JsonValue(json!({
        "status": "error",
        "message": "request could not be processed. Check request headers and body content"
    }))
}