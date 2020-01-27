use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use crate::errors::Result;

type PostgresPool = Pool<ConnectionManager<PgConnection>>;
type PostgresPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref POOL: PostgresPool = {
        let database_url: String = "postgres://root:@db01:26257/gooldbook_example".to_string();
        Pool::builder().build(ConnectionManager::<PgConnection>::new(database_url)).unwrap()
    };
}

pub fn pool_con() -> Result<PostgresPooledConnection>  { POOL.clone().get().map_err(Into::into) }
