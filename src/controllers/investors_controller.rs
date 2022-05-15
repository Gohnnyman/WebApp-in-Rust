use crate::controllers::*;
use crate::errors::ServerError;
use crate::models::*;
use crate::requests_handler::AddInvestor;
use crate::schema::investors;
use crate::DBConnection;
use anyhow::Result;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "investors"]
pub struct NewInvestor {
    pub name: String,
    pub is_company: bool,
}

impl NewInvestor {
    pub fn from(investor: AddInvestor) -> Result<Self, ServerError> {
        Ok(NewInvestor {
            name: investor.name,
            is_company: investor.is_company,
        })
    }
}
#[derive(Serialize, Debug)]
pub struct InvestorsControl {
    pub id: i32,
    pub name: String,
    pub is_company: bool,
}

impl std::convert::From<Investor> for InvestorsControl {
    fn from(investors_struct: Investor) -> Self {
        InvestorsControl {
            id: investors_struct.id,
            name: investors_struct.name,
            is_company: investors_struct.is_company,
        }
    }
}

impl InvestorsControl {
    pub async fn get_statistic(
        conn: &DBConnection,
        id_for_lookup: i32,
    ) -> (i32, Vec<InvestmentsControl>) {
        (
            id_for_lookup,
            InvestorsControl::get_investments(conn, id_for_lookup).await,
        )
    }

    pub async fn get_investments(
        conn: &DBConnection,
        id_for_lookup: i32,
    ) -> Vec<InvestmentsControl> {
        use crate::schema::investments;
        use crate::schema::investors::dsl::*;

        let table = conn
            .run(move |sql_conn| -> Vec<(Investor, Investment)> {
                investors
                    .filter(id.eq(id_for_lookup))
                    .inner_join(investments::table)
                    .load(sql_conn)
                    .unwrap()
            })
            .await;

        let mut vec = Vec::new();
        for (_, investment) in table {
            vec.push(InvestmentsControl::make_investments_control(conn, investment).await);
        }

        vec
    }

    pub async fn get_investors(conn: &DBConnection) -> Result<Vec<InvestorsControl>> {
        use crate::schema::investors::dsl::*;

        let results = conn
            .run(move |sql_conn| -> Result<Vec<Investor>> {
                Ok(investors.order(id.asc()).load::<Investor>(sql_conn)?)
            })
            .await?;

        Ok(results.into_iter().map(InvestorsControl::from).collect())
    }

    pub async fn get_investor_by_id(
        conn: &DBConnection,
        id_for_lookup: i32,
    ) -> Result<InvestorsControl> {
        use crate::schema::investors::dsl::*;

        conn.run(move |sql_conn| -> Result<InvestorsControl> {
            let result: Investor = investors
                .filter(id.eq(id_for_lookup))
                .first(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(InvestorsControl::from(result))
        })
        .await
    }

    pub async fn add_investor(conn: &DBConnection, investor: NewInvestor) -> Result<()> {
        use crate::schema::investors::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::insert_into(investors)
                .values(&investor)
                .get_result::<Investor>(sql_connection)
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

    pub async fn update_investor(
        conn: &DBConnection,
        id_for_update: i32,
        investor: NewInvestor,
    ) -> Result<()> {
        use crate::schema::investors::dsl::*;

        conn.run(move |sql_connection| -> Result<()> {
            diesel::update(investors.filter(&id.eq(id_for_update)))
                .set((name.eq(investor.name), is_company.eq(investor.is_company)))
                .get_result::<Investor>(sql_connection)
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

    pub async fn delete_investor(conn: &DBConnection, id_for_delete: i32) -> Result<()> {
        use crate::schema::investors::dsl::*;

        conn.run(move |sql_conn| -> Result<()> {
            diesel::delete(investors)
                .filter(&id.eq(id_for_delete))
                .get_result::<Investor>(sql_conn)
                .map_err(|_| ServerError::InvalidValue(vec!["Id".to_string()]))?;
            Ok(())
        })
        .await
    }
}
