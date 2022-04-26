use serde::Serialize;
use crate::schema::games::dsl::*;
use crate::sql_connector::Connector;
use crate::models::*;
use anyhow::Result;
use diesel::prelude::*;

#[derive(Serialize)]
pub struct GamesControl {
    pub id: i32,
    pub name: String,
    pub genre: String,
    pub release_date: i32,
    pub prime_cost: i64,
    pub publisher_id: Option<i32>,
    pub cost: i64,
    pub is_subscribable: bool
}

impl GamesControl {
    pub fn from(games_struct: Games) -> Self {
        GamesControl { 
            id: games_struct.id,
            name: games_struct.name,
            genre: games_struct.genre,
            release_date: games_struct.release_date.0,
            prime_cost: games_struct.prime_cost.0,
            publisher_id: games_struct.publisher_id,
            cost: games_struct.cost.0,
            is_subscribable: games_struct.is_subscribable
        }
    }

    pub fn get_games() -> Result<Vec<GamesControl>> {
        let sql_connector = Connector::new(std::env::var("DATABASE_URL")?)?;
        let results = games.load::<Games>(&sql_connector.connection)?;

        Ok(results.iter().map(|result| 
                GamesControl::from(result.clone())
            ).collect())
    }
}