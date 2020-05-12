use futures::executor::block_on;
use rocket_contrib::json::{JsonValue, Json};
use serde_json::json;
use rocket::get;

use crate::db;
use crate::devices;
use super::RegisterBody;

/**
 * Register a certain tracker to a certain location, using json data
 * body: 
 * {
 *  loc: id, 
 *  tag: id
 * }
 *  */
 #[post("/register", data = "<body>")]
 pub fn register_json(body: Json<RegisterBody>) -> Option<JsonValue> {
     register(body.loc.clone(), body.tag.clone())
 }
 
 /**
 * unregister a certain tracker to a certain location, using json data
 * body: 
 * {
 *  loc: id, 
 *  tag: id
 * }
 *  */
 #[post("/unregister", data = "<body>")]
 pub fn unregister_json(body: Json<RegisterBody>) -> Option<JsonValue> {
     unregister(body.loc.clone(), body.tag.clone())
 }
 
 
 /**
  * Register a certain tracker for a certain station
  * Tracker and station must exist, if not 404 is returned
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
  * Unregisters a certain tracker from a certain receiver. If the tracker is not registered to this receiver, nothing happens
  * but a 200 is returned. 
  */
 #[post("/unregister/<station_id>/<tracker_id>")]
 pub fn unregister(station_id: String, tracker_id: String) -> Option<JsonValue> {
     match block_on(devices::ftr_unregister_tracker_location(&station_id, &tracker_id)) {
         Ok(_) =>  Some(JsonValue(json!({"status": "unregistered", "tracker_id": tracker_id}))),
         Err(e) => {println!("{}",e); None}
     }
 }

 /**
 * Find out what receiver if any a tracker is registered at.
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