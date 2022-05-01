table! {
    donations (id) {
        id -> Int4,
        user_id -> Int4,
        game_id -> Int4,
        amount -> Money,
        donation_time -> Timestamp,
    }
}

table! {
    games (id) {
        id -> Int4,
        name -> Varchar,
        genre -> Varchar,
        release_date -> Date,
        prime_cost -> Money,
        publisher_id -> Int4,
        cost -> Money,
        is_subscribable -> Bool,
    }
}

table! {
    investors (id) {
        id -> Int4,
        name -> Varchar,
        is_company -> Bool,
    }
}

table! {
    investors_games (id) {
        id -> Int4,
        investor_id -> Int4,
        game_id -> Int4,
        share -> Int2,
        invested -> Money,
    }
}

table! {
    jobs (id) {
        id -> Int4,
        game_id -> Int4,
        staff_id -> Int4,
        position -> Varchar,
        first_work_day -> Date,
        last_work_day -> Nullable<Date>,
        salary -> Money,
    }
}

table! {
    publishers (id) {
        id -> Int4,
        name -> Varchar,
        price -> Money,
        popularity -> Int2,
    }
}

table! {
    staff (id) {
        id -> Int4,
        name -> Varchar,
        birth -> Date,
    }
}

table! {
    users (id) {
        id -> Int4,
        nickname -> Varchar,
        registration_date -> Date,
    }
}

joinable!(donations -> games (game_id));
joinable!(donations -> users (user_id));
joinable!(games -> publishers (publisher_id));
joinable!(investors_games -> games (game_id));
joinable!(investors_games -> investors (investor_id));
joinable!(jobs -> games (game_id));
joinable!(jobs -> staff (staff_id));

allow_tables_to_appear_in_same_query!(
    donations,
    games,
    investors,
    investors_games,
    jobs,
    publishers,
    staff,
    users,
);
