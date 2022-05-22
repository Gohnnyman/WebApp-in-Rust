use crate::controllers::*;
use crate::errors::ServerError;
use crate::models::*;
use crate::requests_handler::AddUser;
use crate::schema::users;
use crate::DBConnection;
use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use diesel::pg::data_types::PgDate;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub nickname: String,
    pub registration_date: PgDate,
}

impl NewUser {
    pub fn from(user: AddUser) -> Result<Self, ServerError> {
        let registration_date = NaiveDate::parse_from_str(&user.registration_date, "%Y-%m-%d");
        if registration_date.is_err() {
            return Err(ServerError::InvalidDate);
        }
        let registration_date = NaiveDate::from_ymd(
            registration_date.unwrap().year() - 1999,
            registration_date.unwrap().month(),
            registration_date.unwrap().day(),
        );

        Ok(NewUser {
            nickname: user.nickname,
            registration_date: PgDate(registration_date.num_days_from_ce()),
        })
    }
}
#[derive(Serialize)]
pub struct UsersControl {
    pub id: i32,
    pub nickname: String,
    pub registration_date: String,
}

impl std::convert::From<User> for UsersControl {
    fn from(users_struct: User) -> Self {
        let registration_date = NaiveDate::from_num_days_from_ce(users_struct.registration_date.0);
        let registration_date = NaiveDate::from_ymd(
            registration_date.year() + 1999,
            registration_date.month(),
            registration_date.day(),
        );
        let registration_date = registration_date.format("%d-%m-%Y").to_string();
        UsersControl {
            id: users_struct.id,
            nickname: users_struct.nickname,
            registration_date: registration_date,
        }
    }
}

impl UsersControl {
    pub async fn get_statistic(
        conn: &DBConnection,
        id_for_lookup: i32,
    ) -> (i32, Vec<DonationsControl>) {
        (
            id_for_lookup,
            UsersControl::get_donations(conn, id_for_lookup).await,
        )
    }

    pub async fn get_donations(conn: &DBConnection, id_for_lookup: i32) -> Vec<DonationsControl> {
        use crate::schema::donations;
        use crate::schema::users::dsl::*;

        let table = conn
            .run(move |sql_conn| -> Vec<(User, Donation)> {
                users
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

    pub fn change_date_format(&mut self, from: &str, to: &str) -> Result<()> {
        let registration_date = NaiveDate::parse_from_str(&self.registration_date, from)?;
        self.registration_date = registration_date.format(to).to_string();
        Ok(())
    }

    pub async fn get_users(conn: &DBConnection) -> Result<Vec<UsersControl>> {
        use crate::schema::users::dsl::*;

        let results = conn
            .run(move |sql_conn| -> Result<Vec<User>> {
                Ok(users.order(id.asc()).load::<User>(sql_conn)?)
            })
            .await?;

        Ok(results.into_iter().map(UsersControl::from).collect())
    }

    pub async fn get_user_by_id(conn: &DBConnection, id_for_lookup: i32) -> Result<UsersControl> {
        use crate::schema::users::dsl::*;

        conn.run(move |sql_conn| -> Result<UsersControl> {
            let result: User = users
                .filter(id.eq(id_for_lookup))
                .first(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(UsersControl::from(result))
        })
        .await
    }

    pub async fn add_user(conn: &DBConnection, user: NewUser) -> Result<()> {
        use crate::schema::users::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::insert_into(users)
                .values(&user)
                .get_result::<User>(sql_connection)
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

    pub async fn update_user(conn: &DBConnection, id_for_update: i32, user: NewUser) -> Result<()> {
        use crate::schema::users::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::update(users.filter(&id.eq(id_for_update)))
                .set((
                    nickname.eq(user.nickname),
                    registration_date.eq(user.registration_date),
                ))
                .get_result::<User>(sql_connection)
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

    pub async fn delete_users(conn: &DBConnection, id_for_delete: i32) -> Result<()> {
        use crate::schema::users::dsl::*;

        conn.run(move |sql_conn| -> Result<()> {
            diesel::delete(users)
                .filter(&id.eq(id_for_delete))
                .get_result::<User>(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        })
        .await
    }
}
