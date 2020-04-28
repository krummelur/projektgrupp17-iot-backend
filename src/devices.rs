use futures::join;
use crate::db;
use crate::model::*;

/// Registers new tracker location to database
pub async fn ftr_register_tracker_location(receiver_id: &String, tracker_id: &String) -> Result<(), &'static str> {
    match join!(validate_receiver_id(receiver_id), validate_tracker_id(tracker_id)) {
        (Ok(_), Ok(_))  => 
            match db::register_tracker_to_receiver(receiver_id, tracker_id) {
            Ok(_) => Ok(()),
            Err(e) => {println!("{}", e); panic!(e)}
        },
        (Err(err), _) | (_, Err(err)) => 
            {println!("it went bad"); return Err(err)}
    }
}

/**
 * Unregisters a tracker from a location, only if it currently registered to this location. 
 */
pub async fn ftr_unregister_tracker_location(receiver_id: &String, tracker_id: &String) -> Result<(), &'static str> {
    match join!(validate_receiver_id(receiver_id), validate_tracker_id(tracker_id)) {
        (Ok(_), Ok(_))  => {
            let tracker_loc = match db::get_tracker_by_id(tracker_id).unwrap().unwrap().location {
                None => return Ok(()),
                Some(val) => val
            };
            match db::get_receiver_by_id(receiver_id) {
                Ok(Some(Receiver {id: _, location})) if (location == tracker_loc) => 
                match db::unregister_tracker(tracker_id) {
                    Err(e) => {eprintln!("{}", e); Err("Error unregistering tracker")},
                    Ok(v) => Ok(v)
                },
                _ => Ok(())
            }
        },
        (Err(err), _) | (_, Err(err)) => 
            {println!("it went bad"); return Err(err)}
    }
}

/**
 * Validates a receiver by id. Ok(()) if exists, Err() if not  
 */
pub async fn validate_receiver_id(station_id: &String) -> Result<(), &'static str>{
    match db::get_receiver_by_id(station_id) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err("No such tracker exists"),
        Err(e) => {println!("{}",e); Err("Unknown Error when accessing database")}
    }
}

/**
 * Validates a tracker by id. Ok(()) if exists, Err() if not  
 */
pub async fn validate_tracker_id(tracker_id: &String) -> Result<(), &'static str>{
    match db::tracker_exists(tracker_id) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err("No such tracker exists"),
        Err(e) => {println!("{}",e); Err("Unknown Error when accessing database")}
    }
}