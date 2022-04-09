#[macro_use]
extern crate diesel;

use anyhow::Result;
use schema::games::dsl::*;
use models::*;
use dotenv::dotenv;
use sql_connector::Connector;
use diesel::prelude::*;

mod sql_connector;
mod models;
mod schema;

fn main() -> Result<()> {
    dotenv().ok();

    let sq_connector = Connector::new(std::env::var("DATABASE_URL").unwrap())?;



    
        
        
    
    Ok(())
}
