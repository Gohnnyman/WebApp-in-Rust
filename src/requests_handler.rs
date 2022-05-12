use crate::controllers::*;
use crate::errors::ServerError;
use crate::DBConnection;
use anyhow::Result;
use rocket::form::{self, Contextual, Form, FromForm};
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct CustomContext<'a, T: Serialize> {
    values: Vec<T>,
    table: &'a str,
    errors: Vec<String>,
    content: Vec<Vec<String>>,
}

#[derive(Debug, FromForm)]
pub struct AddGame {
    pub name: String,
    pub genre: String,
    pub release_date: String,
    pub prime_cost: f64,
    pub publisher_id: i32,
    pub cost: f64,
    pub is_subscribable: bool,
}

#[derive(Debug, FromForm)]
pub struct GamesForm<'f> {
    add: form::Result<'f, AddGame>,
}

#[derive(Debug, FromForm)]
pub struct AddPublisher {
    pub name: String,
    pub price: f64,
    pub popularity: i16,
}

#[derive(Debug, FromForm)]
pub struct PublishersForm<'f> {
    add: form::Result<'f, AddPublisher>,
}

#[derive(Debug, FromForm)]
pub struct AddInvestor {
    pub name: String,
    pub is_company: bool,
}

#[derive(Debug, FromForm)]
pub struct InvestorsForm<'f> {
    add: form::Result<'f, AddInvestor>,
}

#[derive(Debug, FromForm)]
pub struct AddStaff {
    pub name: String,
    pub birth: String,
}

#[derive(Debug, FromForm)]
pub struct StaffForm<'f> {
    add: form::Result<'f, AddStaff>,
}

#[derive(Debug, FromForm)]
pub struct AddUser {
    pub nickname: String,
    pub registration_date: String,
}

#[derive(Debug, FromForm)]
pub struct UsersForm<'f> {
    add: form::Result<'f, AddUser>,
}

#[derive(Debug, FromForm)]
pub struct AddDonation {
    pub user_id: i32,
    pub game_id: i32,
    pub amount: f64,
    pub donation_time: String,
}

#[derive(Debug, FromForm)]
pub struct DonationsForm<'f> {
    add: form::Result<'f, AddDonation>,
}

#[derive(Debug, FromForm)]
pub struct AddJob {
    pub game_id: i32,
    pub staff_id: i32,
    pub position: String,
    pub first_work_day: String,
    pub last_work_day: String,
    pub salary: f64,
}

#[derive(Debug, FromForm)]
pub struct JobsForm<'f> {
    add: form::Result<'f, AddJob>,
}

#[get("/games")]
pub async fn games(conn: DBConnection) -> Template {
    let ctx = CustomContext {
        values: GamesControl::get_games(&conn).await.unwrap(),
        table: "Games",
        errors: vec![],
        content: vec![],
    };

    Template::render("games", ctx)
}

#[get("/games/add")]
pub async fn games_add(conn: DBConnection) -> Template {
    let publishers = PublishersControl::get_publishers(&conn).await.unwrap();
    let publishers_id = publishers
        .iter()
        .map(|publisher| publisher.id.to_string())
        .collect();

    let publishers_name = publishers
        .into_iter()
        .map(|publisher| publisher.name)
        .collect();

    let ctx = CustomContext::<String> {
        values: vec![],
        table: "Games",
        errors: vec![],
        content: vec![publishers_id, publishers_name],
    };

    Template::render("games_add", ctx)
}

