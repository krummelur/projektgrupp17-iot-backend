#![feature(proc_macro_hygiene, decl_macro)]
/**
 * Interface layer
 */
#[macro_use] extern crate rocket;
extern crate futures;
mod db;
mod environment;
mod video;
use futures::join;
use rocket::response::status;
use rocket::response::content;
use futures::executor::block_on;


#[get("/")]
fn default() -> &'static str {
    "IoT server v0.0.0"
}

/**
 * Register a certain tracker for a certain station
 * Tracker and station must exist, if not 404 is returned
 */
#[post("/register/<station_id>/<tracker_id>")]
fn register(station_id: i32, tracker_id: i32) ->  Option<status::Created<content::Json<String>>> {
    match block_on(ftr_register_tracker_location(station_id, tracker_id)) {
        Ok((ok_station, ok_tracker)) => 
            Some(status::Created(format!("/trackers/{}", ok_station), 
            Some(content::Json(format!("{{ 'status': 'registered', 'tracker_id': '{}' }}", ok_tracker))))),
        Err(e) => {println!("{}",e); None}
    }
}

/**
 * Find out what receiver if any a tracker is registered at.
 */
#[get("/trackers/<tracker_id>")]
fn get_tracker( tracker_id: i32) ->  Option<Result<content::Json<String>, &'static str>> {
    match db::get_tracker_info(tracker_id) {
        Ok(Some(tr)) => Some(Ok(content::Json(format!("{{ 'id': {}, 'location': {}}}", tr.id, match tr.location {
            Some(val) => format!("{}", val),
            None => format!("null")
        })))),
        Ok(None) => None,
        Err(e) => {println!("{}", e); Some(Err("Unknown error"))}
    }
}


#[get("/video/<display_id>")]
fn get_video(display_id: i32) -> Result<String, status::BadRequest<String>> {
    match video::find_relevant_video(display_id) {
        Err(e) => Err(status::BadRequest(Some(e))),
        Ok(Some(v)) => Ok(v),
        Ok(None) => Ok(String::from("No video to play, (no trackers registered)")) 
    }
}

/// Registers new tracker location to database
async fn ftr_register_tracker_location(station: i32, tracker: i32) -> Result<(i32, i32), &'static str> {
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

async fn validate_receiver_id(station_id: i32) -> Result<(), &'static str>{
    match db::receiver_exists(station_id) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err("No such tracker exists"),
        Err(e) => {println!("{}",e); Err("Unknown Error when accessing database")}
    }
}

async fn validate_tracker_id(tracker_id: i32) -> Result<(), &'static str>{
    match db::tracker_exists(tracker_id) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err("No such tracker exists"),
        Err(e) => {println!("{}",e); Err("Unknown Error when accessing database")}
    }
}

/**
 *  Program entrypoint, initializes rocket with the public endpoints
 * */ 
fn main() {
    rocket::ignite().mount("/", routes![default, register, get_tracker, get_video]).launch();
}   

/** 
 * Tests
 * */
#[cfg(test)]
mod tests {
    use super::*;
    
    /*
    #[test]
    fn fail() { assert!(false)}
    */
    
    #[test]
    fn test_validate_station_id() -> Result<(), String> {
        assert_eq!(block_on(validate_receiver_id(1))  , Ok(()));
        match block_on(validate_receiver_id(-1)) {
            Ok(_) => assert!(false, "Got Ok() on incorrect station_id"),
            Err(_) => assert!(true)
        };
        Ok(())
    }

     #[test]
    fn test_validate_tracker_id() -> Result<(), String> {
        assert_eq!(block_on(validate_tracker_id(1)), Ok(()), "Incorrect value on valid tracker_id");
        match block_on(validate_tracker_id(-1)) {
            Ok(_) => assert!(false, "Got Ok() on incorrect tracker_id"),
            Err(_) => assert!(true, "Got error on incorrect tracker_id")
        };
        Ok(())
    }    
}