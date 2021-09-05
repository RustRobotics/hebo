// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use hebo::connectors::redis_conn::{RedisConn, RedisConnConfig};
use hebo::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config = RedisConnConfig::default();
    let mut r = RedisConn::new(&config)?;
    r.init().await?;
    let mut conn = r.conn().expect("redis connection is none");
    redis::cmd("SET")
        .arg("my_key")
        .arg(42)
        .query_async(&mut conn)
        .await?;
    let ret = redis::cmd("GET").arg("my_key").query_async(&mut conn).await;
    log::info!("ret: {:?}", ret);
    assert_eq!(ret, Ok(42i32));

    Ok(())
}
