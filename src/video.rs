/**
 * Video business logic layer
*/
use rand::prelude::*;
use crate::db;
use crate::model::*;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::video::VideoServiceError::{    
    NoSuchVideo,
    NoSuchDisplay,
    NoSuchOrder,
    NoSuchDisplayLocation,
    Other};
    
#[derive(Debug)]
pub enum VideoServiceError {
    NoSuchVideo,
    NoSuchDisplay,
    NoSuchOrder,
    NoSuchDisplayLocation,
    Other,
}

/**
 *  Registers a view in played_videos 
 */
pub fn register_video_view(display_id: i32, video_id: i32, order_id: &String, length_sec: i32) ->  Result<(), VideoServiceError> {
    //TODO make db-intreaction transactonal since one update could fail.

    match (db::get_advertisement_video_by_id(video_id), db::get_display_by_id(display_id), db::get_order_by_id(order_id)) {
        (Ok(None), _, _) => return Err(NoSuchVideo),
        (Ok(Some(_)), Ok(None), _) => return Err(NoSuchDisplay),
        (Ok(Some(_)), Ok(Some(_)), Ok(None)) => return Err(NoSuchOrder),
        (Ok(Some(video)), Ok(Some(_)), Ok(Some(_))) => video,
        _ => return Err(Other)
    };
    
    let credits_amt = std::cmp::max(length_sec/30, 1);
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
pub fn find_relevant_video(display_id: i32) -> Result<Option<AdvertVideoOrder>, VideoServiceError> {
    //Find out where the display is located
    let location = match db::get_display_location(display_id) {
        Some(val) => val,
        None => return Err(NoSuchDisplayLocation)
    };
    
    //Find out what interests are popular at that location
    let interests = match db::get_interests_at_location(location) {
        Ok(Some(val)) => val,
        Ok(None) => return Ok(None),
        Err(e) => {eprintln!("{}", e); return Err(Other)}
    };
    
    for x in interests.iter() {
        println!("interest: {}, weight: {}", x.0, x.1)    
    }
    
    //Find all payed videos, where the interests match location interests
    //TODO: only match payed videos
    let mut videos: Vec<AdvertVideoOrder> = match db::find_eligible_videos_by_interest(interests.iter().map(|x| x.0).collect()) {
        Ok(Some(val)) => val,
        Ok(None) => return Ok(None),
        Err(e) => {println!("{}", e); return Err(Other)}
    };
    
    //If the below line is used, the compiler should not allow it since it reuses a borrowed value, but it does, and seems to silently fail in the loop, after first iteration. 
    //Seems to be a bug in the compiler, or a bug in iter()
    //let mut video_iter = videos.iter();

    //Shuffle the videos to return a random video from the highest rated interest that has a video.
    println!("{:?}",videos);
    videos.shuffle(&mut thread_rng());
    println!("{:?}",videos);
    for x in interests.iter() {
        match videos.iter().find(|el| el.interest == x.0) {
            Some(val) => return Ok(Some(val.clone())),
            _ => ()
        };
    }
    Ok(None)
}

