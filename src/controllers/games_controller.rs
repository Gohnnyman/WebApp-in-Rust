use serde::Serialize;
use crate::schema::games;
use crate::sql_connector::Connector;
use crate::models::*;
use crate::DBConnection;
use crate::errors::ServerError;
use anyhow::Result;
use diesel::prelude::*;
use diesel::pg::data_types::{PgMoney, PgDate};
use diesel::result::Error as DieselError;
use chrono::{NaiveDate, Datelike};


#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame {
    pub name: String,
    pub genre: String,
    pub release_date: PgDate,
    pub prime_cost: PgMoney,
    pub publisher_id: i32,
    pub cost: PgMoney,
    pub is_subscribable: bool
}

#[derive(Serialize)]
pub struct GamesControl {
    pub id: i32,
    pub name: String,
    pub genre: String,
    pub release_date: String,
    pub prime_cost: f64,
    pub publisher_id: i32,
    pub cost: f64,
    pub is_subscribable: bool
}

impl std::convert::From<Games> for GamesControl {
    fn from(games_struct: Games) -> Self {
        let release_date = NaiveDate::from_num_days_from_ce(games_struct.release_date.0);
        let release_date = NaiveDate::from_ymd(
            release_date.year() + 1999,
            release_date.month(),
            release_date.day()
        );
        let release_date = release_date.format("%Y-%m-%d").to_string();
        GamesControl { 
            id: games_struct.id,
            name: games_struct.name,
            genre: games_struct.genre,
            release_date: release_date,
            prime_cost: games_struct.prime_cost.0 as f64 / 100f64,
            publisher_id: games_struct.publisher_id,
            cost: games_struct.cost.0 as f64 / 100f64,
            is_subscribable: games_struct.is_subscribable
        }
    }
}

impl GamesControl {
    pub fn get_games() -> Result<Vec<GamesControl>> {
        use crate::schema::games::dsl::*;

        let sql_connector = Connector::new(std::env::var("DATABASE_URL")?)?;
        let results = games.load::<Games>(&sql_connector.connection)?;

        Ok(results.iter().map(|result| 
                GamesControl::from(result.clone())
            ).collect())
    }

    pub async fn add_game(conn: DBConnection, game: NewGame) -> Result<()> {
        use crate::schema::games::dsl::*;

        conn.run(move |sql_connection| -> Result<()>{
            diesel::insert_into(games)
                .values(&game)
                .get_result::<Games>(sql_connection)
                .map_err(|err| {
                    match err {
                        DieselError::DatabaseError(_, info) => {
                            ServerError::InvalidForeignKey(info.message().to_string())
                        },
                        _ => panic!("PREKOL")
                    }
                })?;
            Ok(())
        }).await
    }

    pub async fn delete_game(conn: DBConnection, id_for_delete: i32) -> Result<()>{
        use crate::schema::games::dsl::*;

        conn.run(move |sql_connection| -> Result<()>{
            diesel::delete(games)
                .filter(&id.eq(id_for_delete))
                .get_result::<Games>(sql_connection)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        }).await
    }
}