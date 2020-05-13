use rocket::get;
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;

use super::VideoBody;
use crate::persistance::db;
use crate::services::videos;
use crate::services::VideoServiceError::{
    NoSuchDisplay, NoSuchDisplayLocation, NoSuchOrder, NoSuchVideo, Other,
};

/**
 * Registers a view of a specified video_id from a specific display_id
 */
#[post("/views/<display_id>/<video_id>/<order_id>", data = "<body>")]
pub fn register_view( display_id: i32, video_id: i32, order_id: String, body: Json<VideoBody>) -> Result<JsonValue, status::BadRequest<JsonValue>> {
    //TODO: The number of registered people in at the location should affect number of credits
    match videos::register_video_view(display_id, video_id, &order_id, body.length_sec) {
        Err(NoSuchVideo) => Err(bad_request_builder(format!(
            "no video with id {} found",
            video_id
        ))),
        Err(NoSuchDisplay) => Err(bad_request_builder(format!(
            "no display with id {} found",
            display_id
        ))),
        Err(NoSuchOrder) => Err(bad_request_builder(format!(
            "no order with id {} found",
            order_id
        ))),
        Ok(_) => Ok(JsonValue(
            json!({"status": "success", "message": "video play logged"}),
        )),
        _ => Err(bad_request_builder(format!(
            "An unknown issue with the request"
        ))),
    }
}

fn bad_request_builder(message: String) -> status::BadRequest<JsonValue> {
    status::BadRequest(Some(JsonValue(
        json!({"status":"error", "message": format!("{}", message)}),
    )))
}

/**
 * Get the most appropriate video to play on the screen of specified id
 * Appropriateness depends on the trackers currently registered to the reciver, and their interests
 */
#[get("/video/<display_id>")]
pub fn get_video(display_id: i32) -> Result<JsonValue, Option<status::BadRequest<JsonValue>>> {
    match db::get_display_by_id(display_id) {
        Ok(None) => return Err(None),
        _ => (),
    };
    match videos::find_relevant_video(display_id) {
        Err(NoSuchDisplayLocation) => Err(Some(bad_request_builder(format!(
            "The display {} did not exist, or does not have a location set",
            display_id
        )))),
        Ok(Some(v)) => Ok(JsonValue(
            json!({"video": {"url": v.url, "length": v.length_sec, "order": v.order, "videoId": v.video_id}, "message": "video found"}),
        )),
        Ok(None) => Ok(JsonValue(
            json!({"video": null, "message": "no trackers registered to location" }),
        )),
        Err(Other) => Err(Some(bad_request_builder(format!(
            "un unknown issue with the request"
        )))),
        Err(e) => panic!("{:?} shouldn't happen here", e),
    }
}




/**************
 * Unit tests *
 **************/
#[cfg(test)]
mod tests {
    use mocktopus::mocking::*;
    use super::*;
    use crate::model::{ Display, AdvertVideoOrder };

    #[test]
    fn get_video_for_nonexistent_display_gives_404_unittest() {
        db::Dbconn::new.mock_safe(|| panic!(""));

        db::get_display_by_id.mock_safe(|param| {
            MockResult::Return(match param {
                1 => Ok(None),
                _ => panic!("wrong argument sent to get_display_by_id when asking for display 1"),
            })
        });
        assert_eq!(get_video(1), Err(None), "Getting video should give none when should get nonexistent")
    }
    
    #[test]
    fn get_video_for_display_no_tracker_at_tisplay_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));

        db::get_display_by_id.mock_safe(|param| {
            MockResult::Return(match param {
                1 => Ok(Some(Display {id: 1, location: 1})),
                _ => panic!("wrong argument sent to get_display_by_id when asking for display 1"),
            })
        });

        videos::find_relevant_video.mock_safe(|param| 
            MockResult::Return(match param {
                1 => Ok(None),
                _ => panic!("wrong argument sent to get_display_by_id when asking for display 1"),
            })
        );
        assert_eq!(
            get_video(1),
            Ok(JsonValue(json!({"video": null, "message": "no trackers registered to location" })))
            ,"Getting video should give none when should get nonexistent"
        )
    }
    
    #[test]
    fn get_video_for_display_that_has_trackers_unittest() {
        db::Dbconn::new.mock_safe(|| panic!(""));

        db::get_display_by_id.mock_safe(|param| {
            MockResult::Return(match param {
                1 => Ok(Some(Display {id: 1, location: 1})),
                _ => panic!("wrong argument sent to get_display_by_id when asking for display 1"),
            })
        });

        videos::find_relevant_video.mock_safe(|param| {
            MockResult::Return(match param {
                1 => Ok(Some(AdvertVideoOrder {
                    video_id: 1,
                    interest: 1,
                    url: "example.com/video".to_owned(), 
                    length_sec: 1,
                    order: "order_1".to_owned()
                })),
                _ => panic!("wrong argument sent to get_display_by_id when asking for display 1"),
            })
        });
        assert_eq!(
            get_video(1),
            Ok(JsonValue(
                json!(
                    {"video": 
                    {"url": "example.com/video", "length": 1, "order": "order_1", "videoId": 1}, "message": "video found"})))
            ,"Getting video should give none when should get nonexistent"
        )
    }

    #[test]
    pub fn register_view_when_nonexistent_video_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        videos::register_video_view.mock_safe(|_, _, _, _| {MockResult::Return(Err(NoSuchVideo))});

        let json_body: Json<VideoBody> = Json(VideoBody { length_sec: 1} );
        assert_eq!(register_view(1, 1,"order_id".to_owned(), json_body), Err(bad_request_builder("no video with id 1 found".to_owned())));
    }

    #[test]
    pub fn register_view_when_nonexistent_display_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        videos::register_video_view.mock_safe(|_, _, _, _| {MockResult::Return(Err(NoSuchDisplay))});

        let json_body: Json<VideoBody> = Json(VideoBody { length_sec: 1} );
        assert_eq!(register_view(1, 1,"order_id".to_owned(), json_body), Err(bad_request_builder("no display with id 1 found".to_owned())));
    }

    #[test]
    pub fn register_view_when_nonexistent_order_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        videos::register_video_view.mock_safe(|_, _, _, _| {MockResult::Return(Err(NoSuchOrder))});

        let json_body: Json<VideoBody> = Json(VideoBody { length_sec: 1} );
        assert_eq!(register_view(1, 1,"order_id".to_owned(), json_body), Err(bad_request_builder("no order with id order_id found".to_owned())));
    }
}
