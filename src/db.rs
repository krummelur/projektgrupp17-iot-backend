/**
 * Database layer.
 */
//use mysql::prelude::*;
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

pub fn unregister_tracker(tracker_id: i32) -> Result<(), String> {
    match DB.lock().unwrap().get_conn().prep_exec("update rfid_tracker set location = null where id = ?", vec![tracker_id]) {
        Ok(_) => Ok(()),
        Err(e) => e.print_err_get_mess::<()>()
    }
}

pub fn register_tracker_to_receiver(receiver: i32, tracker: i32) -> Result<(), String> {
    //mysql::QueryResult
    /*let receiver_location_res = DB.lock().unwrap().get_conn().first_exec(
    "select id, location from rfid_receiver where id = ?", (receiver,));    
    let db_receiver = match receiver_location_res {
        Ok(Some((id, location))) => Receiver{id, location},
        Ok(None) => return Ok(()),
        Err(e) => {println!("{}", e); return Err(e)}
    };
    */
    let db_receiver = match get_receiver_by_id(receiver) {
        Ok(Some(val)) => val,
        Ok(None) => return Ok(()),
        Err(e) => return e.print_err_get_mess()
    };

    //The guard lets us borrow the value for several operations. Like a lock. 
     
    match DB.lock().unwrap().get_conn().prep_exec("update rfid_tracker set location = ? where id = ?", (db_receiver.location, tracker)) {
        Ok(_) => Ok(()),
        Err(e) =>  e.print_err_get_mess()
    }
}

pub fn get_tracker_by_id(tracker_id: i32) -> Result<Option<Tracker>, String> {
    match DB.lock().unwrap().get_conn().first_exec(
        "select id, location from rfid_tracker where id = ?", (tracker_id,)) {
            Ok(Some((id, location))) => Ok(Some(Tracker{id, location})),
            Ok(None) => Ok(None),
            Err(e) => e.print_err_get_mess()
    }
}

pub fn get_receiver_by_id(receiver_id: i32) -> Result<Option<Receiver>, String> {
    match DB.lock().unwrap().get_conn().first_exec(
        "select id, location from rfid_receiver where id = ?", (receiver_id,)) {
            Ok(Some((id, location))) => Ok(Some(Receiver{id, location})),
            Ok(None) => Ok(None),
            Err(e) => e.print_err_get_mess()
    }
}

pub fn get_display_location(display_id: i32) -> Option<i32> {
    match DB.lock().unwrap().get_conn().first_exec(
    "select location from display where id = ?", (display_id,)) {
        Ok(val) => val,
        Err(e) => {println!("{}", e); return None}
    }
}

pub fn get_display_by_id(display_id: i32) ->  Result<Option<Display>, String> {
    match DB.lock().unwrap().get_conn().first_exec(
        "select id, location from display where id = ?", (display_id,)) {
            Ok(Some((id, location))) => Ok(Some(Display{id, location})),
            Err(e) => e.print_err_get_mess(),
            _ => Ok(None)
        }
}

/**
 * Returns sums up all the interests for trackers in this location
 * and turns into a reverse sorted tuple of (interest, weight) 
 */
pub fn get_interests_at_location(location: i32) -> Result<Option<Vec<(i32, f32)>>, String> {
    
    let selected_p: Result<Vec<(i32, f32)>, mysql::error::Error> =  DB.lock().unwrap().get_conn().prep_exec(
        "select interest, sum(weight) as weight from rfid_tracker, tracker_interest where 
        location = ? and tracker = id
        group by interest
        order by weight desc;", (location,)).map(|result| {
           result.map(|x| x.unwrap()).map(|row| {
           let (i, w) = mysql::from_row(row);
           (i, w)
            }).collect()
        });
        match selected_p {
            Err(e) => e.print_err_get_mess(),
            _ => {let res = selected_p.unwrap(); 
                match res.len() {
                    0 => Ok(None),
                    _ => Ok(Some(res))
                }
            }
        }
    
        /*
    mysql_common::value::Value;
    match DB.lock().unwrap().get_conn().prep_exec(
        //Get the aggregate interest for the location, then map into (interest, weight) tuple
        "select interest, sum(weight) as weight from rfid_tracker, tracker_interest where 
                location = ? and tracker = id
                group by interest
                order by weight desc;", (location,)) {
                    Err(e) => e.printErr_getMess::<Option<Vec<(i32, i32)>>>(),
                    Ok(q_result) => {
                            let mapped = q_result.map(|row| {let r = row.unwrap(); (r[0],r[1])});
                            match mapped.collect::<Vec<(i64, i64)>>().len() {
                            0 => Ok(None),
                            _ => Ok(Some(mapped))
                            }
                        } 
                    }
                    
    */
                    /*
    match DB.lock().unwrap().get_conn().exec_prep(
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
    */
}

pub fn find_eligible_videos_by_interest(interests: Vec<i32>) ->  Result<Option<Vec<AdvertVideo>>, String> {
    let q_marks = &interests.iter().fold(String::from(""), |a, _b| format!("{}, ?", a))[1..];
    let prep_q = format!("SELECT interest, url, length_sec
    FROM advertisement_video where interest in ({})", q_marks);
    println!("{}", prep_q);


    let selected_p: Result<Vec<AdvertVideo>, mysql::error::Error> =  DB.lock().unwrap().get_conn().prep_exec(
        prep_q, interests).map(|result| {
           result.map(|x| x.unwrap()).map(|row| {
           let (interest, url, length_sec) = mysql::from_row(row);
           AdvertVideo{interest, url, length_sec}
            }).collect()
        });
        match selected_p {
            Err(e) => e.print_err_get_mess(),
            _ => {let res = selected_p.unwrap(); 
                match res.len() {
                    0 => Ok(None),
                    _ => Ok(Some(res))
                }
            }
        }

/*
    //This fold is stupid but fun.
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
    */
}

pub fn tracker_exists(tr_id: i32) -> mysql::Result<Option<i32>> {
    DB.lock().unwrap().get_conn().first_exec(
    "select id from rfid_tracker where id = ?", (tr_id,))
}

pub struct Agency {
    pub name: String,
    pub orgnr: String
}
trait PrintErr {
    fn print_err_get_mess<T>(&self) -> Result<T, String>;
}
impl PrintErr for mysql::error::Error {   
    fn print_err_get_mess<T>(&self) -> Result<T, String> {
        eprintln!("ERROR: {}", &self);
        Err(format!("{}", &self))
    }
}


impl PrintErr for String {
    fn print_err_get_mess<T>(&self) -> Result<T, String> {
        eprintln!("ERROR: {}", &self);
        Err(format!("{}", &self))
    }
}

