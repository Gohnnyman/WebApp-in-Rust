#[macro_use] 
extern crate rocket;
#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use requests_handler::*;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};
use rocket_sync_db_pools::{database, diesel as rdiesel};


mod sql_connector;
mod models;
mod schema;
mod controllers;
mod requests_handler;
mod errors;

#[database("gamestudio")]
pub struct DBConnection(rdiesel::PgConnection);




#[launch]
fn start() -> _ {
    dotenv().ok();

    // config.address = std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 5));
    rocket::build()
        .mount("/", routes![games, games_submit])
        .mount("/", FileServer::from(relative!("front/static")))
        .attach(Template::fairing())
        .attach(DBConnection::fairing())
}

