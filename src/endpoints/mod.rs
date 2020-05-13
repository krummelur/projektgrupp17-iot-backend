/**
 * Error overrides.
 */
pub mod catchers;
/**
 * Endpoints that deal rfid devices.
 */
pub mod devices_endpoints;
/**
 * Endpoint that deal with videos.
 */
pub mod videos_endpoints;

use serde::Deserialize;
use rocket_contrib::json::{JsonValue};
use serde_json::json;
use rocket::data::{self, FromDataSimple};
use rocket::{Request, Data, Outcome::*};
use rocket::http::Status;
use std::io;
use std::io::Read;

#[derive(Deserialize)]
pub struct VideoBody {
    length_sec: i32
}

#[derive(Deserialize)]
pub struct RegisterBody {
    loc: String,
    tag: String
}

#[derive(Deserialize)]
pub struct StrCont {
    data: String,
}

#[derive(Deserialize)]
pub struct LogMessage {
    error: bool,
    message: String,
}

impl FromDataSimple for StrCont {
    type Error = io::Error;
    #[inline(always)]
    fn from_data(_: &Request, data: Data) -> data::Outcome<Self,  io::Error> {
        let mut value = String::new();
        match data.open().read_to_string(&mut value) {
            Ok(_) => Success(StrCont{data: value}),
            Err(e) => Failure((Status::BadRequest, e))
        }
    }
}

/**
 * Gets the current API version / checks if api is alive
 * 
 * This is an API endpoint mapped to
 * - / [GET]
 */
#[get("/")]
pub fn default() -> &'static str {
    "IoT server v1.0.0"
}

/**
* Logs a message or error to persistent logs.
*
* Returns:
* 
* `{"status": "success", "message": "message|error logged"}`
* 
* # Arguments
* `body` [LogMessage](struct.LogMessage.html) if LogMessage.error is true the message will be logged as an error.
*  */
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


/**
* Logs a message or error to persistent logs.
*
* Returns:
* 
* `{"status": "success", "message": "message|error logged"}`
* 
* # Arguments
* ## Post body (json):
* `{ error: bool, message: <message_to_log> }`
* `body` [LogMessage](struct.LogMessage.html) if LogMessage.error is true the message will be logged as an error.
*  */
#[post("/logs", data = "<body>")]
pub fn log_message(body: StrCont) -> Result<JsonValue, JsonValue>  {
    match serde_json::from_str::<LogMessage>(&body.data[..]) {
        Ok(val) => log_message_json(val),
        Err(_) => log_message_str(body.data)
    }
}

/**
* Logs a string persistent logs.
*
* Returns:
* `{"status": "success", "message": "message logged"}`
* 
* # Arguments
* `body` String, the string to be logged.
*  */
fn log_message_str(body: String) -> Result<JsonValue, JsonValue>  {    
    colour::yellow!("\n====LOG MESSAGE FROM IOT====\n");
    colour::yellow!("{}", body);
    colour::yellow!("\n============================\n");
    Ok(JsonValue(json!({"status": "success", "message": "message logged"})))
}
