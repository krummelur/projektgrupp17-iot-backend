/**
 * Devices business logic
 */
use futures::join;
use crate::persistance::db;
use crate::model::*;
use crate::services::DeviceServiceError;
use crate::services::DeviceServiceError::{
    NoSuchTracker,
    NoSuchReceiver,
    Other
};

/**
 * Registers new tracker location to database
 */
pub async fn ftr_register_tracker_location(receiver_id: &String, tracker_id: &String) -> Result<(), DeviceServiceError> {
    match join!(validate_receiver_id(receiver_id), validate_tracker_id(tracker_id)) {
        (Ok(_), Ok(_))  => 
            match db::register_tracker_to_receiver(receiver_id, tracker_id) {
            Ok(_) => Ok(()),
            Err(e) => {println!("{:?}", e); panic!(e)}
        },
        (Err(_), _) =>  return Err(NoSuchReceiver),
        (_, Err(_)) =>  return Err(NoSuchTracker)
    }
}

/**
 * Unregisters a tracker from a location, only if it currently registered to this location. 
 */
pub async fn ftr_unregister_tracker_location(receiver_id: &String, tracker_id: &String) -> Result<(), DeviceServiceError> {
    match join!(validate_receiver_id(receiver_id), validate_tracker_id(tracker_id)) {
        (Ok(_), Ok(_))  => {
            let tracker_loc = match db::get_tracker_by_id(tracker_id).unwrap().unwrap().location {
                None => return Ok(()),
                Some(val) => val
            };
            match db::get_receiver_by_id(receiver_id) {
                Ok(Some(Receiver {id: _, location})) if (location == tracker_loc) => 
                match db::unregister_tracker(tracker_id) {
                    Err(e) => {eprintln!("{}", e); Err(Other)},
                    Ok(v) => Ok(v)
                },
                _ => Ok(())
            }
        },
        (Err(_), _) => Err(NoSuchReceiver),
        (_, Err(_)) => Err(NoSuchTracker)
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
    match db::get_tracker_by_id(tracker_id) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err("No such tracker exists"),
        Err(e) => {println!("{}",e); Err("Unknown Error when accessing database")}
    }
}





/**************
 * Unit tests *
 **************/
