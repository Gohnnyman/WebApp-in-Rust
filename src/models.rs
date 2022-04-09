use diesel::Queryable;
use diesel::pg::data_types::{PgMoney, PgDate, PgTimestamp};

#[derive(Queryable)]
pub struct Publishers {
    id: i32,
    name: String,
    price: PgMoney,
    popularity: i16
}


#[derive(Queryable, Debug)]
pub struct Games {
    id: i32,
    name: String,
    genre: String,
    release_PgDate: PgDate,
    prime_cost: PgMoney,
    publisher_id: Option<i32>,
    cost: PgMoney,
    is_subscribable: bool
}

#[derive(Queryable)]
pub struct Staff {
    id: i32,
    name: String,
    birth: PgDate 
}

#[derive(Queryable)]
pub struct Jobs {
    id: i32,
    game_id: i32,
    staff_id: i32,
    position: String,
    first_work_day: PgDate,
    last_work_day: PgDate,
    salary: PgMoney
}

#[derive(Queryable)]
pub struct Users {
    id: i32,
    nickname: String,
    registration_PgDate: PgDate
}

#[derive(Queryable)]
pub struct Donations {
    id: i32,
    user_id: i32,
    game_id: i32,
    amount: PgMoney,
    donation_time: PgTimestamp 
}

#[derive(Queryable)]
pub struct Investors {
    id: i32,
    name: String,
    is_company: bool
}

#[derive(Queryable)]
pub struct InvestorsGames {
    id: i32,
    investor_id: i32,
    game_id: i32,
    share: i16,
    invested: PgMoney 
}