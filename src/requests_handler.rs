use crate::controllers::GamesControl;
use crate::sql_connector::Connector;
use rocket::serde::Serialize;
use rocket::form::{self, Form, Contextual, FromForm, Context, Error, FromFormField};
use rocket::http::{Status, ContentType};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[derive(Serialize)]
struct CustomContext<'a, T: Serialize> {
    values: Vec<T>,
    table: &'a str,
    errors: Vec<String>,
    messages: Vec<String>
}


#[derive(Debug, FromForm)]
pub struct AddGame {
 #[field(validate = omits('@'))]
    name: String,
    genre: String,
    release_date: String,
    prime_cost: i32,
    publisher_id: i32, 
    cost: i32, 
    is_subscribable: bool,
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
pub async fn games_submit<'r>(form: Form<Contextual<'r, GamesForm<'r>>>) -> (Status, Template) {
    let template = match form.context.field_value("submit_button").unwrap() {
        "Add" => {
            let games_ctx = CustomContext {
                values: GamesControl::get_games().unwrap(),
                table: "Games",
                errors: vec![],
                messages: vec!["Ok".to_string()]
            };
            
    
            Template::render("games", games_ctx)
        },
        
        "Delete" => {
            let games_ctx = CustomContext {
                values: GamesControl::get_games().unwrap(),
                table: "Games",
                errors: vec![],
                messages: vec!["Ok".to_string()]
            };

            Template::render("games", games_ctx)
        },
        _ => {
            let games_ctx = CustomContext {
                values: GamesControl::get_games().unwrap(),
                table: "Games",
                errors: vec![],
                messages: vec!["Ok".to_string()]
            };

            Template::render("games", games_ctx)
        }
    };
            
    (form.context.status(), template)
}