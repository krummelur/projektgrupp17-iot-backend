use futures::join;
use crate::db;
use crate::model::*;

/// Registers new tracker location to database
pub async fn ftr_register_tracker_location(station: i32, tracker: i32) -> Result<(i32, i32), &'static str> {
    match join!(validate_receiver_id(station), validate_tracker_id(tracker)) {
        (Ok(_), Ok(_))  => 
            match db::register_tracker_to_tracker(station, tracker) {
            Ok(_) => Ok((station, tracker)),
            Err(e) => {println!("{}", e); panic!(e)}
        },
        (Err(err), _) | (_, Err(err)) => 
            {println!("it went bad"); return Err(err)}
    }
}

pub async fn ftr_unregister_tracker_location(receiver_id: i32, tracker_id: i32) -> Result<(), &'static str> {
    match join!(validate_receiver_id(receiver_id), validate_tracker_id(tracker_id)) {
        (Ok(_), Ok(_))  => {
            let tracker_loc = match db::get_tracker_info(tracker_id).unwrap().unwrap().location {
                None => return Ok(()),
                Some(val) => val
            };
            match db::get_receiver_info(receiver_id) {
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

pub async fn validate_receiver_id(station_id: i32) -> Result<(), &'static str>{
    match db::receiver_exists(station_id) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err("No such tracker exists"),
        Err(e) => {println!("{}",e); Err("Unknown Error when accessing database")}
    }
}

pub async fn validate_tracker_id(tracker_id: i32) -> Result<(), &'static str>{
    match db::tracker_exists(tracker_id) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err("No such tracker exists"),
        Err(e) => {println!("{}",e); Err("Unknown Error when accessing database")}
    }
}