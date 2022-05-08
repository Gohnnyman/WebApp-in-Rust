#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use requests_handler::*;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::{database, diesel as rdiesel};

mod controllers;
mod errors;
mod models;
mod requests_handler;
mod schema;

#[database("gamestudio")]
pub struct DBConnection(rdiesel::PgConnection);

#[launch]
fn start() -> _ {
    dotenv().ok();

    // config.address = std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 5));
    rocket::build()
        .mount(
            "/",
            routes![games, games_delete_post, games_edit, games_edit_post, games_add, games_add_post],
        )
        .mount("/", FileServer::from(relative!("front/static")))
        .attach(Template::fairing())
        .attach(DBConnection::fairing())
}
