/**
 * Database layer.
 */
use mysql::TxOpts;
use mysql::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::environment;
use crate::model::*;


//singleton reference to the connection.
lazy_static! { static ref DB: Mutex<Dbconn> = Mutex::new(Dbconn::new()) ;}

struct Dbconn {
    conn: mysql::Pool,
}

impl Dbconn {
    pub fn get_conn(&self) -> mysql::PooledConn {
        self.conn.get_conn().unwrap()   
    } 
    pub fn new() -> Dbconn {
        let environment = environment::db_environment_values();
        let url = format!("mysql://{}:{}@{}/{}", environment.user, environment.pass, environment.host, environment.db_name);
        let pool = mysql::Pool::new_manual(1, 1, url).expect("error creating pool");
        Dbconn {
            conn: pool
        }
    }
}

pub fn get_all_agencies() -> mysql::Result<Vec<Agency>> {
    DB.lock().unwrap().get_conn().query_map(
    "select * from agency",
    |(orgnr, name)| {
        Agency { name, orgnr }
    })
}

pub fn register_tracker_to_tracker(receiver: i32, tracker: i32) -> Result<(), mysql::error::Error> {
    let receiver_location_res = DB.lock().unwrap().get_conn().query_first(
    format!("select id, location from rfid_receiver where id = {}", receiver));    
    let db_receiver = match receiver_location_res {
        Ok(Some((id, location))) => Receiver{id, location},
        Ok(None) => return Ok(()),
        Err(e) => {println!("{}", e); return Err(e)}
    };
    
    //The guard lets us borrow the value for several operations. Like a lock. 
    let guard = DB.lock().unwrap(); 
    let mut tx = guard.conn.start_transaction(TxOpts::default()).unwrap();
    let result =  tx.exec_drop("update rfid_tracker set location = ? where id = ?", (db_receiver.location, tracker));
        match result {
        Ok(_) => {tx.commit().expect("Error commiting transacton");},
        _ => {tx.rollback().expect("Error rolling back transaction");}
    };

    //dropping the guard releases the resource.
    drop(guard);
    match result {
        Ok(_) => Ok(()),
        Err(e) =>  Err(e)
    }
}

pub fn get_tracker_info(tracker_id: i32) -> Result<Option<Tracker>, String> {
    let matches = DB.lock().unwrap().get_conn().query_map(
    format!("select * from rfid_tracker where id = {}", tracker_id),
    |(id, location)| { 
        Tracker { id, location }
    }).unwrap();

    match matches.len() {
        0 => Ok(None),
        1 => Ok(Some(matches[0])),
        _ => Err(format!("unexpected result, malformed database or backend bug" ))
    }
}

pub fn receiver_exists(tr_id: i32) ->  mysql::Result<Option<i32>> {
    DB.lock().unwrap().get_conn().query_first(
    format!("select id from rfid_receiver where id = {}", tr_id))
    /*
    match DB.lock().unwrap().conn.query_first(
    format!("select id, location from rfid_receiver where id = {}", tr_id)) {
        Ok(Some((id, loc))) => {println!("okay"); Ok(Receiver {id: id, location: loc})},
        Ok(None) => {println!("select id, location from rfid_tracker where id = {}", tr_id); Err("hmm")},
        Err(e) => panic!(e) 
    }
    */
}

pub fn get_display_location(display_id: i32) -> Option<i32> {
    match DB.lock().unwrap().get_conn().query_first(
    format!("select location from display where id = {}", display_id)) {
        Ok(val) => val,
        Err(e) => {println!("{}", e); None}
    }
}

pub fn get_display_by_id(display_id: i32) ->  mysql::Result<Option<i32>> {
    DB.lock().unwrap().get_conn().query_first(
    format!("select id from display where id = {}", display_id))
}


/**
 * Returns sums up all the interests for trackers in this location
 * and turns into a reverse sorted tuple of (interest, weight) 
 */
pub fn get_interests_at_location(location: i32) -> Result<Option<Vec<(i32, i32)>>, String> {
    match DB.lock().unwrap().get_conn().query_map(
        //Get the aggregate interest for the location, then map into (interest, weight) tuple
        format!("select interest, sum(weight) as weight from rfid_tracker, tracker_interest where 
                location = {} and tracker = id
                group by interest
                order by weight desc;", location),
        |(interest, weight)| {
        (interest, weight) }) {
        Ok(val) => match val.len() {
            0 => Ok(None),
            _ => Ok(Some(val))
        },
        Err(_) => Err(String::from("error"))
    }
}

pub fn find_eligible_videos_by_interest(interests: Vec<i32>) ->  Result<Option<Vec<AdvertVideo>>, String> {
    //This fold is stupid but fun.
    println!("\n\n\nSELECT interest, url, length_sec
    FROM advertisement_video where interest in ({}) \n\n\n", &interests.iter().fold(String::from(""), |a, b| format!("{},{}", a , b))[1..]);
    let vids = DB.lock().unwrap().get_conn().query_map(
    format!("SELECT interest, url, length_sec
    FROM advertisement_video where interest in ({})", &interests.iter().fold(String::from(""), |a, b| format!("{}, {}", a , b))[1..]),
    |(interest, url, length_sec)| {
        AdvertVideo {interest, url, length_sec}
    });
    match vids {
        Ok(val) => 
            match val.len() {
            0 => Ok(None),
            _ => Ok(Some(val))
            },
        Err(e) => Err(format!("{}",e))
    } 
}


pub fn tracker_exists(tr_id: i32) -> mysql::Result<Option<i32>> {
    DB.lock().unwrap().get_conn().query_first(
    format!("select id from rfid_tracker where id = {}", tr_id))
}

pub struct Agency {
    pub name: String,
    pub orgnr: String
}