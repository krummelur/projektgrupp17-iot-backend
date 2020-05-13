#![feature(proc_macro_hygiene, decl_macro)]
/**
 * Interface layer
 * Magnus Fredriksson
 */
#[macro_use] extern crate rocket;
#[cfg(test)]
mod integration_tests;
extern crate futures;

mod db;
mod environment;
mod model;
mod video;
mod devices;
mod endpoints;

use rocket::http::Method;
use rocket_cors;
use rocket::routes;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

/**
 *  Program entrypoint, initializes rocket with the public endpoints
 */ 
fn main() {
    check_env();
    rocket().launch();
}


#[derive(Default)]
struct ResponsePostProcessor {}

impl Fairing for ResponsePostProcessor {
    fn info(&self) -> Info {
        Info {
            name: "Allow origin header",
            kind: Kind::Request | Kind::Response
        }
    }
    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_raw_header("Access-Control-Allow-Origin", "*");
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .attach(ResponsePostProcessor{})
    .mount("/", routes![
        endpoints::default, 
        endpoints::log_message,
        endpoints::iot_devices::register, 
        endpoints::iot_devices::register_json, 
        endpoints::iot_devices::get_tracker, 
        endpoints::iot_devices::unregister, 
        endpoints::iot_devices::unregister_json, 
        endpoints::videos::register_view,
        endpoints::videos::get_video]) 
    .register( catchers![
        endpoints::catchers::not_found, 
        endpoints::catchers::bad_request, 
        endpoints::catchers::unproc_request])
    .mount("/", rocket_cors::catch_all_options_routes())
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
