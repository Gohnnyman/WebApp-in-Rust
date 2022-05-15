use crate::controllers::*;
use crate::errors::ServerError;
use crate::models::*;
use crate::requests_handler::AddPublisher;
use crate::schema::publishers;
use crate::DBConnection;
use anyhow::Result;
use diesel::pg::data_types::PgMoney;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "publishers"]
pub struct NewPublisher {
    pub name: String,
    pub price: PgMoney,
    pub popularity: i16,
}

impl NewPublisher {
    pub fn from(publisher: AddPublisher) -> Result<Self, ServerError> {
        Ok(NewPublisher {
            name: publisher.name,
            price: PgMoney((publisher.price * 100f64) as i64),
            popularity: publisher.popularity,
        })
    }
}

#[derive(Serialize)]
pub struct PublishersControl {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub popularity: i16,
}

impl std::convert::From<Publisher> for PublishersControl {
    fn from(publishers_struct: Publisher) -> Self {
        PublishersControl {
            id: publishers_struct.id,
            name: publishers_struct.name,
            price: publishers_struct.price.0 as f64 / 100f64,
            popularity: publishers_struct.popularity,
        }
    }
}

impl PublishersControl {
    pub async fn get_statistic(
        conn: &DBConnection,
        id_for_lookup: i32,
    ) -> (i32, Vec<GamesControl>) {
        (
            id_for_lookup,
            PublishersControl::get_games(conn, id_for_lookup).await,
        )
    }

    pub async fn get_games(conn: &DBConnection, id_for_lookup: i32) -> Vec<GamesControl> {
        use crate::schema::games;
        use crate::schema::publishers::dsl::*;

        let table = conn
            .run(move |sql_conn| -> Vec<(Publisher, Game)> {
                publishers
                    .filter(id.eq(id_for_lookup))
                    .inner_join(games::table)
                    .load(sql_conn)
                    .unwrap()
            })
            .await;

        let mut vec = Vec::new();
        for (_, game) in table {
            vec.push(GamesControl::make_games_control(conn, game).await);
        }

        vec
    }
    pub async fn get_publishers(conn: &DBConnection) -> Result<Vec<PublishersControl>> {
        use crate::schema::publishers::dsl::*;

        let results = conn
            .run(move |sql_conn| -> Result<Vec<Publisher>> {
                Ok(publishers.order(id.asc()).load::<Publisher>(sql_conn)?)
            })
            .await?;

        Ok(results.into_iter().map(PublishersControl::from).collect())
    }

    pub async fn get_publisher_by_id(
        conn: &DBConnection,
        id_for_lookup: i32,
    ) -> Result<PublishersControl> {
        use crate::schema::publishers::dsl::*;

        conn.run(move |sql_conn| -> Result<PublishersControl> {
            let result: Publisher = publishers
                .filter(id.eq(id_for_lookup))
                .first(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(PublishersControl::from(result))
        })
        .await
    }

    pub async fn add_publisher(conn: &DBConnection, publisher: NewPublisher) -> Result<()> {
        use crate::schema::publishers::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::insert_into(publishers)
                .values(&publisher)
                .get_result::<Publisher>(sql_connection)
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

    pub async fn update_publisher(
        conn: &DBConnection,
        id_for_update: i32,
        publisher: NewPublisher,
    ) -> Result<()> {
        use crate::schema::publishers::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::update(publishers.filter(&id.eq(id_for_update)))
                .set((
                    name.eq(publisher.name),
                    price.eq(publisher.price),
                    popularity.eq(publisher.popularity),
                ))
                .get_result::<Publisher>(sql_connection)
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

    pub async fn delete_publisher(conn: &DBConnection, id_for_delete: i32) -> Result<()> {
        use crate::schema::publishers::dsl::*;

        conn.run(move |sql_conn| -> Result<()> {
            diesel::delete(publishers)
                .filter(&id.eq(id_for_delete))
                .get_result::<Publisher>(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        })
        .await
    }
}
