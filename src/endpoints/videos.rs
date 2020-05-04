
use rocket::response::status;
use rocket_contrib::json::{JsonValue};
use serde_json::json;
use rocket::get;

use crate::video;
use crate::video::VideoServiceError:: {NoSuchDisplay, NoSuchOrder, NoSuchVideo, NoSuchDisplayLocation, Other};
use crate::db;

/**
 * Registers a view of a specified video_id from a specific display_id
 */
#[post("/views/<display_id>/<video_id>/<order_id>")]
pub fn register_view(display_id: i32, video_id: i32, order_id: String) -> Result<JsonValue, status::BadRequest<JsonValue>> {
    //TODO: The number of registered people in at the location should affect number of credits    
    match video::register_video_view(display_id, video_id, &order_id) {
        Err(NoSuchVideo) => Err(bad_request_builder(format!("no video with id {} found", video_id))),
        Err(NoSuchDisplay) => Err(bad_request_builder(format!("no display with id {} found", display_id))),
        Err(NoSuchOrder) =>  Err(bad_request_builder(format!("no order with id {} found", order_id))),
        Ok(_) => Ok(JsonValue(json!({"status": "success", "message": "video play logged"}))),
        _ => Err(bad_request_builder(format!("An unknown issue with the request"))), 
    }
}

fn bad_request_builder(message: String) -> status::BadRequest<JsonValue>{
    status::BadRequest(Some(JsonValue(json!({"status":"error", "message": format!("{}", message)}))))
}

/**
 * Get the most appropriate video to play on the screen of specified id
 * Appropriateness depends on the trackers currently registered to the reciver, and their interests
 */
#[get("/video/<display_id>")]
pub fn get_video(display_id: i32) -> Result<JsonValue, Option<status::BadRequest<JsonValue>>> {
    match db::get_display_by_id(display_id) {
        Ok(None) =>  return Err(None),
        _ => ()
    };
    match video::find_relevant_video(display_id) {
        Err(NoSuchDisplayLocation) => Err(Some(bad_request_builder(format!("The display {} did not exist, or does not have a location set", display_id)))),
        Ok(Some(v)) => Ok(JsonValue(json!({"video": {"url": v.url, "length": v.length_sec, "order": v.order, "videoId": v.video_id}, "message": "video found"}))),
        Ok(None) =>Ok(JsonValue(json!({"video": null, "message": "no trackers registered to location" }))),
        Err(Other) => Err(Some(bad_request_builder(format!("un unknown issue with the request")))),
        Err(e) => panic!("{:?} shouldn't happen here", e)
    }
}
