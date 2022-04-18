use diesel::Queryable;
use diesel::pg::data_types::{PgMoney, PgDate, PgTimestamp};
use serde::Serialize;

#[derive(Queryable)]
pub struct Publishers {
    id: i32,
    name: String,
    price: PgMoney,
    popularity: i16
}


#[derive(Queryable, Debug, Clone)]
pub struct Games {
    pub id: i32,
    pub name: String,
    pub genre: String,
    pub release_date: PgDate,
    pub prime_cost: PgMoney,
    pub publisher_id: Option<i32>,
    pub cost: PgMoney,
    pub is_subscribable: bool
}

#[derive(Serialize)]
pub struct GamesSer {
    pub id: i32,
    pub name: String,
    pub genre: String,
    pub release_date: i32,
    pub prime_cost: i64,
    pub publisher_id: Option<i32>,
    pub cost: i64,
    pub is_subscribable: bool
}

impl GamesSer {
    pub fn from(games: Games) -> Self {
        GamesSer { 
            id: games.id,
            name: games.name,
            genre: games.genre,
            release_date: games.release_date.0,
            prime_cost: games.prime_cost.0,
            publisher_id: games.publisher_id,
            cost: games.cost.0,
            is_subscribable: games.is_subscribable
        }
    }
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