use mysql::*;
use mysql::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::env;

//singleton reference to the connection.
lazy_static! { static ref DB: Mutex<Dbconn> = Mutex::new(Dbconn::new()) ;}

struct Dbconn {
    conn: Conn,
}

impl Dbconn {
    pub fn new() -> Dbconn {
        let user_var = "SQL_USERNAME";
        let pass_var = "SQL_PASSWORD";
        let username = env::var(user_var).unwrap_or_else(|_| panic!(format!("Error reading environment variable {}", user_var)));
        let password = env::var(pass_var).unwrap_or_else(|_| panic!(format!("Error reading environment variable {}", pass_var)));
        let url = format!("mysql://{}:{}@eu-cdbr-west-02.cleardb.net/heroku_7625137638b3157", username, password);
        let conn_ = Conn::new(url).expect("error creating pool");
        Dbconn {
            conn: conn_
        }
    }
}

pub fn get_all_agencies() -> Result<Vec<Agency>> {
    DB.lock().unwrap().conn.query_map(
    "select * from agency",
    |(orgnr, name)| {
        Agency { name, orgnr }
    })
}

fn find_tracker_by_id(tr_id: String) -> Result<Vec<Tracker>> {
    //let stmt = DB.lock().unwrap().conn.prep("select * from trackers where id = ?");
    //assert!(DB.lock().unwrap().conn.exec_drop(&stmt, ("foo",)).is_ok()); 
    DB.lock().unwrap().conn.query_map(
    format!("select id from trackers where id = {}", tr_id),
    |id| {
        Tracker { id }
    })
}

pub struct Agency {
    pub name: String,
    pub orgnr: String
}


pub struct Tracker {
    pub id: String
}
