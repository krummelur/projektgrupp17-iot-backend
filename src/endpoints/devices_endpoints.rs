use futures::executor::block_on;
use rocket_contrib::json::{JsonValue, Json};
use serde_json::json;
use rocket::get;

use crate::persistance::db;
use crate::services::devices;
use super::RegisterBody;
use crate::services::DeviceServiceError::{
    NoSuchTracker,
    NoSuchReceiver,
    Other
};


/**
* Registers a specified tracker from a specified receiver, granted both exist, where the arguments are given as request body.
* 
* Responds with:
* - 200: if the tracker and receiver exist.
* - 404: if either the receiver or tracker does not exist
*
* This is an API endpoint mapped to
* - /register [POST]
* 
* # Arguments
* * Post body (json):
*
* `{ loc: <receiver_id>, tag: <tracker_id> }`
* 
* 
*  */
#[post("/register", data = "<body>")]
pub fn register_json(body: Json<RegisterBody>) -> Option<JsonValue> {
    register(body.loc.clone(), body.tag.clone())
}


/**
 * Registers a specified tracker from a specified receiver, granted both exist.
 * 
 * Responds with:
 * - 200: if the tracker and receiver exist.
 * - 404: if either the receiver or tracker does not exist 
 * 
 * This is an API endpoint mapped to
 * - /register/<station_id>/<tracker_id> [POST]
 * 
 * # Arguments
 * * `station_id` - an identifier String of a receiver
 * * `tracker_id` - an identifier String of a tracker
 */
#[post("/register/<station_id>/<tracker_id>")]
pub fn register(station_id: String, tracker_id: String) ->  Option<JsonValue> {
    match block_on(devices::ftr_register_tracker_location(&station_id, &tracker_id)) {
        Ok(()) => 
            Some(JsonValue(json!({"status": "registered", "tracker_id": tracker_id}))),
        Err(e) => {println!("{:?}",e); None}
    }
}

/**
*  Unregisters a specified tracker from a specified receiver. If the tracker is not currently registered to this receiver, this call will have no effect.
* 
* Responds with:
* - 200: if the tracker and receiver exist.
* - 404: if either the receiver or tracker does not exist
*
* This is an API endpoint mapped to
* - /unregister [POST]
* 
* # Arguments
* * Post body (json):
*
* `{ loc: <receiver_id>, tag: <tracker_id> }`
* 
* 
*  */
#[post("/unregister", data = "<body>")]
pub fn unregister_json(body: Json<RegisterBody>) -> Result<JsonValue, Option<JsonValue>> {
    unregister(body.loc.clone(), body.tag.clone())
}


/**
 * Unregisters a specified tracker from a specified receiver. If the tracker is not currently registered to this receiver, this call will have no effect.
 * 
 * Responds with:
 * - 200: if the tracker and receiver exist.
 * - 404: if either the receiver or tracker does not exist 
 * 
 * This is an API endpoint mapped to
 * - /unregister/<station_id>/<tracker_id> [POST]
 * 
 * # Arguments
 * * `station_id` - an identifier String of a receiver
 * * `tracker_id` - an identifier String of a tracker
 */
#[post("/unregister/<station_id>/<tracker_id>")]
pub fn unregister(station_id: String, tracker_id: String) -> Result<JsonValue, Option<JsonValue>> {
    match block_on(devices::ftr_unregister_tracker_location(&station_id, &tracker_id)) {
        Ok(_) =>  Ok(JsonValue(json!({"status": "unregistered", "tracker_id": tracker_id}))),
        Err(NoSuchReceiver) | Err(NoSuchTracker) => Err(None),
        Err(Other) => Err(Some(JsonValue(json!({"status": "error", "message": "unknown error performing the request"}))))
    }
}

 /**
 * Responds with the receiver if any a tracker is registered at.
 * 
 * Responds with:
 * - 200: if the tracker and the receiver exist
 * - 404: if either the receiver or tracker does not exist 
 * 
 * This is an API endpoint mapped to
 * - /trackers/<tracker_id> [GET]
 * 
 * # Arguments
 * * `tracker_id` - an identifier String of a tracker
 */
#[get("/trackers/<tracker_id>")]
pub fn get_tracker(tracker_id: String) ->  Option<Result<JsonValue, &'static str>> {
    match db::get_tracker_by_id(&tracker_id) {
        Ok(Some(tr)) => 
        Some(Ok(JsonValue(json!({"id": tr.id, "location": tr.location})))),
        Ok(None) =>None,
        Err(e) => {println!("{}", e); Some(Err("Unknown error"))}
    }
}