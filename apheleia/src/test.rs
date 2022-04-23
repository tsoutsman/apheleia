// Adapted from https://github.com/diesel-rs/diesel/issues/1549#issuecomment-892978784

use crate::db::{tokio::AsyncRunQueryDsl, DbPool};

use diesel::{
    pg::PgConnection,
    query_dsl::methods::ExecuteDsl,
    r2d2::{ConnectionManager, Pool},
    sql_query,
};
use diesel_migrations::MigrationHarness;
use std::sync::atomic::AtomicU32;

static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

pub(crate) struct TestDbPool {
    name: String,
    pool: DbPool,
    leak: bool,
}

impl TestDbPool {
    pub(crate) async fn new() -> Result<Self, &'static str> {
        let name = format!(
            "test_db_{}_{}",
            std::process::id(),
            TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );

        let mut db_url = url::Url::parse(
            &std::env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL environment variable not set")?,
        )
        .map_err(|_| "failed to parse DATABASE_URL")?;
        let manager = ConnectionManager::<PgConnection>::new(db_url.clone());
        let pool = Pool::new(manager).map_err(|_| "failed to create connection pool")?;

        sql_query(format!("CREATE DATABASE {};", name))
            .execute(&pool)
            .await
            .map_err(|_| "failed to create database")?;

        db_url.set_path(&name);
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::new(manager).map_err(|_| "failed to create connection pool")?;

        let mut conn = pool.get().expect("failed to connect to database");
        let mut harness = diesel_migrations::HarnessWithOutput::write_to_stdout(&mut conn);
        tokio::task::block_in_place(|| -> Result<(), ()> {
            harness
                .run_pending_migrations(crate::MIGRATIONS)
                .map_err(|_| ())?;
            Ok(())
        })
        .map_err(|_| "failed to run migrations on newly created database")?;
        drop(conn);

        Ok(Self {
            name,
            pool,
            leak: false,
        })
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

        let mut conn = self.pool.get().expect("failed to connect to database");
        ExecuteDsl::execute(
            sql_query(format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
                self.name
            )),
            &mut conn,
        )
        .expect("failed to select database to drop");
        ExecuteDsl::execute(sql_query(format!("DROP DATABASE {}", self.name)), &mut conn)
            .expect("failed to drop database");
    }
}
