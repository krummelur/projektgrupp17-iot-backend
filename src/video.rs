/**
 * Video business logic layer
*/
use crate::db;
use crate::model::*;

/**
 * Returns the url for the most relevant video, or None, if there is no matching video
 */
pub fn find_relevant_video(display_id: i32) -> Result<Option<AdvertVideo>, String> {
    
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

