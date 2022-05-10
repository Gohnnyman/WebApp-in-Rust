use crate::controllers::{GamesControl, NewGame, NewPublisher, PublishersControl};
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
pub async fn games_add() -> Template {
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
