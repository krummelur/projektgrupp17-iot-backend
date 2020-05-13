/**
 * Database layer.
 */
#[cfg(test)]
use mocktopus::macros::*;


#[cfg_attr(test, mockable)]
pub mod db {
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    use crate::model::*;
    use crate::environment;

    /**
     * Database connection pool structure
     */
    pub struct Dbconn {
        conn: mysql::Pool,
    }

    impl Dbconn {
        /**
         * Creates a connection pool
         */
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
    
    //lazy initialized singleton reference to the connection.
    lazy_static! { static ref DB: Mutex<Dbconn> = Mutex::new(Dbconn::new()) ;}

    /**
     * Sets the id of the tracker with id tracker_id to null
     * */ 
    pub fn unregister_tracker(tracker_id: &String) -> Result<(), String> {
        match DB.lock().unwrap().get_conn().prep_exec("update rfid_tracker set location = null where id = ?", vec![tracker_id]) {
            Ok(_) => Ok(()),
            Err(e) => e.print_err_get_mess::<()>()
        }
    }

    /**
     * Sets the location of the tracker with id tracker_id to the location found in location of receiver with id receiver_id.  
     */
    pub fn register_tracker_to_receiver(receiver_id: &String, tracker_id: &String) -> Result<(), String> {
        let db_receiver = match get_receiver_by_id(receiver_id) {
            Ok(Some(val)) => val,
            Ok(None) => return Ok(()),
            Err(e) => return e.print_err_get_mess()
        };
        match DB.lock().unwrap().get_conn().prep_exec("update rfid_tracker set location = ? where id = ?", (db_receiver.location, tracker_id)) {
            Ok(_) => Ok(()),
            Err(e) =>  e.print_err_get_mess()
        }
    }

    /**
     * Gets the Tracker with id tracker_id
     */
    pub fn get_tracker_by_id(tracker_id: &String) -> Result<Option<Tracker>, String> {
        match DB.lock().unwrap().get_conn().first_exec(
            "select id, location from rfid_tracker where id = ?", (tracker_id,)) {
                Ok(Some((id, location))) => Ok(Some(Tracker{id, location})),
                Ok(None) => Ok(None),
                Err(e) => e.print_err_get_mess()
        }
    }

    /**
     * Gets the receiver with id receiver_id  
     */
    pub fn get_receiver_by_id(receiver_id: &String) -> Result<Option<Receiver>, String> {
        match DB.lock().unwrap().get_conn().first_exec(
            "select id, location from rfid_receiver where id = ?", (receiver_id,)) {
                Ok(Some((id, location))) => Ok(Some(Receiver{id, location})),
                Ok(None) => Ok(None),
                Err(e) => e.print_err_get_mess()
        }
    }

    /**
     * Gets the location of the dislay with id display_id 
     */
    //#[cfg_attr(test, mockable)]
    pub fn get_display_location(display_id: i32) -> Option<i32> {
        match DB.lock().unwrap().get_conn().first_exec(
        "select location from display where id = ?", (display_id,)) {
            Ok(val) => val,
            Err(e) => {println!("{}", e); return None}
        }
    }

    /**
     * Gets the Display with the id display_id
     */
    //#[cfg_attr(test, mockable)]
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
    //#[cfg_attr(test, mockable)]
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
    }

    pub fn get_advertisement_video_by_id(video_id: i32) -> Result<Option<AdvertVideo>, String> {
        match DB.lock().unwrap().get_conn().first_exec("SELECT interest, url, length_sec
        FROM advertisement_video where id = ?", (video_id,)) {
            Ok(Some((interest, url, length_sec))) => Ok(Some(AdvertVideo{interest, url, length_sec})),
            Err(e) => e.print_err_get_mess(),
            _ => Ok(None)
        }
    }

    pub fn get_order_by_id(order_id: &String) -> Result<Option<Order>, String> {
        match DB.lock().unwrap().get_conn().first_exec("SELECT id, credits, user
        FROM orders where id = ?", (order_id,)) {
            Ok(Some((id, credits, user))) => Ok(Some(Order{id, credits, user})),
            Err(e) => e.print_err_get_mess(),
            _ => Ok(None)
        }
    }

    pub fn insert_played_video(video_id: i32, time_epoch: u64, order_id: &String) -> Result<(), String> {
        match DB.lock().unwrap().get_conn().prep_exec("INSERT INTO played_video (video, time_epoch, `order`) values(?, ?, ?)", (video_id, time_epoch, order_id)) {
            Ok(_) => Ok(()),
            Err(e) => e.print_err_get_mess::<()>()
        }
    } 

    pub fn draw_credits_for_order(order_id: &String, credits: i32) -> Result<(), String>{
        match DB.lock().unwrap().get_conn().prep_exec("UPDATE orders set credits = credits - ? where id = ?", (credits, order_id)) {
            Ok(_) => Ok(()),
            Err(e) => e.print_err_get_mess::<()>()
        }
    } 

    /**
     * Finds all elligible videos for the interesets contained in the Vec<i32> interests with interest_id's 
     */
    pub fn find_eligible_videos_by_interest(interests: Vec<i32>) ->  Result<Option<Vec<AdvertVideoOrder>>, String> {
        let q_marks = &interests.iter().fold(String::from(""), |a, _b| format!("{}, ?", a))[1..];
        let prep_q = format!(
            "SELECT  advertisement_order.video as video_id, interest, url, length_sec, orders FROM advertisement_video, advertisement_order, orders
            where interest in ({})
            and advertisement_order.video = advertisement_video.id
            and advertisement_order.orders = orders.id
            and orders.credits > 0", q_marks);
        println!("{}", prep_q);

        let selected_p: Result<Vec<AdvertVideoOrder>, mysql::error::Error> =  DB.lock().unwrap().get_conn().prep_exec(
            prep_q, interests).map(|result| {
               result.map(|x| x.unwrap()).map(|row| {
               let (video_id, interest, url, length_sec, order) = mysql::from_row(row);
               AdvertVideoOrder{video_id, interest, url, length_sec, order}
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
    }

    trait PrintErr {
        fn print_err_get_mess<T>(&self) -> Result<T, String>;
    }

    impl PrintErr for mysql::error::Error {   
        fn print_err_get_mess<T>(&self) -> Result<T, String> {
            eprintln!("ERROR: {}", &self);
            panic!("{}", &self);
            //Err(format!("{}", &self))
        }
    }

    impl PrintErr for String {
        fn print_err_get_mess<T>(&self) -> Result<T, String> {
            eprintln!("ERROR: {}", &self);
            panic!("{}", &self);
            //Err(format!("{}", &self))
        }
    }
}