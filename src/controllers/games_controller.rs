use crate::controllers::*;
use crate::errors::ServerError;
use crate::models::*;
use crate::requests_handler::AddGame;
use crate::schema::games;
use crate::DBConnection;
use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use diesel::pg::data_types::{PgDate, PgMoney};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "games"]
pub struct NewGame {
    pub name: String,
    pub genre: String,
    pub release_date: PgDate,
    pub prime_cost: PgMoney,
    pub publisher_id: i32,
    pub cost: PgMoney,
    pub is_subscribable: bool,
}

impl NewGame {
    pub fn from(game: AddGame) -> Result<Self, ServerError> {
        let release_date = NaiveDate::parse_from_str(&game.release_date, "%Y-%m-%d");
        if release_date.is_err() {
            return Err(ServerError::InvalidDate);
        }
        let release_date = NaiveDate::from_ymd(
            release_date.unwrap().year() - 1999,
            release_date.unwrap().month(),
            release_date.unwrap().day(),
        );

        Ok(NewGame {
            name: game.name,
            genre: game.genre,
            release_date: PgDate(release_date.num_days_from_ce()),
            prime_cost: PgMoney((game.prime_cost * 100f64) as i64),
            publisher_id: game.publisher_id,
            cost: PgMoney((game.cost * 100f64) as i64),
            is_subscribable: game.is_subscribable,
        })
    }
}
#[derive(Serialize, Debug)]
pub struct GamesControl {
    pub id: i32,
    pub name: String,
    pub genre: String,
    pub release_date: String,
    pub prime_cost: f64,
    pub publisher: String,
    pub publisher_id: i32,
    pub cost: f64,
    pub is_subscribable: bool,
}

impl GamesControl {
    pub async fn get_statistic(
        conn: &DBConnection,
        id_for_lookup: i32,
    ) -> (
        i32,
        Vec<JobsControl>,
        Vec<DonationsControl>,
        Vec<InvestmentsControl>,
    ) {
        (
            id_for_lookup,
            GamesControl::get_game_staff(conn, id_for_lookup).await,
            GamesControl::get_donations(conn, id_for_lookup).await,
            GamesControl::get_investments(conn, id_for_lookup).await,
        )
    }

    pub async fn get_investments(
        conn: &DBConnection,
        id_for_lookup: i32,
    ) -> Vec<InvestmentsControl> {
        use crate::schema::games::dsl::*;
        use crate::schema::investments;

        let table = conn
            .run(move |sql_conn| -> Vec<(Game, Investment)> {
                games
                    .filter(id.eq(id_for_lookup))
                    .inner_join(investments::table)
                    .load(sql_conn)
                    .unwrap()
            })
            .await;

        let mut vec = Vec::new();
        for (_, investment) in table {
            vec.push(InvestmentsControl::make_investments_control(conn, investment).await);
        }

        vec
    }

    pub async fn get_donations(conn: &DBConnection, id_for_lookup: i32) -> Vec<DonationsControl> {
        use crate::schema::donations;
        use crate::schema::games::dsl::*;

        let table = conn
            .run(move |sql_conn| -> Vec<(Game, Donation)> {
                games
                    .filter(id.eq(id_for_lookup))
                    .inner_join(donations::table)
                    .load(sql_conn)
                    .unwrap()
            })
            .await;

        let mut vec = Vec::new();
        for (_, donation) in table {
            vec.push(DonationsControl::make_donations_control(conn, donation).await);
        }

        vec
    }

    pub async fn get_game_staff(conn: &DBConnection, id_for_lookup: i32) -> Vec<JobsControl> {
        use crate::schema::games::dsl::*;
        use crate::schema::jobs;

        let table = conn
            .run(move |sql_conn| -> Vec<(Game, Job)> {
                games
                    .filter(id.eq(id_for_lookup))
                    .inner_join(jobs::table)
                    .load(sql_conn)
                    .unwrap()
            })
            .await;

        let mut vec = Vec::new();
        for (_, job) in table {
            vec.push(JobsControl::make_jobs_control(conn, job).await);
        }

        vec
    }