#[post("/games/add", data = "<form>")]
pub async fn games_add_post<'r>(
    conn: DBConnection,
    mut form: Form<Contextual<'r, GamesForm<'r>>>,
) -> Result<Redirect, Template> {
    let game = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match game {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(game) => {
            let game = NewGame::from(game);
            if let Err(err) = game {
                errs.push(err.to_string());
            } else {
                if let Some(err) = GamesControl::add_game(&conn, game.unwrap()).await.err() {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let ctx = CustomContext::<String> {
            values: vec![],
            table: "Games",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("games_add", ctx))
    } else {
        Ok(Redirect::to(uri!(games)))
    }
}

#[get("/games/edit?<id>")]
pub async fn games_edit<'r>(conn: DBConnection, id: i32) -> Template {
    let mut game = GamesControl::get_game_by_id(&conn, id).await.unwrap();
    game.change_date_format("%d-%m-%Y", "%Y-%m-%d").unwrap();
    let publishers = PublishersControl::get_publishers(&conn).await.unwrap();
    let publishers_id = publishers
        .iter()
        .map(|publisher| publisher.id.to_string())
        .collect();

    let publishers_name = publishers
        .into_iter()
        .map(|publisher| publisher.name)
        .collect();

    let ctx = CustomContext {
        values: vec![game],
        table: "Games",
        errors: vec![],
        content: vec![publishers_id, publishers_name],
    };

    Template::render("games_edit", ctx)
}

#[post("/games/edit?<id>", data = "<form>")]
pub async fn games_edit_post<'r>(
    conn: DBConnection,
    id: i32,
    mut form: Form<Contextual<'r, GamesForm<'r>>>,
) -> Result<Redirect, Template> {
    let game = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match game {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(game) => {
            let game = NewGame::from(game);
            if let Err(err) = game {
                errs.push(err.to_string());
            } else {
                if let Some(err) = GamesControl::update_game(&conn, id, game.unwrap())
                    .await
                    .err()
                {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let mut game = GamesControl::get_game_by_id(&conn, id).await.unwrap();
        game.change_date_format("%d-%m-%Y", "%Y-%m-%d").unwrap();
        let ctx = CustomContext {
            values: vec![game],
            table: "Games",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("games_edit", ctx))
    } else {
        Ok(Redirect::to(uri!(games)))
    }
}

#[post("/games/delete?<id>")]
pub async fn games_delete_post<'r>(conn: DBConnection, id: i32) -> Result<Redirect, Template> {
    GamesControl::delete_game(&conn, id).await.unwrap();

    Ok(Redirect::to(uri!(games)))
}

#[get("/publishers")]
pub async fn publishers(conn: DBConnection) -> Template {
    let ctx = CustomContext {
        values: PublishersControl::get_publishers(&conn).await.unwrap(),
        table: "Publishers",
        errors: vec![],
        content: vec![],
    };

    Template::render("publishers", ctx)
}

#[get("/publishers/add")]
pub async fn publishers_add() -> Template {
    let ctx = CustomContext::<String> {
        values: vec![],
        table: "Publishers",
        errors: vec![],
        content: vec![],
    };

    Template::render("publishers_add", ctx)
}

#[post("/publishers/add", data = "<form>")]
pub async fn publishers_add_post<'r>(
    conn: DBConnection,
    mut form: Form<Contextual<'r, PublishersForm<'r>>>,
) -> Result<Redirect, Template> {
    let publisher = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match publisher {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(publisher) => {
            let publisher = NewPublisher::from(publisher);
            if let Err(err) = publisher {
                errs.push(err.to_string());
            } else {
                if let Some(err) = PublishersControl::add_publisher(&conn, publisher.unwrap())
                    .await
                    .err()
                {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let ctx = CustomContext::<String> {
            values: vec![],
            table: "Publishers",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("publishers_add", ctx))
    } else {
        Ok(Redirect::to(uri!(publishers)))
    }
}

#[get("/publishers/edit?<id>")]
pub async fn publishers_edit<'r>(conn: DBConnection, id: i32) -> Template {
    let publisher = PublishersControl::get_publisher_by_id(&conn, id)
        .await
        .unwrap();

    let ctx = CustomContext {
        values: vec![publisher],
        table: "Publishers",
        errors: vec![],
        content: vec![],
    };

    Template::render("publishers_edit", ctx)
}

#[post("/publishers/edit?<id>", data = "<form>")]
pub async fn publishers_edit_post<'r>(
    conn: DBConnection,
    id: i32,
    mut form: Form<Contextual<'r, PublishersForm<'r>>>,
) -> Result<Redirect, Template> {
    let publisher = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match publisher {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(publisher) => {
            let publisher = NewPublisher::from(publisher);
            if let Err(err) = publisher {
                errs.push(err.to_string());
            } else {
                if let Some(err) =
                    PublishersControl::update_publisher(&conn, id, publisher.unwrap())
                        .await
                        .err()
                {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let publisher = PublishersControl::get_publisher_by_id(&conn, id)
            .await
            .unwrap();
        let ctx = CustomContext {
            values: vec![publisher],
            table: "Publishers",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("publishers_edit", ctx))
    } else {
        Ok(Redirect::to(uri!(publishers)))
    }
}

#[post("/publishers/delete?<id>")]
pub async fn publishers_delete_post<'r>(conn: DBConnection, id: i32) -> Result<Redirect, Template> {
    PublishersControl::delete_publisher(&conn, id)
        .await
        .unwrap();

    Ok(Redirect::to(uri!(publishers)))
}

#[get("/investors")]
pub async fn investors(conn: DBConnection) -> Template {
    let ctx = CustomContext {
        values: InvestorsControl::get_investors(&conn).await.unwrap(),
        table: "Investors",
        errors: vec![],
        content: vec![],
    };

    Template::render("investors", ctx)
}

#[get("/investors/add")]
pub async fn investors_add() -> Template {
    let ctx = CustomContext::<String> {
        values: vec![],
        table: "Investors",
        errors: vec![],
        content: vec![],
    };

    Template::render("investors_add", ctx)
}

#[post("/investors/add", data = "<form>")]
pub async fn investors_add_post<'r>(
    conn: DBConnection,
    mut form: Form<Contextual<'r, InvestorsForm<'r>>>,
) -> Result<Redirect, Template> {
    let investor = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match investor {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(investor) => {
            let investor = NewInvestor::from(investor);
            if let Err(err) = investor {
                errs.push(err.to_string());
            } else {
                if let Some(err) = InvestorsControl::add_investor(&conn, investor.unwrap())
                    .await
                    .err()
                {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let ctx = CustomContext::<String> {
            values: vec![],
            table: "Investors",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("investors_add", ctx))
    } else {
        Ok(Redirect::to(uri!(investors)))
    }
}

#[get("/investors/edit?<id>")]
pub async fn investors_edit<'r>(conn: DBConnection, id: i32) -> Template {
    let investor = InvestorsControl::get_investor_by_id(&conn, id)
        .await
        .unwrap();
    let ctx = CustomContext {
        values: vec![investor],
        table: "Investors",
        errors: vec![],
        content: vec![],
    };

    Template::render("investors_edit", ctx)
}

#[post("/investors/edit?<id>", data = "<form>")]
pub async fn investors_edit_post<'r>(
    conn: DBConnection,
    id: i32,
    mut form: Form<Contextual<'r, InvestorsForm<'r>>>,
) -> Result<Redirect, Template> {
    let investor = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match investor {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(investor) => {
            let investor = NewInvestor::from(investor);
            if let Err(err) = investor {
                errs.push(err.to_string());
            } else {
                if let Some(err) = InvestorsControl::update_investor(&conn, id, investor.unwrap())
                    .await
                    .err()
                {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let investor = InvestorsControl::get_investor_by_id(&conn, id)
            .await
            .unwrap();
        let ctx = CustomContext {
            values: vec![investor],
            table: "Investors",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("investors_edit", ctx))
    } else {
        Ok(Redirect::to(uri!(investors)))
    }
}

#[post("/investors/delete?<id>")]
pub async fn investors_delete_post<'r>(conn: DBConnection, id: i32) -> Result<Redirect, Template> {
    InvestorsControl::delete_investor(&conn, id).await.unwrap();

    Ok(Redirect::to(uri!(investors)))
}

#[get("/staff")]
pub async fn staff(conn: DBConnection) -> Template {
    let ctx = CustomContext {
        values: StaffControl::get_staff(&conn).await.unwrap(),
        table: "Staff",
        errors: vec![],
        content: vec![],
    };

    Template::render("staff", ctx)
}

#[get("/staff/add")]
pub async fn staff_add() -> Template {
    let ctx = CustomContext::<String> {
        values: vec![],
        table: "Staff",
        errors: vec![],
        content: vec![],
    };

    Template::render("staff_add", ctx)
}

#[post("/staff/add", data = "<form>")]
pub async fn staff_add_post<'r>(
    conn: DBConnection,
    mut form: Form<Contextual<'r, StaffForm<'r>>>,
) -> Result<Redirect, Template> {
    let staff = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match staff {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(staff) => {
            let staff = NewStaff::from(staff);
            if let Err(err) = staff {
                errs.push(err.to_string());
            } else {
                if let Some(err) = StaffControl::add_staff(&conn, staff.unwrap()).await.err() {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let ctx = CustomContext::<String> {
            values: vec![],
            table: "Staff",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("staff_add", ctx))
    } else {
        Ok(Redirect::to(uri!(staff)))
    }
}

#[get("/staff/edit?<id>")]
pub async fn staff_edit<'r>(conn: DBConnection, id: i32) -> Template {
    let mut staff = StaffControl::get_staff_by_id(&conn, id).await.unwrap();
    staff.change_date_format("%d-%m-%Y", "%Y-%m-%d").unwrap();

    let ctx = CustomContext {
        values: vec![staff],
        table: "Staff",
        errors: vec![],
        content: vec![],
    };

    Template::render("staff_edit", ctx)
}

#[post("/staff/edit?<id>", data = "<form>")]
pub async fn staff_edit_post<'r>(
    conn: DBConnection,
    id: i32,
    mut form: Form<Contextual<'r, StaffForm<'r>>>,
) -> Result<Redirect, Template> {
    let staff = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match staff {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(staff) => {
            let staff = NewStaff::from(staff);
            if let Err(err) = staff {
                errs.push(err.to_string());
            } else {
                if let Some(err) = StaffControl::update_staff(&conn, id, staff.unwrap())
                    .await
                    .err()
                {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let mut staff = StaffControl::get_staff_by_id(&conn, id).await.unwrap();
        staff.change_date_format("%d-%m-%Y", "%Y-%m-%d").unwrap();
        let ctx = CustomContext {
            values: vec![staff],
            table: "Staff",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("staff_edit", ctx))
    } else {
        Ok(Redirect::to(uri!(staff)))
    }
}

#[post("/staff/delete?<id>")]
pub async fn staff_delete_post<'r>(conn: DBConnection, id: i32) -> Result<Redirect, Template> {
    StaffControl::delete_staff(&conn, id).await.unwrap();

    Ok(Redirect::to(uri!(staff)))
}

#[get("/users")]
pub async fn users(conn: DBConnection) -> Template {
    let ctx = CustomContext {
        values: UsersControl::get_users(&conn).await.unwrap(),
        table: "Users",
        errors: vec![],
        content: vec![],
    };

    Template::render("users", ctx)
}

#[get("/users/add")]
pub async fn users_add() -> Template {
    let ctx = CustomContext::<String> {
        values: vec![],
        table: "Users",
        errors: vec![],
        content: vec![],
    };

    Template::render("users_add", ctx)
}

#[post("/users/add", data = "<form>")]
pub async fn users_add_post<'r>(
    conn: DBConnection,
    mut form: Form<Contextual<'r, UsersForm<'r>>>,
) -> Result<Redirect, Template> {
    let users = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match users {
        Err(errors) => {
            let nicknames = errors
                .iter()
                .map(|err| {
                    let nickname = err.name.as_ref().unwrap().to_string();
                    nickname
                        .rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(nicknames).to_string());
        }
        Ok(users) => {
            let users = NewUser::from(users);
            if let Err(err) = users {
                errs.push(err.to_string());
            } else {
                if let Some(err) = UsersControl::add_user(&conn, users.unwrap()).await.err() {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let ctx = CustomContext::<String> {
            values: vec![],
            table: "Users",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("users_add", ctx))
    } else {
        Ok(Redirect::to(uri!(users)))
    }
}

#[get("/users/edit?<id>")]
pub async fn users_edit<'r>(conn: DBConnection, id: i32) -> Template {
    let mut users = UsersControl::get_user_by_id(&conn, id).await.unwrap();
    users.change_date_format("%d-%m-%Y", "%Y-%m-%d").unwrap();

    let ctx = CustomContext {
        values: vec![users],
        table: "Users",
        errors: vec![],
        content: vec![],
    };

    Template::render("users_edit", ctx)
}

#[post("/users/edit?<id>", data = "<form>")]
pub async fn users_edit_post<'r>(
    conn: DBConnection,
    id: i32,
    mut form: Form<Contextual<'r, UsersForm<'r>>>,
) -> Result<Redirect, Template> {
    let users = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match users {
        Err(errors) => {
            let nicknames = errors
                .iter()
                .map(|err| {
                    let nickname = err.name.as_ref().unwrap().to_string();
                    nickname
                        .rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(nicknames).to_string());
        }
        Ok(users) => {
            let users = NewUser::from(users);
            if let Err(err) = users {
                errs.push(err.to_string());
            } else {
                if let Some(err) = UsersControl::update_user(&conn, id, users.unwrap())
                    .await
                    .err()
                {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let mut users = UsersControl::get_user_by_id(&conn, id).await.unwrap();
        users.change_date_format("%d-%m-%Y", "%Y-%m-%d").unwrap();
        let ctx = CustomContext {
            values: vec![users],
            table: "Users",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("users_edit", ctx))
    } else {
        Ok(Redirect::to(uri!(users)))
    }
}

#[post("/users/delete?<id>")]
pub async fn users_delete_post<'r>(conn: DBConnection, id: i32) -> Result<Redirect, Template> {
    UsersControl::delete_users(&conn, id).await.unwrap();

    Ok(Redirect::to(uri!(users)))
}

#[get("/donations")]
pub async fn donations(conn: DBConnection) -> Template {
    let ctx = CustomContext {
        values: DonationsControl::get_donations(&conn).await.unwrap(),
        table: "Donations",
        errors: vec![],
        content: vec![],
    };

    Template::render("donations", ctx)
}

#[get("/donations/add")]
pub async fn donations_add(conn: DBConnection) -> Template {
    let users = UsersControl::get_users(&conn).await.unwrap();
    let users_id = users.iter().map(|user| user.id.to_string()).collect();

    let users_name = users.into_iter().map(|user| user.nickname).collect();

    let games = GamesControl::get_games(&conn).await.unwrap();
    let games_id = games.iter().map(|game| game.id.to_string()).collect();

    let games_name = games.into_iter().map(|game| game.name).collect();

    let ctx = CustomContext::<String> {
        values: vec![],
        table: "Donations",
        errors: vec![],
        content: vec![users_id, users_name, games_id, games_name],
    };

    Template::render("donations_add", ctx)
}

#[post("/donations/add", data = "<form>")]
pub async fn donations_add_post<'r>(
    conn: DBConnection,
    mut form: Form<Contextual<'r, DonationsForm<'r>>>,
) -> Result<Redirect, Template> {
    let donation = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match donation {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(donation) => {
            let donation = NewDonation::from(donation);
            if let Err(err) = donation {
                errs.push(err.to_string());
            } else {
                if let Some(err) = DonationsControl::add_donation(&conn, donation.unwrap())
                    .await
                    .err()
                {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let ctx = CustomContext::<String> {
            values: vec![],
            table: "Donations",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("donations_add", ctx))
    } else {
        Ok(Redirect::to(uri!(donations)))
    }
}

#[get("/donations/edit?<id>")]
pub async fn donations_edit<'r>(conn: DBConnection, id: i32) -> Template {
    let mut donation = DonationsControl::get_donation_by_id(&conn, id)
        .await
        .unwrap();
    donation
        .change_date_format("%d-%m-%Y, %H:%M", "%Y-%m-%dT%H:%M")
        .unwrap();
    let users = UsersControl::get_users(&conn).await.unwrap();
    let users_id = users.iter().map(|user| user.id.to_string()).collect();

    let users_name = users.into_iter().map(|user| user.nickname).collect();

    let games = GamesControl::get_games(&conn).await.unwrap();
    let games_id = games.iter().map(|game| game.id.to_string()).collect();

    let games_name = games.into_iter().map(|game| game.name).collect();

    let ctx = CustomContext {
        values: vec![donation],
        table: "Donations",
        errors: vec![],
        content: vec![users_id, users_name, games_id, games_name],
    };

    Template::render("donations_edit", ctx)
}

#[post("/donations/edit?<id>", data = "<form>")]
pub async fn donations_edit_post<'r>(
    conn: DBConnection,
    id: i32,
    mut form: Form<Contextual<'r, DonationsForm<'r>>>,
) -> Result<Redirect, Template> {
    let donation = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match donation {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(donation) => {
            let donation = NewDonation::from(donation);
            if let Err(err) = donation {
                errs.push(err.to_string());
            } else {
                if let Some(err) = DonationsControl::update_donation(&conn, id, donation.unwrap())
                    .await
                    .err()
                {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let mut donation = DonationsControl::get_donation_by_id(&conn, id)
            .await
            .unwrap();
        donation.change_date_format("%d-%m-%Y", "%Y-%m-%d").unwrap();
        let ctx = CustomContext {
            values: vec![donation],
            table: "Donations",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("donations_edit", ctx))
    } else {
        Ok(Redirect::to(uri!(donations)))
    }
}

#[post("/donations/delete?<id>")]
pub async fn donations_delete_post<'r>(conn: DBConnection, id: i32) -> Result<Redirect, Template> {
    DonationsControl::delete_donation(&conn, id).await.unwrap();

    Ok(Redirect::to(uri!(donations)))
}

#[get("/jobs")]
pub async fn jobs(conn: DBConnection) -> Template {
    let ctx = CustomContext {
        values: JobsControl::get_jobs(&conn).await.unwrap(),
        table: "Jobs",
        errors: vec![],
        content: vec![],
    };

    Template::render("jobs", ctx)
}

#[get("/jobs/add")]
pub async fn jobs_add(conn: DBConnection) -> Template {
    let staff = StaffControl::get_staff(&conn).await.unwrap();
    let staff_id = staff.iter().map(|user| user.id.to_string()).collect();

    let staff_name = staff.into_iter().map(|user| user.name).collect();

    let games = GamesControl::get_games(&conn).await.unwrap();
    let games_id = games.iter().map(|game| game.id.to_string()).collect();

    let games_name = games.into_iter().map(|game| game.name).collect();

    let ctx = CustomContext::<String> {
        values: vec![],
        table: "Jobs",
        errors: vec![],
        content: vec![games_id, games_name, staff_id, staff_name],
    };

    Template::render("jobs_add", ctx)
}

#[post("/jobs/add", data = "<form>")]
pub async fn jobs_add_post<'r>(
    conn: DBConnection,
    mut form: Form<Contextual<'r, JobsForm<'r>>>,
) -> Result<Redirect, Template> {
    let job = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match job {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(job) => {
            let job = NewJob::from(job);
            if let Err(err) = job {
                errs.push(err.to_string());
            } else {
                if let Some(err) = JobsControl::add_job(&conn, job.unwrap()).await.err() {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let ctx = CustomContext::<String> {
            values: vec![],
            table: "Jobs",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("jobs_add", ctx))
    } else {
        Ok(Redirect::to(uri!(jobs)))
    }
}

#[get("/jobs/edit?<id>")]
pub async fn jobs_edit<'r>(conn: DBConnection, id: i32) -> Template {
    let mut job = JobsControl::get_job_by_id(&conn, id).await.unwrap();
    job.change_date_format("%d-%m-%Y", "%Y-%m-%d")
        .unwrap();
    let staff = StaffControl::get_staff(&conn).await.unwrap();
    let staff_id = staff.iter().map(|user| user.id.to_string()).collect();

    let staff_name = staff.into_iter().map(|user| user.name).collect();

    let games = GamesControl::get_games(&conn).await.unwrap();
    let games_id = games.iter().map(|game| game.id.to_string()).collect();

    let games_name = games.into_iter().map(|game| game.name).collect();

    let ctx = CustomContext {
        values: vec![job],
        table: "Jobs",
        errors: vec![],
        content: vec![games_id, games_name, staff_id, staff_name],
    };

    Template::render("jobs_edit", ctx)
}

#[post("/jobs/edit?<id>", data = "<form>")]
pub async fn jobs_edit_post<'r>(
    conn: DBConnection,
    id: i32,
    mut form: Form<Contextual<'r, JobsForm<'r>>>,
) -> Result<Redirect, Template> {
    let job = std::mem::replace(&mut form.value, None).unwrap().add;
    let mut errs = Vec::new();

    match job {
        Err(errors) => {
            let names = errors
                .iter()
                .map(|err| {
                    let name = err.name.as_ref().unwrap().to_string();
                    name.rsplit_once('.')
                        .unwrap()
                        .1
                        .replace("_", " ")
                        .to_string()
                })
                .collect();
            errs.push(ServerError::NullValues(names).to_string());
        }
        Ok(job) => {
            let job = NewJob::from(job);
            if let Err(err) = job {
                errs.push(err.to_string());
            } else {
                if let Some(err) = JobsControl::update_job(&conn, id, job.unwrap()).await.err() {
                    errs.push(err.to_string());
                }
            }
        }
    }

    if !errs.is_empty() {
        let mut job = JobsControl::get_job_by_id(&conn, id).await.unwrap();
        job.change_date_format("%d-%m-%Y", "%Y-%m-%d").unwrap();
        let ctx = CustomContext {
            values: vec![job],
            table: "Jobs",
            errors: errs,
            content: vec![],
        };
        Err(Template::render("jobs_edit", ctx))
    } else {
        Ok(Redirect::to(uri!(jobs)))
    }
}

#[post("/jobs/delete?<id>")]
pub async fn jobs_delete_post<'r>(conn: DBConnection, id: i32) -> Result<Redirect, Template> {
    JobsControl::delete_job(&conn, id).await.unwrap();

    Ok(Redirect::to(uri!(jobs)))
}
