/**
 * This module contains the public facing interface.
 */
pub mod catchers;
pub mod devices_endpoints;
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
pub fn log_message(body: StrCont) -> Result<JsonValue, JsonValue>  {
    match serde_json::from_str::<LogMessage>(&body.data[..]) {
        Ok(val) => log_message_json(val),
        Err(_) => log_message_str(body.data)
    }
}

fn log_message_str(body: String) -> Result<JsonValue, JsonValue>  {    
    colour::yellow!("\n====LOG MESSAGE FROM IOT====\n");
    colour::yellow!("{}", body);
    colour::yellow!("\n============================\n");

    Ok(JsonValue(json!({"status": "success", "message": "message logged"})))
}
