/**
 * Video business logic layer
*/
use crate::db;
use crate::model::*;
use std::time::{SystemTime, UNIX_EPOCH};


/**
 *  Registers a view in played_videos 
 */
pub fn register_video_view(display_id: i32, video_id: i32, order_id: &String) ->  Result<(), Option<()>> {
    //TODO make db-intreaction transactonal since one update could fail.

    let video_data: AdvertVideo = match (db::get_advertisement_video_by_id(video_id), db::get_display_by_id(display_id), db::get_order_by_id(order_id)) {
        (Ok(Some(_)), Ok(None), _)  => {println!("DISPLAY"); return Err(None)},
        (Ok(None), _, _) => {println!("VIDEO NOT FOUND"); return Err(Some(()))},
        (Ok(Some(video)), Ok(Some(_)), Ok(Some(_))) => {println!("ALL FOUND"); video},
        _ => {println!("other"); return Err(None);}
    };
    
    let credits_amt = std::cmp::max(video_data.length_sec/30, 1);
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) =>    match (db::insert_played_video(video_id, n.as_secs(), order_id), db::draw_credits_for_order(order_id, credits_amt)) {
                        (Ok(_), Ok(_)) => Ok(()),
                        _ => panic!("ERROR UPDATING TABLES")
                    },
        Err(e) => panic!("ERROR GETTING SYSTEM TIME!, ERROR\n{}", e),
    }
}

/**
 * Returns an Optional AdvertVideo for the most relevant video, None if there is no payed for video that matches the interests at the location
 */
pub fn find_relevant_video(display_id: i32) -> Result<Option<AdvertVideoOrder>, String> {
    //Find out where the display is located
    let location = match db::get_display_location(display_id) {
        Some(val) => val,
        None => return Err(String::from("Display didn't exist, or did not have a location"))
    };
    
    //Find out what interests are popular at that location
    let interests = match db::get_interests_at_location(location) {
        Ok(Some(val)) => val,
        Ok(None) => return Ok(None),
        Err(e) => {eprintln!("{}", e); return Err(String::from("Error finding interests"))}
    };
    
    for x in interests.iter() {
        println!("interest: {}, weight: {}", x.0, x.1)    
    }
    
    //Find all payed videos, where the interests match location interests
    //TODO: only match payed videos
    let videos = match db::find_eligible_videos_by_interest(interests.iter().map(|x| x.0).collect()) {
        Ok(Some(val)) => val,
        Ok(None) => return Ok(None),
        Err(e) => return Err(format!("Error finding matching videos, {}", e))
    };
    
    //If the below line is used, the compiler should not allow it since it reuses a borrowed value, but it does, and seems to silently fail in the loop, after first iteration. 
    //Seems to be a bug in the compiler, or a bug in iter()
    //let mut video_iter = videos.iter();
    for x in interests.iter() {
        match videos.iter().find(|el| el.interest == x.0) {
            Some(val) => return Ok(Some(val.clone())),
            _ => ()
        };
    }
    Ok(None)
}

