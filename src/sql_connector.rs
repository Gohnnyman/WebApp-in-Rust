use diesel::prelude::*;
use anyhow::Result;

pub struct Connector {
    connection: PgConnection
}

impl Connector {
    pub fn new(database_url: String) -> Result<Self> {
        let connector = Connector {
            connection: PgConnection::establish(&database_url).expect("Cannot establish connection")
        };
        Ok(connector)
    }
}