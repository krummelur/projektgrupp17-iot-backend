/**
 * Video business logic layer
*/
#[cfg(test)]
use mocktopus::macros::*;

use rand::prelude::*;
use crate::persistance::db;
use crate::model::*;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::services::VideoServiceError;
use crate::services::VideoServiceError::{    
    NoSuchVideo,
    NoSuchDisplay,
    NoSuchOrder,
    NoSuchDisplayLocation,
    Other
};

/**
 *  Registers a view in played_videos 
 */
#[cfg_attr(test, mockable)]
pub fn register_video_view(display_id: i32, video_id: i32, order_id: &String, length_sec: i32) ->  Result<(), VideoServiceError> {

    match (db::get_advertisement_video_by_id(video_id), db::get_display_by_id(display_id), db::get_order_by_id(order_id)) {
        (Ok(None), _, _) => return Err(NoSuchVideo),
        (Ok(Some(_)), Ok(None), _) => return Err(NoSuchDisplay),
        (Ok(Some(_)), Ok(Some(_)), Ok(None)) => return Err(NoSuchOrder),
        (Ok(Some(video)), Ok(Some(_)), Ok(Some(_))) => video,
        _ => return Err(Other)
    };
    
    let credits_amt = std::cmp::max(length_sec/8, 1);
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
 * The video returned will be a random video for which
 * - it is payed
 * - its interest is the highest weighted at the location
 * holds
 */
#[cfg_attr(test, mockable)]
pub fn find_relevant_video(display_id: i32) -> Result<Option<AdvertVideoOrder>, VideoServiceError> {
    //Find out where the display is located
    let location = match db::get_display_location(display_id) {
        Some(val) => val,
        None => return Err(NoSuchDisplayLocation)
    };
    
    let interests = match db::get_interests_at_location(location) {
        Ok(Some(val)) => val,
        Ok(None) => return Ok(None),
        Err(e) => {eprintln!("{}", e); return Err(Other)}
    };
    
    for x in interests.iter() {
        println!("interest: {}, weight: {}", x.0, x.1)    
    }
    
    let mut videos: Vec<AdvertVideoOrder> = match db::find_eligible_videos_by_interest(interests.iter().map(|x| x.0).collect()) {
        Ok(Some(val)) => val,
        Ok(None) => return Ok(None),
        Err(e) => {println!("{}", e); return Err(Other)}
    };

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


/**************
 * Unit tests *
 **************/
#[cfg(test)]
mod tests {
    use mocktopus::mocking::*;
    use super::*;
    use crate::model::{ Display, AdvertVideoOrder };

    #[test]
    fn register_video_view_for_nonexistent_video_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));

        db::get_advertisement_video_by_id.mock_safe(|param| {
            MockResult::Return(match param {
                1 => Ok(None),
                val => panic!("wrong argument sent to get_display_by_id when asking for display 1: {}", val),
            })
        });

        db::get_display_by_id.mock_safe(|param| 
            MockResult::Return(match param {
                2 => Ok(None),
                val => panic!("wrong argument sent to get_display_by_id when asking for display 1: {}", val),
            })
        );

        db::get_order_by_id.mock_safe(|param| 
            MockResult::Return(
                match &param[..]  {
                "order_id" => Ok(None),
                val => panic!("wrong argument sent to get_display_by_id when asking for display 1: {}", val),
            })
        );

        assert!(match register_video_view(2,1,&"order_id".to_owned(),  100) {
            Err(NoSuchVideo) => true,
            _ => false
        },"incorrect error type on register")
    }
    
    #[test]
    fn register_video_view_for_nonexistent_display_unittest() {
        db::Dbconn::new.mock_safe(|| panic!(""));
        
        db::get_advertisement_video_by_id.mock_safe(|param| 
            MockResult::Return(match param {
                1 => Ok(Some(AdvertVideo { interest: 1, url: "interest_1".to_owned(), length_sec: 1 })),
                _ => panic!("wrong argument sent to get_display_by_id when asking for display 1"),
            })
        );

        db::get_display_by_id.mock_safe(|param| 
            MockResult::Return(match param {
                2 => Ok(None),
                _ => panic!("wrong argument sent to get_display_by_id when asking for display 1"),
            })
        );

        db::get_order_by_id.mock_safe(|param| 
            MockResult::Return(
                match &param[..]  {
                "order_id" => Ok(None),
                _ => panic!("wrong argument sent to get_display_by_id when asking for display 1"),
            })
        );

        assert!(match register_video_view(2,1,&"order_id".to_owned(),  100) {
            Err(NoSuchDisplay) => true,
            _ => false
        },"incorrect error type on register")
    }
   
    #[test]
    fn register_video_view_success_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_advertisement_video_by_id.mock_safe(|_| 
            MockResult::Return(
                Ok(Some(AdvertVideo {interest: 1, url: "interest_1".to_owned(),length_sec: 1})
            )));
            
            db::get_display_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Display {id: 1, location: 1}))));
            db::get_order_by_id.mock_safe(|_| MockResult::Return(
                Ok(Some(Order { id: "order_1".to_owned(), credits: 100, user: "user_1".to_owned() }))
            ));
            
            db::insert_played_video.mock_safe(|_,_,_| MockResult::Return(Ok(())));    
            db::draw_credits_for_order.mock_safe(|_,_| MockResult::Return(Ok(())));
            
            assert!(match register_video_view(2,1,&"order_id".to_owned(),  100) {
                Ok(()) => true,
                _ => false
            },"incorrect error type on register")
        }
        
    #[test]
    fn get_video_with_notexistent_display_unittest() {
        db::get_display_location.mock_safe(|_| MockResult::Return(None));
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        assert!(match find_relevant_video(1) {
            Err(NoSuchDisplayLocation) => true,
            _ => false
        },"incorrect error on find relevant video")
    }
    
    #[test]
    fn get_video_with_no_trackers_at_display_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_display_location.mock_safe(|_| MockResult::Return(Some(1)));
        db::get_interests_at_location.mock_safe(|_| MockResult::Return(Ok(None)));
        
        assert!(match find_relevant_video(1) {
            Ok(None) => true,
            _ => false
        },"incorrect error on find relevant video")
    }
    
    #[test]
    fn get_video_success_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_display_location.mock_safe(|_| MockResult::Return(Some(1)));
        db::get_interests_at_location.mock_safe(|_| MockResult::Return(Ok(Some(std::vec![(1, 1.0)]))));
        db::find_eligible_videos_by_interest.mock_safe(|_| MockResult::Return(Ok(Some(std::vec![
            AdvertVideoOrder {
                video_id: 1,
                interest: 1,
                url: "example.com/video".to_owned(), 
                length_sec: 100,
                order: "order_1".to_owned()
        }]))));
        
        assert!(match find_relevant_video(1) {
            Ok(Some(AdvertVideoOrder {
                video_id: 1,
                interest: 1,
                url: _, 
                length_sec: 100,
                order: _}
            )) => true,
            _ => false
        },"incorrect error on find relevant video")
    }
}


