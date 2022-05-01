use crate::controllers::{GamesControl, NewGame};
use crate::sql_connector::Connector;
use crate::errors::ServerError;
use crate::DBConnection;
use diesel::data_types::{PgDate, PgMoney};
use rocket::serde::Serialize;
use rocket::form::{self, Form, Contextual, FromForm, Error, FromFormField};
use rocket::http::Status;
use chrono::{NaiveDate, Datelike};
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct CustomContext<'a, T: Serialize> {
    values: Vec<T>,
    table: &'a str,
    errors: Vec<String>,
    messages: Vec<String>
}


#[derive(Debug, FromForm)]
pub struct AddGame {
    name: String,
    genre: String,
    release_date: String,
    prime_cost: f64,
    publisher_id: i32, 
    cost: f64, 
    is_subscribable: bool,
}

impl NewGame {
    fn from(game: AddGame) -> Result<Self, ServerError> {
        let release_date = NaiveDate::parse_from_str(&game.release_date, "%Y-%m-%d");
        if release_date.is_err() {
            return Err(ServerError::InvalidDate);
        }
        let release_date = NaiveDate::from_ymd(
            release_date.unwrap().year() - 1999,
            release_date.unwrap().month(),
            release_date.unwrap().day()
        );

        Ok(NewGame {
            name: game.name, 
            genre: game.genre,
            release_date: PgDate(release_date.num_days_from_ce()),
            prime_cost: PgMoney((game.prime_cost * 100f64) as i64),
            publisher_id: game.publisher_id,
            cost: PgMoney((game.cost * 100f64) as i64),
            is_subscribable: game.is_subscribable
        })
    }
}

#[derive(Debug, FromForm)]
pub struct DeleteGame {
    id: i32
}


#[derive(Debug, FromForm)]
pub struct GamesForm<'a> {
    add: form::Result<'a, AddGame>,
    delete: form::Result<'a, DeleteGame>
}


#[get("/games")]
pub async fn games() -> Template {
    let games_ctx = CustomContext {
        values: GamesControl::get_games().unwrap(),
        table: "Games",
        errors: vec![],
        messages: vec![]
    };
    
    Template::render("games", games_ctx)
}

#[post("/games", data = "<form>")]
pub async fn games_submit<'r>(conn: DBConnection, mut form: Form<Contextual<'r, GamesForm<'r>>>) -> (Status, Template) {
    let method = form.context.field_value("submit_button").unwrap();
    let mut msgs = Vec::new();
    let mut errs = Vec::new();
    let value = std::mem::replace(&mut form.value, None);
    let template = match method {
        "Add" => {
            let add = value.unwrap().add;
            match add {
                Err(errors) => {
                    let names = errors
                        .iter()
                        .map(|err| {
                            let name = err.name.as_ref().unwrap().to_string();
                            name.rsplit_once('.')
                                .unwrap().1
                                .replace("_", " ")
                                .to_string()
                        }).collect();
                    errs.push(ServerError::NullValues(names).to_string())
                }, 
                Ok(game) => {
                    let game = NewGame::from(game);
                    if let Err(err) = game {
                        errs.push(err.to_string());
                    } else {
                        if let Some(err) = GamesControl::add_game(conn, game.unwrap()).await.err() {
                            errs.push(err.to_string());
                        } else {
                            msgs.push("Ok".to_string());
                        }
                    }
                }
            }

            let games_ctx = CustomContext {
                values: GamesControl::get_games().unwrap(),
                table: "Games",
                errors: errs,
                messages: msgs
            };
            
            Template::render("games", games_ctx)
        },
        
        "Delete" => {
            if value.is_none() {
                errs.push(String::from("There is no value"));
            } else {
                let delete = value.unwrap().delete;

                match delete {
                    Err(errors) => {
                        errors.iter().for_each(|err| {
                            let name = err.name.as_ref().unwrap();
                            errs.push(name.to_string());
                        });
                    },
                    Ok(value) => {
                        if let Some(val) = GamesControl::delete_game(conn, value.id).await.err() {
                            errs.push(val.to_string());
                        } else {
                            msgs.push("Ok".to_string());
                        }
                    }
                }
            }

            let games_ctx = CustomContext {
                values: GamesControl::get_games().unwrap(),
                table: "Games",
                errors: errs,
                messages: msgs
            };

            Template::render("games", games_ctx)
        },
        _ => {
            let games_ctx = CustomContext {
                values: GamesControl::get_games().unwrap(),
                table: "Games",
                errors: vec!["Ой-йой, щось пішло не так!".to_string()],
                messages: vec![]
            };
            
            Template::render("games", games_ctx)
        }
    };

    println!("{:#?}", form);
    (form.context.status(), template)
}