    pub async fn make_games_control(pool: &DBConnection, games_struct: Game) -> Self {
        let release_date = NaiveDate::from_num_days_from_ce(games_struct.release_date.0);
        let release_date = NaiveDate::from_ymd(
            release_date.year() + 1999,
            release_date.month(),
            release_date.day(),
        );

        let publishers = PublishersControl::get_publisher_by_id(&pool, games_struct.publisher_id)
            .await
            .unwrap();
        let release_date = release_date.format("%d-%m-%Y").to_string();
        GamesControl {
            id: games_struct.id,
            name: games_struct.name,
            genre: games_struct.genre,
            release_date: release_date,
            prime_cost: games_struct.prime_cost.0 as f64 / 100f64,
            publisher: publishers.name,
            publisher_id: games_struct.publisher_id,
            cost: games_struct.cost.0 as f64 / 100f64,
            is_subscribable: games_struct.is_subscribable,
        }
    }
    pub fn change_date_format(&mut self, from: &str, to: &str) -> Result<()> {
        let release_date = NaiveDate::parse_from_str(&self.release_date, from)?;
        self.release_date = release_date.format(to).to_string();
        Ok(())
    }

    pub async fn get_games(conn: &DBConnection) -> Result<Vec<GamesControl>> {
        use crate::schema::games::dsl::*;

        let results = conn
            .run(move |sql_conn| -> Result<Vec<Game>> {
                Ok(games.order(id.asc()).load::<Game>(sql_conn)?)
            })
            .await?;

        let mut games_result: Vec<GamesControl> = vec![];

        for game in results {
            games_result.push(GamesControl::make_games_control(conn, game).await);
        }

        Ok(games_result)
    }

    pub async fn get_game_by_id(conn: &DBConnection, id_for_lookup: i32) -> Result<GamesControl> {
        use crate::schema::games::dsl::*;

        let game = conn
            .run(move |sql_conn| -> Result<Game> {
                let result: Game = games
                    .filter(id.eq(id_for_lookup))
                    .first(sql_conn)
                    .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
                Ok(result)
            })
            .await?;

        Ok(GamesControl::make_games_control(conn, game).await)
    }

    pub async fn add_game(conn: &DBConnection, game: NewGame) -> Result<()> {
        use crate::schema::games::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::insert_into(games)
                .values(&game)
                .get_result::<Game>(sql_connection)
                .map_err(|err| match err {
                    DieselError::DatabaseError(_, info) => {
                        ServerError::InvalidForeignKey(info.message().to_string())
                    }
                    _ => panic!("PREKOL"),
                })?;
            Ok(())
        })
        .await
    }

    pub async fn update_game(conn: &DBConnection, id_for_update: i32, game: NewGame) -> Result<()> {
        use crate::schema::games::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::update(games.filter(&id.eq(id_for_update)))
                .set((
                    name.eq(game.name),
                    genre.eq(game.genre),
                    release_date.eq(game.release_date),
                    prime_cost.eq(game.prime_cost),
                    publisher_id.eq(game.publisher_id),
                    cost.eq(game.cost),
                    is_subscribable.eq(game.is_subscribable),
                ))
                .get_result::<Game>(sql_connection)
                .map_err(|err| match err {
                    DieselError::DatabaseError(_, info) => {
                        ServerError::InvalidForeignKey(info.message().to_string())
                    }
                    _ => panic!("PREKOL"),
                })?;
            Ok(())
        })
        .await
    }

    pub async fn delete_game(conn: &DBConnection, id_for_delete: i32) -> Result<()> {
        use crate::schema::games::dsl::*;

        conn.run(move |sql_conn| -> Result<()> {
            diesel::delete(games)
                .filter(&id.eq(id_for_delete))
                .get_result::<Game>(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        })
        .await
    }
}
