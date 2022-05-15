use crate::controllers::*;
use crate::errors::ServerError;
use crate::models::*;
use crate::requests_handler::AddStaff;
use crate::schema::staff;
use crate::DBConnection;
use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use diesel::pg::data_types::PgDate;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "staff"]
pub struct NewStaff {
    pub name: String,
    pub birth: PgDate,
}

impl NewStaff {
    pub fn from(staff: AddStaff) -> Result<Self, ServerError> {
        let birth = NaiveDate::parse_from_str(&staff.birth, "%Y-%m-%d");
        if birth.is_err() {
            return Err(ServerError::InvalidDate);
        }
        let birth = NaiveDate::from_ymd(
            birth.unwrap().year() - 1999,
            birth.unwrap().month(),
            birth.unwrap().day(),
        );

        Ok(NewStaff {
            name: staff.name,
            birth: PgDate(birth.num_days_from_ce()),
        })
    }
}
#[derive(Serialize, Debug)]
pub struct StaffControl {
    pub id: i32,
    pub name: String,
    pub birth: String,
}

impl std::convert::From<Staff> for StaffControl {
    fn from(staff_struct: Staff) -> Self {
        let birth = NaiveDate::from_num_days_from_ce(staff_struct.birth.0);
        let birth = NaiveDate::from_ymd(birth.year() + 1999, birth.month(), birth.day());
        let birth = birth.format("%d-%m-%Y").to_string();
        StaffControl {
            id: staff_struct.id,
            name: staff_struct.name,
            birth: birth,
        }
    }
}

impl StaffControl {
    pub async fn get_statistic(conn: &DBConnection, id_for_lookup: i32) -> (i32, Vec<JobsControl>) {
        (
            id_for_lookup,
            StaffControl::get_jobs(conn, id_for_lookup).await,
        )
    }

    pub async fn get_jobs(conn: &DBConnection, id_for_lookup: i32) -> Vec<JobsControl> {
        use crate::schema::jobs;
        use crate::schema::staff::dsl::*;

        let table = conn
            .run(move |sql_conn| -> Vec<(Staff, Job)> {
                staff
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
    pub fn change_date_format(&mut self, from: &str, to: &str) -> Result<()> {
        let birth = NaiveDate::parse_from_str(&self.birth, from)?;
        self.birth = birth.format(to).to_string();
        Ok(())
    }

    pub async fn get_staff(conn: &DBConnection) -> Result<Vec<StaffControl>> {
        use crate::schema::staff::dsl::*;

        let results = conn
            .run(move |sql_conn| -> Result<Vec<Staff>> {
                Ok(staff.order(id.asc()).load::<Staff>(sql_conn)?)
            })
            .await?;

        Ok(results.into_iter().map(StaffControl::from).collect())
    }

    pub async fn get_staff_by_id(conn: &DBConnection, id_for_lookup: i32) -> Result<StaffControl> {
        use crate::schema::staff::dsl::*;

        conn.run(move |sql_conn| -> Result<StaffControl> {
            let result: Staff = staff
                .filter(id.eq(id_for_lookup))
                .first(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(StaffControl::from(result))
        })
        .await
    }

    pub async fn add_staff(conn: &DBConnection, new_staff: NewStaff) -> Result<()> {
        use crate::schema::staff::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::insert_into(staff)
                .values(&new_staff)
                .get_result::<Staff>(sql_connection)
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

    pub async fn update_staff(
        conn: &DBConnection,
        id_for_update: i32,
        new_staff: NewStaff,
    ) -> Result<()> {
        use crate::schema::staff::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::update(staff.filter(&id.eq(id_for_update)))
                .set((name.eq(new_staff.name), birth.eq(new_staff.birth)))
                .get_result::<Staff>(sql_connection)
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

    pub async fn delete_staff(conn: &DBConnection, id_for_delete: i32) -> Result<()> {
        use crate::schema::staff::dsl::*;

        conn.run(move |sql_conn| -> Result<()> {
            diesel::delete(staff)
                .filter(&id.eq(id_for_delete))
                .get_result::<Staff>(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        })
        .await
    }
}
