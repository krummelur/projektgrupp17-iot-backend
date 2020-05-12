use super::rocket;
use crate::environment;
mod test_data;
use rocket::local::Client;
use rocket::http::Status;
use lazy_static::lazy_static;
use std::sync::Mutex;
use serde_json::Value;

lazy_static! { static ref CONN: Mutex<mysql::Pool> = Mutex::new(connect()) ;}


fn connect() -> mysql::Pool {
    is_test_or_panic();
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
//Never let integration tests run outside test environment!
is_test_or_panic();
Client::new(rocket()).unwrap()
}

fn is_test_or_panic() {
    let ok_env = environment::get_current_env() == environment::TEST_STRING; 
    assert!(ok_env, "Environment was not set to TEST during test");
}

/*Reset the database to an empty state*/
fn reset_db() {
    CONN.lock()
    .unwrap().get_conn().unwrap().query(
        format!(r#"{query}"#, query = test_data::CREATE_SQL_STMT)
    ).map(|_| ()).expect("ERROR RESETTING DB")
}

fn query_db(query: &'static str) {
    CONN.lock()
    .unwrap().get_conn().unwrap().query(query)
        .map(|_| ()).expect(query);
}

/*TESTS*/
#[test]
fn rocket_has_launched() {
    let client = guarded_client();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("IoT server v1.0.0".into()));
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
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response_json["id"], String::from("1"),"Id should be 1 when getting tracker 1");
    assert_eq!(response_json["location"], Value::Null, "Id should be 1 when getting tracker 1");
}

#[test]
fn register_nonexistant_tracker() { 
    reset_db();
    let client = guarded_client();
    let response = client.post("/register/1/1").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());
}

#[test]
fn register_and_get_tracker() {
    reset_db();
    query_db("insert into rfid_tracker (id) values(1);");
    query_db("insert into location (name) values('location1');");
    query_db("insert into location (name) values('location2');");
    query_db("insert into rfid_receiver (id, location) values(1, 1);");
    query_db("insert into rfid_receiver (id, location) values(100, 2);");
    let client = guarded_client();
    
    let mut response = client.post("/register/1/1").dispatch();
    assert_eq!(response.status(), Status::from_code(201).unwrap());

    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response_json["status"], "registered", "Correct status on register");
    assert_eq!(response_json["tracker_id"], Value::String("1".to_string()), "Correct status on register");
    
    client.post("/register/100/1").dispatch();
    let mut response = client.get("/trackers/1").dispatch();
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response.status(), Status::from_code(200).unwrap());
    
    assert_eq!(response_json["id"], String::from("1"), "Correct id on get registered tracker");
    assert_eq!(response_json["location"], 2, "Correct location on get registered tracker");
}

#[test]
fn unregister_nonexistant_tracker() { 
    reset_db();
    let client = guarded_client();
    let response = client.post("/unregister/1/1").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());
}

#[test]
fn unregister_and_get_tracker() {
    reset_db();
    query_db("insert into rfid_tracker (id) values(1);");
    query_db("insert into location (name) values('location1');");
    query_db("insert into location (name) values('location2');");
    query_db("insert into rfid_receiver (id, location) values(1, 1);");
    query_db("insert into rfid_receiver (id, location) values(100, 2);");
    let client = guarded_client();
    
    client.post("/register/1/1").dispatch();
    client.post("/register/100/1").dispatch();
    let mut response = client.post("/unregister/100/1").dispatch();
    
    assert_eq!(response.status(), Status::from_code(200).unwrap());
    
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response_json["status"], "unregistered", "Correct mesg on unregister");
    assert_eq!(response_json["tracker_id"], Value::String("1".to_string()), "Correct location on get registered tracker");
    
    let mut response = client.get("/trackers/1").dispatch();
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response.status(), Status::from_code(200).unwrap());
    
    assert_eq!(response_json["id"], String::from("1"), "Correct id on get registered tracker");
    assert_eq!(response_json["location"], Value::Null, "Correct location on get registered tracker");
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
    assert_eq!(response_json["message"], "no trackers registered to location", "Wrong message on video response");
    assert_eq!(response_json["video"], Value::Null, "When receiver has no trackers, video should be null");
}

