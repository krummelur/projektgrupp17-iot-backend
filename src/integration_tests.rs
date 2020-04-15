use super::rocket;
use crate::environment;
mod test_data;
use rocket::local::Client;
use rocket::http::Status;
use lazy_static::lazy_static;
use std::sync::Mutex;
use mysql::prelude::*;
use std::{thread, time};
use serde_json::Value;

lazy_static! { static ref CONN: Mutex<mysql::Pool> = Mutex::new(connect()) ;}

//Integration tests must run in a single thread. Unit tests may still run in parallel, hence this mutex.
//lazy_static! {static ref test_mutex: Mutex<()> = Mutex::new(());}

fn connect() -> mysql::Pool {
    let environment = environment::db_environment_values();
    //Dont connect to aspecific database, it may not exist yet
    let url = format!("mysql://{}:{}@{}", environment.user, environment.pass, environment.host);
    mysql::Pool::new_manual(1, 1, url).expect("error creating pool")
}

fn guarded_client() -> rocket::local::Client {
let ok_env = environment::get_current_env() == environment::TEST_STRING; 
    if !ok_env {
        colour::dark_red!("\n### TRYING TO RUN TESTS OUTSIDE TEST ENVIRONMENT ###\n\n");
    }
    //NEVER! let integration tests run outside test environment!
    assert!(ok_env, "Environment was not set to TEST during test");
    Client::new(rocket()).unwrap()
}

/*Reset the database to an empty state*/
fn reset_db() {
    CONN.lock()
    .unwrap().get_conn().unwrap()
    .query_drop(format!(
        r#"{query}"#, query = test_data::CREATE_SQL_STMT 
    )).unwrap();
    
    thread::sleep(time::Duration::from_millis(1));
}

fn query_db(query: &'static str) {
    CONN.lock()
    .unwrap().get_conn().unwrap()
    .query_drop(query).unwrap();
}


#[test]
fn rocket_has_launched() {
    let client = guarded_client();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("IoT server v0.0.0".into()));
}

#[test]
fn get_nonexistant_tracker() {
    reset_db();
    let client = guarded_client();
    let response = client.get("/trackers/1").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());
}

#[test]
fn get_untracked_tracker_info() {
    reset_db();
    query_db("insert into rfid_tracker (id) values(1)");
    let client = guarded_client();
    let mut response = client.get("/trackers/1").dispatch();
    assert_eq!(response.status(), Status::from_code(200).unwrap());
    assert_eq!(response.body_string(), Some("{ \'id\': 1, \'location\': null}".into()));
}

#[test]
fn register_tracker() {
    reset_db();
    query_db("insert into rfid_tracker (id) values(1);");
    query_db("insert into location (name) values('location1');");
    query_db("insert into location (name) values('location2');");
    query_db("insert into rfid_receiver (id, location) values(1, 1);");
    query_db("insert into rfid_receiver (id, location) values(100, 2);");
    let client = guarded_client();
    
    let mut response = client.post("/register/1/1").dispatch();
    assert_eq!(response.status(), Status::from_code(201).unwrap());
    assert_eq!(response.body_string(), Some("{ \'status\': \'registered\', \'tracker_id\': \'1\' }".into()));
    
    client.post("/register/100/1").dispatch();
    let mut response2 = client.get("/trackers/1").dispatch();
    assert_eq!(response2.status(), Status::from_code(200).unwrap());
    assert_eq!(response2.body_string(), Some("{ \'id\': 1, \'location\': 2}".into()));
}

#[test]
fn get_video_for_nonexistant_display() {
    reset_db();
    query_db("insert into location (name) values('location1');");
    query_db("insert into display (location) values(1);");
    let client = guarded_client();
    
    let response = client.get("/video/2").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());
}


#[test]
fn get_video_for_display_at_empty_location() {
    reset_db();
    query_db("insert into location (name) values('location1');");
    query_db("insert into display (location) values(1);");
    let client = guarded_client();
    
    let mut response = client.get("/video/1").dispatch();
    assert_eq!(response.status(), Status::from_code(200).unwrap());
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response_json["message"], Value::String(String::from("no trackers registered to location")), "Wrong message on video response");
    assert_eq!(response_json["video"], Value::Null, "When receiver has no trackers, video should be null");
}

#[test]
fn get_video_for_with_trackers() {
    reset_db();
    query_db("insert into location (name) values('location1');");
    query_db("insert into location (name) values('location2');");
    query_db("insert into display (location) values(1);");
    query_db("insert into interest (name) values('sport');");
    query_db("insert into interest (name) values('movies');");
    query_db("insert into rfid_tracker (id) values(1);");
    query_db("insert into rfid_tracker (id) values(2);");
    query_db("insert into rfid_receiver (id, location) values(1, 1);");
    query_db("insert into rfid_receiver (id, location) values(2, 2);");
    query_db("insert into tracker_interest (tracker, interest, weight) values(1, 1, 100);");
    query_db("insert into tracker_interest (tracker, interest, weight) values(2, 1, 10);");
    query_db("insert into tracker_interest (tracker, interest, weight) values(2, 2, 90);");
    query_db("insert into advertisement_video (url, length_sec, interest) values('https://www.youtube.com/watch?v=oHg5SJYRHA0', 120, 1);");
    query_db("insert into advertisement_video (url, length_sec, interest) values('interest2_video', 10, 2);");
    
    let client = guarded_client();
    client.post("/register/1/1").dispatch();
    client.post("/register/1/2").dispatch();
    
    let mut response = client.get("/video/1").dispatch();
    assert_eq!(response.status(), Status::from_code(200).unwrap());
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response_json["video"]["url"], String::from("https://www.youtube.com/watch?v=oHg5SJYRHA0"));
    assert_eq!(response_json["video"]["length"], 120);
    assert_eq!(response_json["message"], Value::String(String::from("video found")));

    client.post("/register/2/1").dispatch();
    let mut response = client.get("/video/1").dispatch();
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response_json["video"]["url"], String::from("interest2_video"));
    assert_eq!(response_json["video"]["length"], 10);
    assert_eq!(response_json["message"], Value::String(String::from("video found")));
}