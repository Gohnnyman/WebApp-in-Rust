mod sql_server;

use sql_server::SqlServer;
use async_std::task;

fn main() {

    let mut buffer = String::new();

    let mut sql_server: SqlServer = 
        match task::block_on(SqlServer::new("GameStudio", "SuperDB228")) {
            Ok(t) => t,
            Err(f) => {panic!("{:?}", f);}
        };


        
        
    while true {
        std::io::stdin().read_line(&mut buffer).expect("Failed to read from stdin!");
        let rslt = task::block_on(async {
            match buffer.trim().to_lowercase().as_str() {
                "select games" => sql_server.get_games().await,
                "select investors" => sql_server.get_investors().await,
                "select donations" => sql_server.get_donations().await,
                "select investor_game" => sql_server.get_investor_game().await,
                "select jobs" => sql_server.get_jobs().await,
                "select publishers" => sql_server.get_publishers().await,
                "select staff" => sql_server.get_staff().await,
                _ => Ok(())
            }
        });
    }
    

}
