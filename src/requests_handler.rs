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