#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use mocktopus::mocking::*;
    use super::*;

    #[test]
    fn validate_nonexistant_tracker_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_tracker_by_id.mock_safe(|_| MockResult::Return(Ok(None)));
        assert!(match block_on(validate_tracker_id(&"id".to_owned())) {
            Err(_) => true,
            _ => false
        }, "Wrong value returned when validating nonextistant tracker"); 
    }

    #[test]
    fn validate_tracker_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_tracker_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Tracker{id: String::from("tracker_id"), location: None}))));
        assert!(match block_on(validate_tracker_id(&"id".to_owned())) {
            Ok(()) => true,
            _ => false
        }, "Wrong value returned when validating tracker"); 
    }

    #[test]
    fn validate_nonexistant_receiver_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_receiver_by_id.mock_safe(|_| MockResult::Return(Ok(None)));
        assert!(match block_on(validate_receiver_id(&"id".to_owned())) {
            Err(_) => true,
            _ => false
        }, "Wrong value returned when validating nonextistant tracker"); 
    }
    
    #[test]
    fn validate_tracker_receiver_unittest() {
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_receiver_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Receiver {id: String::from("receiver_1"), location: 1}))));
        assert!(match block_on(validate_receiver_id(&"id".to_owned())) {
            Ok(()) => true,
            _ => false
        }, "Wrong value returned when validating tracker"); 
    }
    
    #[test]
    fn ftr_register_tracker_location_when_nonexistent_receiver_unittest() {    
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_receiver_by_id.mock_safe(|_| MockResult::Return(Err(String::from("ERROR"))));
        db::get_tracker_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Tracker {id: String::from("tracker_id"), location: None}))));
        assert!(match block_on(ftr_register_tracker_location(&String::from("tr"),&String::from("rec"))) {
            Err(NoSuchReceiver) => true,
            _ => false
        })
    }
    
    #[test]
    fn ftr_register_tracker_location_when_nonexistent_tracker_unittest() {    
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_receiver_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Receiver{id: String::from("receiver_id"), location: 1}))));
        db::get_tracker_by_id.mock_safe(|_| MockResult::Return(Err(String::from("no such"))));
        assert!(match block_on(ftr_register_tracker_location(&String::from("tr"),&String::from("rec"))) {
            Err(NoSuchTracker) => true,
            _ => false
        })
    }
    
    #[test]
    fn ftr_register_tracker_location_success_unittest() {    
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_receiver_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Receiver{id: String::from("receiver_id"), location: 1}))));
        db::get_tracker_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Tracker {id: String::from("tracker_id"), location: None}))));
        db::register_tracker_to_receiver.mock_safe(|_,_| MockResult::Return(Ok(())));
        assert!(match block_on(ftr_register_tracker_location(&String::from("tr"),&String::from("rec"))) {
            Ok(()) => true,
            _ => false
        })
    }
    
    #[test]
    fn ftr_unregister_tracker_from_receiver_where_tracker_is_not_in_same_location_unittest() {    
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_receiver_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Receiver{id: String::from("receiver_id"), location: 1}))));
        db::get_tracker_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Tracker {id: String::from("tracker_id"), location: Some(2)}))));
        db::register_tracker_to_receiver.mock_safe(|_,_| MockResult::Return(Ok(())));
        assert!(match block_on(ftr_unregister_tracker_location(&String::from("tr"),&String::from("rec"))) {
            Ok(()) => true,
            _ => false
        })
    }
    
    #[test]
    fn ftr_unregister_tracker_from_receiver_success_unittest() {    
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_receiver_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Receiver{id: String::from("receiver_id"), location: 1}))));
        db::get_tracker_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Tracker {id: String::from("tracker_id"), location: Some(1)}))));
        db::unregister_tracker.mock_safe(|_| MockResult::Return(Ok(())));
        db::register_tracker_to_receiver.mock_safe(|_,_| MockResult::Return(Ok(())));
        assert!(match block_on(ftr_unregister_tracker_location(&String::from("tr"),&String::from("rec"))) {
            Ok(()) => true,
            _ => false
        })
    }
    
    #[test]
    fn ftr_unregister_tracker_from_receiver_where_receiver_nonexistent_unittest() {    
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_receiver_by_id.mock_safe(|_| MockResult::Return(Ok(None)));
        db::get_tracker_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Tracker {id: String::from("tracker_id"), location: Some(1)}))));
        db::unregister_tracker.mock_safe(|_| MockResult::Return(Ok(())));
        db::register_tracker_to_receiver.mock_safe(|_,_| MockResult::Return(Ok(())));
        assert!(match block_on(ftr_unregister_tracker_location(&String::from("tr"),&String::from("rec"))) {
            Err(_) => true,
            _ => false
        })
    }
    
    #[test]
    fn ftr_unregister_tracker_from_receiver_where_tracker_nonexistent_unittest() {    
        db::Dbconn::new.mock_safe(|| panic!("TRIED TO CONNECT TO DB"));
        db::get_tracker_by_id.mock_safe(|_| MockResult::Return(Ok(None)));
        db::get_receiver_by_id.mock_safe(|_| MockResult::Return(Ok(Some(Receiver{id: String::from("receiver_id"), location: 1}))));
        db::unregister_tracker.mock_safe(|_| MockResult::Return(Ok(())));
        db::register_tracker_to_receiver.mock_safe(|_,_| MockResult::Return(Ok(())));
        assert!(match block_on(ftr_unregister_tracker_location(&String::from("tr"),&String::from("rec"))) {
            Err(_) => true,
            _ => false
        })
    }
}