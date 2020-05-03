pub mod catchers;
pub mod iot_devices;
pub mod videos;

use serde::Deserialize;
use rocket_contrib::json::{JsonValue};
use serde_json::json;

#[derive(Deserialize)]
pub struct RegisterBody {
    loc: String,
    tag: String
}

#[derive(Deserialize)]
pub struct LogMessage {
    error: bool,
    message: String,
}

/**
 * Gets the current API version / checks if api is alive
 */
#[get("/")]
pub fn default() -> &'static str {
    "IoT server v1.0.0"
}

/**
 * Gets the current API version / checks if api is alive
 */
fn log_message_json(body: LogMessage) -> Result<JsonValue, JsonValue>  {
    match body.error {
        true => {
            colour::yellow!("\n====LOG ERROR FROM IOT====\n");   
            colour::dark_red!("{}", body.message);
            colour::yellow!("\n============================\n");
            Ok(JsonValue(json!({"status": "success", "message": "error logged"})))
        } 
        _ => {
            colour::yellow!("\n====LOG MESSAGE FROM IOT====\n");   
            colour::yellow!("{}", body.message);
            colour::yellow!("\n============================\n");
            Ok(JsonValue(json!({"status": "success", "message": "message logged"})))
        }
    }
}

#[post("/logs", data = "<body>")]
pub fn log_message(body: String) -> Result<JsonValue, JsonValue>  {
    match serde_json::from_str::<LogMessage>(&body[..]) {
        Ok(val) => log_message_json(val),
        Err(_) => log_message_str(body)
    }
}

fn log_message_str(body: String) -> Result<JsonValue, JsonValue>  {    
    colour::yellow!("\n====LOG MESSAGE FROM IOT====\n");
    colour::yellow!("{}", body);
    colour::yellow!("\n============================\n");

    Ok(JsonValue(json!({"status": "success", "message": "message logged"})))
}
