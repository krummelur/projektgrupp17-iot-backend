#![feature(proc_macro_hygiene, decl_macro)]
/**
 * Interface layer
 */
#[macro_use] extern crate rocket;
#[cfg(test)] mod integration_tests;
extern crate futures;

mod db;
mod environment;
mod model;
mod video;

use rocket::response::status;
use futures::executor::block_on;
use rocket_contrib::json::{JsonValue, Json};
use serde_json::json;
use serde::Deserialize;
use rocket::http::Method;
use rocket_cors;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Guard, Responder};

mod devices;


#[derive(Deserialize)]
struct RegisterBody {
    loc: String,
    tag: String
}

#[catch(404)]
fn not_found() -> JsonValue {
    JsonValue(json!({
        "status": "error",
        "reason": "not found"
    }))
}


#[catch(400)]
fn bad_request() -> JsonValue {
    JsonValue(json!({
        "status": "error",
        "message": "request could not be fullfilled. Check request headers and body format."
    }))
}

#[catch(422)]
fn unproc_request() -> JsonValue {
    JsonValue(json!({
        "status": "error",
        "message": "request could not be processed. Check request headers and body content."
    }))
}

/**
 * Gets the current API version 
 */
#[get("/")]
fn default(cors: Guard<'_>) -> Responder<'_,&str> {
    cors.responder("IoT server v1.0.0")
}

/**
 * Register a certain tracker to a certain location, using json data
 * body: 
 * {
 *  loc: id, 
 *  tag: id
 * }
 *  */
#[post("/register", data = "<body>")]
fn register_json(cors: Guard<'_>, body: Json<RegisterBody>) -> Responder<'_, Option<status::Created<JsonValue>>> {
    register(cors, body.loc.clone(), body.tag.clone())
}

/**
 * Register a certain tracker for a certain station
 * Tracker and station must exist, if not 404 is returned
 */
#[post("/register/<station_id>/<tracker_id>")]
fn register(cors: Guard<'_>, station_id: String, tracker_id: String) ->  Responder<'_, Option<status::Created<JsonValue>>> {
    match block_on(devices::ftr_register_tracker_location(&station_id, &tracker_id)) {
        Ok(()) => 
            cors.responder(Some(status::Created(format!("/trackers/{}", station_id), 
            Some(JsonValue(json!({"status": "registered", "tracker_id": tracker_id})))))),
        Err(e) => {println!("{}",e); cors.responder(None)}
    }
}

/**
 * Unregisters a certain tracker from a certain receiver. If the tracker is not registered to this receiver, nothing happens
 * but a 200 is returned. 
 */
#[post("/unregister/<station_id>/<tracker_id>")]
fn unregister(cors: Guard<'_>, station_id: String, tracker_id: String) ->  Responder<'_, Option<JsonValue>> {
    match block_on(devices::ftr_unregister_tracker_location(&station_id, &tracker_id)) {
        Ok(_) =>  cors.responder(Some(JsonValue(json!({"status": "unregistered", "tracker_id": tracker_id})))),
        Err(e) => {println!("{}",e); cors.responder(None)}
    }
}

/**
 * Registers a view of a specified video_id from a specific display_id
 */
#[post("/views/<display_id>/<video_id>/<order_id>")]
fn register_view(cors: Guard<'_>, display_id: i32, video_id: i32, order_id: i32) -> Responder<'_, Result<JsonValue, Option<status::BadRequest<JsonValue>>>> {
//TODO: The number of credits could be a factor of the playtime and the amount of registered people in at the location    
    match video::register_video_view(display_id, video_id, order_id) {
        Err(None) => cors.responder(Err(None)),
        Err(Some(_)) => cors.responder(Err(Some(status::BadRequest(Some(JsonValue(json!({"status": "error", "message": "could not be fullfilled, check video_id"}))))))),
        Ok(_) => cors.responder(Ok(JsonValue(json!({"status": "success", "message": "video play logged"}))))
    }
}

/**
 * Find out what receiver if any a tracker is registered at.
 */
#[get("/trackers/<tracker_id>")]
fn get_tracker(cors: Guard<'_>, tracker_id: String) ->  Responder<'_,Option<Result<JsonValue, &str>>> {
    println!("{}", tracker_id);
    match db::get_tracker_by_id(&tracker_id) {
        Ok(Some(tr)) => 
        cors.responder(Some(Ok(
            JsonValue(json!({"id": tr.id, "location": tr.location}))))),
        Ok(None) => cors.responder(None),
        Err(e) => {println!("{}", e); cors.responder(Some(Err("Unknown error")))}
    }
}

/**
 * Get the most appropriate video to play on the screen of specified id
 * Appropriateness depends on the trackers currently registered to the reciver, and their interests
 */
#[get("/video/<display_id>")]
fn get_video(cors: Guard<'_>, display_id: i32) -> Responder<'_,Result<JsonValue, Option<status::BadRequest<String>>>> {
    match db::get_display_by_id(display_id) {
        Ok(None) =>  return cors.responder(Err(None)),
        _ => ()
    };
    match video::find_relevant_video(display_id) {
        Err(e) => cors.responder(Err(Some(status::BadRequest(Some(e))))),
        Ok(Some(v)) => cors.responder(Ok(JsonValue(json!({"video": {"url": v.url, "length": v.length_sec, "order": v.order}, "message": "video found"})))),
        Ok(None) => cors.responder(Ok(JsonValue(json!({"video": null, "message": "no trackers registered to location" }))))
    }
}

/**
 *  Program entrypoint, initializes rocket with the public endpoints
 */ 
fn main() {
    check_env();
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![default, register, register_json, get_tracker, get_video, unregister, register_view])
    .mount("/", rocket_cors::catch_all_options_routes())
    .register( catchers![not_found, bad_request, unproc_request])
    .manage(cors())
}

fn cors() -> rocket_cors::Cors {
    let allowed_origins = AllowedOrigins::all();
    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().unwrap()
}

fn check_env() {
    match String::from(environment::PRODUCTION_STRING) == environment::get_current_env() {
        false => colour::yellow!("\n### USING STAGING ENVIRONMENT (not an error) ###\n\n"),
        true =>  colour::dark_red!("\n### WARNING! USING PRODUCTION ENVIRONMENT ###\n\n")
    }
}
