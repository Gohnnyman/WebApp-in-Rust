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

impl std::convert::From<Publishers> for PublishersControl {
    fn from(publishers_struct: Publishers) -> Self {
        PublishersControl {
            id: publishers_struct.id,
            name: publishers_struct.name,
            price: publishers_struct.price.0 as f64 / 100f64,
            popularity: publishers_struct.popularity,
        }
    }
}

impl PublishersControl {
    pub async fn get_publishers(conn: &DBConnection) -> Result<Vec<PublishersControl>> {
        use crate::schema::publishers::dsl::*;

        let results = conn
            .run(move |sql_conn| -> Result<Vec<Publishers>> {
                Ok(publishers.order(id.asc()).load::<Publishers>(sql_conn)?)
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
            let result: Publishers = publishers
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
                .get_result::<Publishers>(sql_connection)
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
                .get_result::<Publishers>(sql_connection)
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
                .get_result::<Publishers>(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        })
        .await
    }
}
