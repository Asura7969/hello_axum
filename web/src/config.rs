use crate::{RestJson, AppError};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{MySql, MySqlConnection, Pool};

// #[derive(Deserialize)]
// pub struct Config {
//
// }


/// 应用状态共享
#[derive(Clone)]
pub struct AppState {
    /// mysql 连接池
    pub pool: Pool<MySql>,
}


pub async fn creat_pool() -> Result<Pool<MySql>, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:123456@localhost/skeleton")
        .await?;

    Ok(pool)
}
