#[macro_use]
extern crate diesel;
#[macro_use] 
extern crate rocket;

use anyhow::Result;
use schema::games::dsl::*;
use models::*;
use dotenv::dotenv;
use sql_connector::Connector;
use diesel::prelude::*;
use rocket::{fs::NamedFile};
use rocket::serde::Serialize;
use rocket_dyn_templates::{Template, tera::Tera};

mod sql_connector;
mod models;
mod schema;


#[derive(Serialize)]
struct Context<T: Serialize> {
    context: Vec<T>
}



#[get("/")]
async fn index() -> Template {
    let sql_connector = Connector::new(std::env::var("DATABASE_URL").unwrap()).unwrap();

    let results = games.filter(is_subscribable.eq(false))
        .limit(5)
        .load::<Games>(&sql_connector.connection)
        .unwrap();

    // results.iter().for_each(|result| {
    //     println!("{:#?}", result);  
    // });


    let gamesser = GamesSer::from(results[0].clone());
    let games_ctx = results.iter().map(|result| GamesSer::from(result.clone())).collect();




    Template::render("index", Context {context: games_ctx})
}



#[launch]
fn start() -> _ {
    dotenv().ok();




    // config.address = std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 5));
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}

