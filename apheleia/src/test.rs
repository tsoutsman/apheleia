// Adapted from https://github.com/diesel-rs/diesel/issues/1549#issuecomment-892978784

use crate::{
    db::{tokio::AsyncRunQueryDsl, DbPool},
    Result,
};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{sql_query, Connection};
use std::sync::atomic::AtomicU32;

static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

pub(crate) struct TestDbPool {
    name: String,
    pool: DbPool,
    leak: bool,
}

impl TestDbPool {
    pub(crate) async fn new() -> Self {
        let name = format!(
            "test_db_{}_{}",
            std::process::id(),
            TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );

        let default_db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let conn = ConnectionManager::<PgConnection>::new(default_db_url);
        let mut pool = Pool::new(conn).expect("failed to create pool");

        sql_query(format!("CREATE DATABASE {};", name))
            .execute(&mut pool)
            .await
            .unwrap();

        Self {
            name,
            pool,
            leak: false,
        }
    }

    pub(crate) fn pool(&self) -> DbPool {
        self.pool.clone()
    }

    pub(crate) fn leak(&mut self) {
        self.leak = true;
    }
}

impl Drop for TestDbPool {
    fn drop(&mut self) {
        if self.leak {
            log::warn!("leaking test database: {}", self.name);
            return;
        }

        let pool = self.pool.clone();
        let name = self.name.clone();
        tokio::task::block_in_place(move || -> Result<()> {
            let mut conn = pool.get()?;
            conn.execute(&format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
                name
            ))
            .expect("failed to select database to drop");
            Ok(())
        })
        .unwrap();
        tokio::task::block_in_place(move || -> Result<()> {
            let mut conn = self.pool.get()?;
            conn.execute(&format!("DROP DATABASE {}", self.name))?;
            Ok(())
        })
        .expect("failed to drop database");
    }
}
