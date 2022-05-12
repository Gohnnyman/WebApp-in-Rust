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
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                games,
                games_delete_post,
                games_edit,
                games_edit_post,
                games_add,
                games_add_post,
                publishers,
                publishers_delete_post,
                publishers_edit,
                publishers_edit_post,
                publishers_add,
                publishers_add_post,
                investors,
                investors_delete_post,
                investors_edit,
                investors_edit_post,
                investors_add,
                investors_add_post,
                staff,
                staff_delete_post,
                staff_edit,
                staff_edit_post,
                staff_add,
                staff_add_post,
                users,
                users_delete_post,
                users_edit,
                users_edit_post,
                users_add,
                users_add_post,
                donations,
                donations_delete_post,
                donations_edit,
                donations_edit_post,
                donations_add,
                donations_add_post,
                jobs,
                jobs_delete_post,
                jobs_edit,
                jobs_edit_post,
                jobs_add,
                jobs_add_post,
                investments,
                investments_delete_post,
                investments_edit,
                investments_edit_post,
                investments_add,
                investments_add_post,
            ],
        )
        .mount("/", FileServer::from(relative!("front/static")))
        .attach(Template::fairing())
        .attach(DBConnection::fairing())
}
