
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;

//
pub fn establish_connection() -> MysqlConnection {
    // mysql://user:pass@host/dbname
    let database_url = "mysql://db_user:password@127.0.0.1/vue1";

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
