
use rocket::response::status;
use rocket_contrib::json::{JsonValue};
use serde_json::json;
use rocket::get;

use crate::video;
use crate::db;

/**
 * Registers a view of a specified video_id from a specific display_id
 */
#[post("/views/<display_id>/<video_id>/<order_id>")]
pub fn register_view(display_id: i32, video_id: i32, order_id: String) -> Result<JsonValue, Option<status::BadRequest<JsonValue>>> {
    //TODO: The number of registered people in at the location should affect number of credits    
    match video::register_video_view(display_id, video_id, order_id) {
        Err(None) => Err(None),
        Err(Some(_)) => Err(Some(status::BadRequest(Some(JsonValue(json!({"status": "error", "message": "could not be fullfilled, check video_id"})))))),
        Ok(_) => Ok(JsonValue(json!({"status": "success", "message": "video play logged"})))
    }
}

/**
 * Get the most appropriate video to play on the screen of specified id
 * Appropriateness depends on the trackers currently registered to the reciver, and their interests
 */
#[get("/video/<display_id>")]
pub fn get_video(display_id: i32) -> Result<JsonValue, Option<status::BadRequest<String>>> {
    match db::get_display_by_id(display_id) {
        Ok(None) =>  return Err(None),
        _ => ()
    };
    match video::find_relevant_video(display_id) {
        Err(e) => Err(Some(status::BadRequest(Some(e)))),
        Ok(Some(v)) => Ok(JsonValue(json!({"video": {"url": v.url, "length": v.length_sec, "order": v.order, "videoId": v.video_id}, "message": "video found"}))),
        Ok(None) =>Ok(JsonValue(json!({"video": null, "message": "no trackers registered to location" })))
    }
}
