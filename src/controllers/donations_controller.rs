use crate::controllers::GamesControl;
use crate::controllers::UsersControl;
use crate::errors::ServerError;
use crate::models::*;
use crate::requests_handler::AddDonation;
use crate::schema::donations;
use crate::DBConnection;
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::pg::data_types::PgMoney;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "donations"]
pub struct NewDonation {
    pub user_id: i32,
    pub game_id: i32,
    pub amount: PgMoney,
    pub donation_time: NaiveDateTime,
}

impl NewDonation {
    pub fn from(donation: AddDonation) -> Result<Self, ServerError> {
        let donation_time =
            NaiveDateTime::parse_from_str(&donation.donation_time, "%Y-%m-%dT%H:%M");
        if donation_time.is_err() {
            return Err(ServerError::InvalidDate);
        }

        Ok(NewDonation {
            game_id: donation.game_id,
            user_id: donation.user_id,
            amount: PgMoney((donation.amount * 100f64) as i64),
            donation_time: donation_time.unwrap(),
        })
    }
}
#[derive(Serialize, Debug)]
pub struct DonationsControl {
    pub id: i32,
    pub user: String,
    pub user_id: i32,
    pub game: String,
    pub game_id: i32,
    pub amount: f64,
    pub donation_time: String,
}

impl DonationsControl {
    pub async fn make_donations_control(conn: &DBConnection, donations_struct: Donation) -> Self {
        let donation_time = donations_struct.donation_time;

        let game = GamesControl::get_game_by_id(conn, donations_struct.game_id)
            .await
            .unwrap()
            .name;
        let user = UsersControl::get_user_by_id(conn, donations_struct.user_id)
            .await
            .unwrap()
            .nickname;

        let donation_time = donation_time.format("%d-%m-%Y, %H:%M").to_string();
        DonationsControl {
            id: donations_struct.id,
            game: game,
            game_id: donations_struct.game_id,
            user: user,
            user_id: donations_struct.user_id,
            donation_time: donation_time,
            amount: donations_struct.amount.0 as f64 / 100f64,
        }
    }

    pub fn change_date_format(&mut self, from: &str, to: &str) -> Result<()> {
        let donation_time = NaiveDateTime::parse_from_str(&self.donation_time, from)?;
        self.donation_time = donation_time.format(to).to_string();
        Ok(())
    }

    pub async fn get_donations(conn: &DBConnection) -> Result<Vec<DonationsControl>> {
        use crate::schema::donations::dsl::*;

        let results = conn
            .run(move |sql_conn| -> Result<Vec<Donation>> {
                Ok(donations.order(id.asc()).load::<Donation>(sql_conn)?)
            })
            .await?;

        let mut donations_result: Vec<DonationsControl> = vec![];
        for donation in results {
            donations_result.push(DonationsControl::make_donations_control(conn, donation).await);
        }

        Ok(donations_result)
    }

    pub async fn get_donation_by_id(
        conn: &DBConnection,
        id_for_lookup: i32,
    ) -> Result<DonationsControl> {
        use crate::schema::donations::dsl::*;

        let donation = conn
            .run(move |sql_conn| -> Result<Donation> {
                let result: Donation = donations
                    .filter(id.eq(id_for_lookup))
                    .first(sql_conn)
                    .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
                Ok(result)
            })
            .await?;

        Ok(DonationsControl::make_donations_control(conn, donation).await)
    }

    pub async fn add_donation(conn: &DBConnection, donation: NewDonation) -> Result<()> {
        use crate::schema::donations::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::insert_into(donations)
                .values(&donation)
                .get_result::<Donation>(sql_connection)
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

    pub async fn update_donation(
        conn: &DBConnection,
        id_for_update: i32,
        donation: NewDonation,
    ) -> Result<()> {
        use crate::schema::donations::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::update(donations.filter(&id.eq(id_for_update)))
                .set((
                    game_id.eq(donation.game_id),
                    user_id.eq(donation.user_id),
                    amount.eq(donation.amount),
                    donation_time.eq(donation.donation_time),
                ))
                .get_result::<Donation>(sql_connection)
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

    pub async fn delete_donation(conn: &DBConnection, id_for_delete: i32) -> Result<()> {
        use crate::schema::donations::dsl::*;

        conn.run(move |sql_conn| -> Result<()> {
            diesel::delete(donations)
                .filter(&id.eq(id_for_delete))
                .get_result::<Donation>(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        })
        .await
    }
}
