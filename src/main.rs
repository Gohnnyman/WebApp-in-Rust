#[macro_use]
extern crate diesel;
#[macro_use] 
extern crate rocket;

use dotenv::dotenv;
use requests_handler::*;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

mod sql_connector;
mod models;
mod schema;
mod controllers;
mod requests_handler;



#[launch]
fn start() -> _ {
    dotenv().ok();

    // config.address = std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 5));
    rocket::build()
        .mount("/", routes![games, games_submit])
        .mount("/", FileServer::from(relative!("front/static")))
        .attach(Template::fairing())
}

