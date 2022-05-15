use diesel::pg::data_types::{PgDate, PgMoney};
use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Publishers {
    pub id: i32,
    pub name: String,
    pub price: PgMoney,
    pub popularity: i16,
}

#[derive(Queryable, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub genre: String,
    pub release_date: PgDate,
    pub prime_cost: PgMoney,
    pub publisher_id: i32,
    pub cost: PgMoney,
    pub is_subscribable: bool,
}

#[derive(Queryable, Debug)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub birth: PgDate,
}

#[derive(Queryable, Debug)]
pub struct Job {
    pub id: i32,
    pub game_id: i32,
    pub staff_id: i32,
    pub position: String,
    pub first_work_day: chrono::NaiveDate,
    pub last_work_day: Option<chrono::NaiveDate>,
    pub salary: PgMoney,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub registration_date: PgDate,
}

#[derive(Queryable)]
pub struct Donation {
    pub id: i32,
    pub user_id: i32,
    pub game_id: i32,
    pub amount: PgMoney,
    pub donation_time: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Investor {
    pub id: i32,
    pub name: String,
    pub is_company: bool,
}

#[derive(Queryable)]
pub struct Investment {
    pub id: i32,
    pub investor_id: i32,
    pub game_id: i32,
    pub share: i16,
    pub invested: PgMoney,
}
