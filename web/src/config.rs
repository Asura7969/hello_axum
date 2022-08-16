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
    // pub redis: Option<redis::Client>,
}

impl AppState {
    pub async fn creat_db_pool(db_url: &str,
                               max_connections: u32) -> Pool<MySql> {
        MySqlPoolOptions::new()
            .max_connections(max_connections)
            .connect(db_url)
            .await.expect("初始化数据库连接池失败")
    }

    // pub async fn creat_redis(params: &'static str) -> redis::Client {
    //     redis::Client::open(params).expect("redis 客户端创建失败！")
    // }
}


pub async fn creat_db_pool() -> Result<Pool<MySql>, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:123456@localhost/skeleton")
        .await?;

    Ok(pool)
}
