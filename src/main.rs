#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate futures;
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
            Some(status::Created(format!("baseurl/trackers/{}", ok_station), 
            Some(content::Json(format!("{{ 'status': 'registered', 'tracker_id': '{}' }}", ok_tracker))))),
        Err(_) => None
    }
}

/// Registers new tracker location to database
async fn ftr_register_tracker_location(station: i32, tracker: i32) -> Result<(i32, i32), &'static str> {
    match join!(validate_station_id(station), validate_tracker_id(tracker)) {
        (Ok(_), Ok(_))  => return Ok((station, tracker)),
        (Err(err), _) | (_, Err(err)) => return Err(err)
    };
}

async fn validate_station_id(station_id: i32) -> Result<&'static str, &'static str>{
    match station_id {
        -1 => Err("Incorrect station_id"),
        _ => Ok("Existing station_id")
    }
}

async fn validate_tracker_id(station_id: i32) -> Result<&'static str, &'static str>{
    match station_id {
        -1 => Err("Incorrect tracker_id"),
        _ => Ok("Existing tracker_id")
    }
}

/**
 *  Program entrypoint, initializes rocket with the public endpoints
 * */ 
fn main() {
    rocket::ignite().mount("/", routes![default, register]).launch();
}


//              Tests
#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn fail() {
        assert!(false)
    }
    */
    
    #[test]
    fn test_validate_station_id() -> Result<(), String> {
        assert_eq!(block_on(validate_station_id(1))  , Ok("Existing station_id"));
        match block_on(validate_station_id(-1)) {
            Ok(_) => assert!(false, "Got Ok() on incorrect station_id"),
            Err(_) => assert!(true)
        };
        Ok(())
    }
     #[test]
    fn test_validate_tracker_id() -> Result<(), String> {
        assert_eq!(block_on(validate_tracker_id(1)), Ok("Existing tracker_id"), "Incorrect value on valid tracker_id");
        match block_on(validate_tracker_id(-1)) {
            Ok(_) => assert!(false, "Got Ok() on incorrect tracker_id"),
            Err(_) => assert!(true, "Got error on incorrect tracker_id")
        };
        Ok(())
    }    
}