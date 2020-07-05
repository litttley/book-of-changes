use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;
pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
//pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn connect() -> MysqlPool {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager).expect("Error")
}
