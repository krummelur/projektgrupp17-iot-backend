use super::rocket;
use crate::environment;
mod test_data;
use crate::db;
use rocket::local::Client;
use rocket::http::Status;
use mysql::PooledConn;
use lazy_static::lazy_static;
use std::sync::Mutex;
use mysql::TxOpts;
use mysql::prelude::*;

lazy_static! { static ref CONN: Mutex<mysql::Pool> = Mutex::new(connect()) ;}

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
}


#[test]
fn rocket_has_launched() {
    let client = guarded_client();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("IoT server v0.0.0".into()));
}

#[test]
fn tracker_not_found() {
    reset_db();
    let client = guarded_client();
    let mut response = client.get("/tracker/1").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());
}
