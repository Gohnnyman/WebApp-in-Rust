use crate::controllers::GamesControl;
use crate::controllers::InvestorsControl;
use crate::errors::ServerError;
use crate::models::*;
use crate::requests_handler::AddInvestment;
use crate::schema::investments;
use crate::DBConnection;
use anyhow::Result;
use diesel::pg::data_types::PgMoney;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "investments"]
pub struct NewInvestment {
    pub game_id: i32,
    pub investor_id: i32,
    pub share: i16,
    pub invested: PgMoney,
}

impl NewInvestment {
    pub fn from(investment: AddInvestment) -> Result<Self, ServerError> {
        Ok(NewInvestment {
            game_id: investment.game_id,
            investor_id: investment.investor_id,
            share: investment.share,
            invested: PgMoney((investment.invested * 100f64) as i64),
        })
    }
}
#[derive(Serialize)]
pub struct InvestmentsControl {
    pub id: i32,
    pub game: String,
    pub game_id: i32,
    pub investor: String,
    pub investor_id: i32,
    pub share: i16,
    pub invested: f64,
}

impl InvestmentsControl {
    pub async fn make_investments_control(conn: &DBConnection, investments_struct: Investments) -> Self {
        let game = GamesControl::get_game_by_id(conn, investments_struct.game_id)
            .await
            .unwrap()
            .name;
        let investor = InvestorsControl::get_investor_by_id(conn, investments_struct.investor_id)
            .await
            .unwrap()
            .name;

        InvestmentsControl {
            id: investments_struct.id,
            game: game,
            game_id: investments_struct.game_id,
            investor: investor,
            investor_id: investments_struct.investor_id,
            share: investments_struct.share,
            invested: investments_struct.invested.0 as f64 / 100f64,
        }
    }


    pub async fn get_investments(conn: &DBConnection) -> Result<Vec<InvestmentsControl>> {
        use crate::schema::investments::dsl::*;

        let results = conn
            .run(move |sql_conn| -> Result<Vec<Investments>> {
                Ok(investments.order(id.asc()).load::<Investments>(sql_conn)?)
            })
            .await?;

        let mut investments_result: Vec<InvestmentsControl> = vec![];
        for investment in results {
            investments_result.push(InvestmentsControl::make_investments_control(conn, investment).await);
        }

        Ok(investments_result)
    }

    pub async fn get_investment_by_id(conn: &DBConnection, id_for_lookup: i32) -> Result<InvestmentsControl> {
        use crate::schema::investments::dsl::*;

        let investment = conn
            .run(move |sql_conn| -> Result<Investments> {
                let result: Investments = investments
                    .filter(id.eq(id_for_lookup))
                    .first(sql_conn)
                    .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
                Ok(result)
            })
            .await?;

        Ok(InvestmentsControl::make_investments_control(conn, investment).await)
    }

    pub async fn add_investment(conn: &DBConnection, investment: NewInvestment) -> Result<()> {
        use crate::schema::investments::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::insert_into(investments)
                .values(&investment)
                .get_result::<Investments>(sql_connection)
                .map_err(|err| match err {
                    DieselError::DatabaseError(_, info) => {
                        ServerError::InvalidForeignKey(info.message().to_string())
                    }
                    _ => panic!("PREKOL"),
                })?;
            Ok(())
        })
        .await
    }

    pub async fn update_investment(conn: &DBConnection, id_for_update: i32, investment: NewInvestment) -> Result<()> {
        use crate::schema::investments::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::update(investments.filter(&id.eq(id_for_update)))
                .set((
                    game_id.eq(investment.game_id),
                    investor_id.eq(investment.investor_id),
                    share.eq(investment.share),
                    invested.eq(investment.invested),
                ))
                .get_result::<Investments>(sql_connection)
                .map_err(|err| match err {
                    DieselError::DatabaseError(_, info) => {
                        ServerError::InvalidForeignKey(info.message().to_string())
                    }
                    _ => panic!("PREKOL"),
                })?;
            Ok(())
        })
        .await
    }

    pub async fn delete_investment(conn: &DBConnection, id_for_delete: i32) -> Result<()> {
        use crate::schema::investments::dsl::*;

        conn.run(move |sql_conn| -> Result<()> {
            diesel::delete(investments)
                .filter(&id.eq(id_for_delete))
                .get_result::<Investments>(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        })
        .await
    }
}
