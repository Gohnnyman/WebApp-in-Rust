use crate::controllers::GamesControl;
use crate::controllers::StaffControl;
use crate::errors::ServerError;
use crate::models::*;
use crate::requests_handler::AddJob;
use crate::schema::jobs;
use crate::DBConnection;
use anyhow::Result;
use chrono::NaiveDate;
use diesel::pg::data_types::PgMoney;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "jobs"]
pub struct NewJob {
    pub game_id: i32,
    pub staff_id: i32,
    pub position: String,
    pub first_work_day: NaiveDate,
    pub last_work_day: Option<NaiveDate>,
    pub salary: PgMoney,
}

impl NewJob {
    pub fn from(job: AddJob) -> Result<Self, ServerError> {
        let first_work_day = NaiveDate::parse_from_str(&job.first_work_day, "%Y-%m-%d");

        if first_work_day.is_err() {
            return Err(ServerError::InvalidDate);
        }

        let last_work_day = NaiveDate::parse_from_str(&job.last_work_day, "%Y-%m-%d");

        if last_work_day.is_err() && job.last_work_day != "" {
            return Err(ServerError::InvalidDate);
        }

        let last_work_day = last_work_day.ok();

        Ok(NewJob {
            game_id: job.game_id,
            staff_id: job.staff_id,
            position: job.position,
            first_work_day: first_work_day.unwrap(),
            last_work_day: last_work_day,
            salary: PgMoney((job.salary * 100f64) as i64),
        })
    }
}
#[derive(Serialize, Debug)]
pub struct JobsControl {
    pub id: i32,
    pub game: String,
    pub game_id: i32,
    pub staff: String,
    pub staff_id: i32,
    pub position: String,
    pub first_work_day: String,
    pub last_work_day: String,
    pub salary: f64,
}

impl JobsControl {
    pub async fn make_jobs_control(conn: &DBConnection, jobs_struct: Job) -> Self {
        let first_work_day = jobs_struct.first_work_day;
        let last_work_day = jobs_struct.last_work_day;

        let game = GamesControl::get_game_by_id(conn, jobs_struct.game_id)
            .await
            .unwrap()
            .name;
        let staff = StaffControl::get_staff_by_id(conn, jobs_struct.staff_id)
            .await
            .unwrap()
            .name;

        let first_work_day = first_work_day.format("%d-%m-%Y").to_string();
        let last_work_day = if last_work_day.is_some() {
            last_work_day.unwrap().format("%d-%m-%Y").to_string()
        } else {
            "".to_string()
        };
        JobsControl {
            id: jobs_struct.id,
            game: game,
            game_id: jobs_struct.game_id,
            staff: staff,
            staff_id: jobs_struct.staff_id,
            position: jobs_struct.position,
            first_work_day,
            last_work_day,
            salary: jobs_struct.salary.0 as f64 / 100f64,
        }
    }

    pub fn change_date_format(&mut self, from: &str, to: &str) -> Result<()> {
        let first_work_day = NaiveDate::parse_from_str(&self.first_work_day, from)?;
        self.first_work_day = first_work_day.format(to).to_string();

        if self.last_work_day != "" {
            let last_work_day = NaiveDate::parse_from_str(&self.last_work_day, from)?;
            self.last_work_day = last_work_day.format(to).to_string();
        }
        Ok(())
    }

    pub async fn get_jobs(conn: &DBConnection) -> Result<Vec<JobsControl>> {
        use crate::schema::jobs::dsl::*;

        let results = conn
            .run(move |sql_conn| -> Result<Vec<Job>> {
                Ok(jobs.order(id.asc()).load::<Job>(sql_conn)?)
            })
            .await?;

        let mut jobs_result: Vec<JobsControl> = vec![];
        for job in results {
            jobs_result.push(JobsControl::make_jobs_control(conn, job).await);
        }

        Ok(jobs_result)
    }

    pub async fn get_job_by_id(conn: &DBConnection, id_for_lookup: i32) -> Result<JobsControl> {
        use crate::schema::jobs::dsl::*;

        let job = conn
            .run(move |sql_conn| -> Result<Job> {
                let result: Job = jobs
                    .filter(id.eq(id_for_lookup))
                    .first(sql_conn)
                    .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
                Ok(result)
            })
            .await?;

        Ok(JobsControl::make_jobs_control(conn, job).await)
    }

    pub async fn add_job(conn: &DBConnection, job: NewJob) -> Result<()> {
        use crate::schema::jobs::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::insert_into(jobs)
                .values(&job)
                .get_result::<Job>(sql_connection)
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

    pub async fn update_job(conn: &DBConnection, id_for_update: i32, job: NewJob) -> Result<()> {
        use crate::schema::jobs::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::update(jobs.filter(&id.eq(id_for_update)))
                .set((
                    game_id.eq(job.game_id),
                    staff_id.eq(job.staff_id),
                    position.eq(job.position),
                    first_work_day.eq(job.first_work_day),
                    last_work_day.eq(job.last_work_day),
                    salary.eq(job.salary),
                ))
                .get_result::<Job>(sql_connection)
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

    pub async fn delete_job(conn: &DBConnection, id_for_delete: i32) -> Result<()> {
        use crate::schema::jobs::dsl::*;

        conn.run(move |sql_conn| -> Result<()> {
            diesel::delete(jobs)
                .filter(&id.eq(id_for_delete))
                .get_result::<Job>(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        })
        .await
    }
}
