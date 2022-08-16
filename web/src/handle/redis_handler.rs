

use redis::AsyncCommands;
use axum::{Extension, Json};
use crate::{Result, RestJson, AppState};
use axum::extract::Path;
use crate::handle::user_handler::User;

// redis_cache!("get(key)", method)
// redis_cache!("hash(key,Option(field))", method)
// redis_cache!("list(key)", method)
pub async fn redis_cache(Extension(state): Extension<AppState>,
                         Path(id): Path<i64>) -> Result<RestJson<User>> {

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await?;

    con.set("key1", b"foo").await?;
    redis::cmd("SET")
        .arg(&["key2", "bar"])
        .query_async(&mut con)
        .await?;

    let result = redis::cmd("MGET")
        .arg(&["key1", "key2"])
        .query_async(&mut con)
        .await;

    match result {
        Ok()
    }

    // assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
}
