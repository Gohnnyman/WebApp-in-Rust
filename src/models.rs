use diesel::pg::data_types::{PgDate, PgMoney};
use diesel::Queryable;

#[derive(Queryable)]
pub struct Publishers {
    pub id: i32,
    pub name: String,
    pub price: PgMoney,
    pub popularity: i16,
}

#[derive(Queryable, Debug)]
pub struct Games {
    pub id: i32,
    pub name: String,
    pub genre: String,
    pub release_date: PgDate,
    pub prime_cost: PgMoney,
    pub publisher_id: i32,
    pub cost: PgMoney,
    pub is_subscribable: bool,
}

#[derive(Queryable)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub birth: PgDate,
}

#[derive(Queryable)]
pub struct Jobs {
    pub id: i32,
    pub game_id: i32,
    pub staff_id: i32,
    pub position: String,
    pub first_work_day: chrono::NaiveDate,
    pub last_work_day: Option<chrono::NaiveDate>,
    pub salary: PgMoney,
}

#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub nickname: String,
    pub registration_date: PgDate,
}

#[derive(Queryable)]
pub struct Donations {
    pub id: i32,
    pub user_id: i32,
    pub game_id: i32,
    pub amount: PgMoney,
    pub donation_time: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Investors {
    pub id: i32,
    pub name: String,
    pub is_company: bool,
}

#[derive(Queryable)]
pub struct InvestorsGames {
    pub id: i32,
    pub investor_id: i32,
    pub game_id: i32,
    pub share: i16,
    pub invested: PgMoney,
}