#[test]
fn get_video_for_with_trackers_when_trackers_move_around() {
    reset_db();
    query_db("insert into location (name) values('location1');");
    query_db("insert into location (name) values('location2');");
    query_db("insert into display (location) values(1);");
    query_db("insert into interest (name) values('sport');");
    query_db("insert into interest (name) values('movies');");
    query_db("insert into rfid_tracker (id) values('tracker1');");
    query_db("insert into rfid_tracker (id) values('tracker2');");
    query_db("insert into rfid_receiver (id, location) values('receiver1', 1);");
    query_db("insert into rfid_receiver (id, location) values('receiver2', 2);");
    query_db("insert into tracker_interest (tracker, interest, weight) values('tracker1', 1, 100);");
    query_db("insert into tracker_interest (tracker, interest, weight) values('tracker2', 1, 10);");
    query_db("insert into tracker_interest (tracker, interest, weight) values('tracker2', 2, 90);");
    query_db("insert into advertisement_video (url, length_sec, interest) values('https://www.youtube.com/watch?v=oHg5SJYRHA0', 120, 1);");
    query_db("insert into advertisement_video (url, length_sec, interest) values('interest2_video', 10, 2);");
    query_db("insert into agency (orgnr, name) values(1, \"agency1\");");
    query_db("insert into users (username, email, pass_hash, agency) values(\"user1\", \"email@example.com\", \"HASH\",1);");
    query_db("insert into orders (id, credits, user) values(\"1\", 100, \"email@example.com\");");
    query_db("insert into advertisement_order (video, orders, start_time_epoch, end_time_epoch) values(1, '1',0, 1);");
    query_db("insert into orders (id, credits, user) values(\"2\", 20, \"email@example.com\");");
    query_db("insert into advertisement_order (video, orders, start_time_epoch, end_time_epoch) values(2, '2',0, 1);");
    
    let client = guarded_client();
    client.post("/register/receiver1/tracker1").dispatch();
    /*
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
    */
}
/*
#[test]
fn when_getting_videos_orders_with_no_credit_are_not_given() {
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
    query_db("insert into agency (orgnr, name) values(1, \"agency1\");");
    query_db("insert into users (username, email, pass_hash, agency) values(\"user1\",  \"email@example.com\", \"HASH\",1);");
    query_db("insert into orders (id, credits, user) values(\"1\", 0, \"user1\");");
    query_db("insert into advertisement_order (video, orders, start_time_epoch, end_time_epoch) values(1, 1,0, 1);");
    query_db("insert into orders (id, credits, user) values(\"2\", 20, \"user1\");");
    query_db("insert into advertisement_order (video, orders, start_time_epoch, end_time_epoch) values(2, 2,0, 1);");

    let client = guarded_client();
    client.post("/register/1/1").dispatch();
    client.post("/register/1/2").dispatch();
    
    let mut response = client.get("/video/1").dispatch();
    assert_eq!(response.status(), Status::from_code(200).unwrap());
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response_json["video"]["url"], String::from("interest2_video"));
    assert_eq!(response_json["video"]["length"], 10);
    assert_eq!(response_json["message"], Value::String(String::from("video found")));
}


#[test]
fn when_video_played_order_credits_are_withdrawn() {
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
    query_db("insert into agency (orgnr, name) values(1, \"agency1\");");
    query_db("insert into users (username, email, pass_hash, agency) values(\"user1\",  \"email@example.com\", \"HASH\",1);");
    query_db("insert into orders (id, credits, user) values(\"1\", 100, \"user1\");");
    query_db("insert into advertisement_order (video, orders, start_time_epoch, end_time_epoch) values(1, '1',0, 1);");
    
    let client = guarded_client();
    let mut response = client.post("/views/1/1/1").dispatch();
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response.status(), Status::from_code(200).unwrap());
    assert_eq!(response_json["status"], String::from("success"));
    assert_eq!(response_json["message"], String::from("video play logged"));
    
    let sql_res: i32 = CONN.lock().unwrap().get_conn().unwrap().first("select credits from orders where id = '1'").unwrap().unwrap();
    assert_eq!(sql_res, 96);
}


#[test]
fn correct_error_on_register_nonexistent_video() {
    reset_db();
    let client = guarded_client();
    let mut response = client.post("/views/1/1/1").dispatch();
    let response_json: Value = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response.status(), Status::from_code(400).unwrap());
    assert_eq!(response_json["status"], String::from("error"));
    assert_eq!(response_json["message"], String::from("no video with id 1 found"));
}

#[test]
fn get_with_invalid_input_should_not_sql_error() {
    reset_db();
    let client = guarded_client();
    
    let response = client.get("/trackers/%27ss%22").dispatch();
    assert_ne!(response.status(), Status::from_code(500).unwrap());
    assert_eq!(response.status(), Status::from_code(404).unwrap());

    let response = client.get("/trackers/%27sda").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());

    let response = client.post("/trackers/%27sda%27%60/1").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());

    let response = client.post("/trackers/1/%5C%22%27%60r%60%27").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());
}
*/