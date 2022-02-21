
use tiberius::{Client, Config, AuthMethod, Row, QueryItem, numeric::Numeric};
pub use async_std::net::TcpStream;



use chrono::naive::NaiveDate;


pub struct SqlServer {
    client: Client<TcpStream>
}

impl SqlServer {
    pub async fn new(db: &str, password: &str) -> anyhow::Result<SqlServer>{

        let mut config = Config::new();

        config.host("localhost");
        config.port(1433);
        config.database(db);
    
        config.authentication(AuthMethod::sql_server("SA", password));
        config.trust_cert();
    
    
        let tcp = TcpStream::connect(config.get_addr()).await?;
        
        tcp.set_nodelay(true)?;

        Ok(SqlServer {
            client: Client::connect(config, tcp).await?
        })
    }


    pub async fn get_publishers(&mut self) -> anyhow::Result<()>{
        let stream = self.client.simple_query("SELECT * FROM Publishers").await?;
    
    
        let rows = stream.into_first_result().await?;
        for row in rows {
            println!("{} {} {} {}", row.get::<i32, usize>(0).unwrap(),
                row.get::<&str, usize>(1).unwrap(), row.get::<f64, usize>(2).unwrap(),
                row.get::<u8, usize>(3).unwrap());
        }
        Ok(())
    }

    pub async fn get_investors(&mut self) -> anyhow::Result<()> {
        let stream = self.client.simple_query("SELECT * FROM Investors").await?;
    
    
        let rows = stream.into_first_result().await?;
        for row in rows {
            println!("{} {} {}", row.get::<i32, usize>(0).unwrap(),
                row.get::<&str, usize>(1).unwrap(), row.get::<bool, usize>(2).unwrap());
        }
        Ok(())
    }

    pub async fn get_games(&mut self) -> anyhow::Result<()> {
        let stream = self.client.simple_query("SELECT * FROM Games").await?;
    
    
        let rows = stream.into_first_result().await?;
        for row in rows {
            println!("{} {} {} {} {} {} {} {}", row.get::<i32, usize>(0).unwrap(),
                row.get::<&str, usize>(1).unwrap(), row.get::<&str, usize>(2).unwrap(),
                row.get::<NaiveDate, usize>(3).unwrap(), row.get::<f64, usize>(4).unwrap(),
                row.get::<i32, usize>(5).unwrap(), row.get::<f64, usize>(6).unwrap(),
                row.get::<bool, usize>(7).unwrap());
        }
        Ok(())

    }

    pub async fn get_staff(&mut self) -> anyhow::Result<()> {

        let stream = self.client.simple_query("SELECT * FROM Staff").await?;
    
    
        let rows = stream.into_first_result().await?;
        for row in rows {
            println!("{} {} {}", row.get::<i32, usize>(0).unwrap(),
                row.get::<&str, usize>(1).unwrap(), row.get::<NaiveDate, usize>(2).unwrap());
        }
        Ok(())
    }

    pub async fn get_jobs(&mut self) -> anyhow::Result<()> {

        let stream = self.client.simple_query("SELECT * FROM Jobs").await?;
    
    
        let rows = stream.into_first_result().await?;
        for row in rows {
            println!("{} {} {} {} {} {} {}", row.get::<i32, usize>(0).unwrap(),
                row.get::<i32, usize>(1).unwrap(), row.get::<i32, usize>(2).unwrap(),
                row.get::<&str, usize>(3).unwrap(), row.get::<NaiveDate, usize>(4).unwrap(),
                row.get::<NaiveDate, usize>(5).unwrap(), row.get::<f64, usize>(6).unwrap());
        }
        Ok(())
    }

    pub async fn get_donations(&mut self) -> anyhow::Result<()> {

        let stream = self.client.simple_query("SELECT * FROM Donations").await?;
    
    
        let rows = stream.into_first_result().await?;
        for row in rows {
            println!("{} {} {} {}", row.get::<i32, usize>(0).unwrap(),
                row.get::<&str, usize>(1).unwrap(), row.get::<i32, usize>(2).unwrap(),
                row.get::<f64, usize>(3).unwrap());
        }
        Ok(())
    }

    pub async fn get_investor_game(&mut self) -> anyhow::Result<()> {

        let stream = self.client.simple_query("SELECT * FROM Investor_Game").await?;
    
    
        let rows = stream.into_first_result().await?;
        for row in rows {
            println!("{} {} {} {} {}", row.get::<i32, usize>(0).unwrap(),
                row.get::<i32, usize>(1).unwrap(), row.get::<i32, usize>(2).unwrap(),
                row.get::<u8, usize>(3).unwrap(), row.get::<f64, usize>(4).unwrap());
        }
        Ok(())

    }
    
}