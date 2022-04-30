// Adapted from https://github.com/diesel-rs/diesel/issues/1549#issuecomment-892978784

use crate::{
    auth::{Config, User},
    db::{tokio::AsyncRunQueryDsl, DbPool},
    id::{self, Id},
};

use actix_http::{header, Request};
use actix_web::test::{self, TestRequest};
use diesel::{
    pg::PgConnection,
    query_dsl::methods::ExecuteDsl,
    r2d2::{ConnectionManager, Pool},
    sql_query, Connection,
};
use diesel_migrations::MigrationHarness;
use std::sync::atomic::{AtomicI32, AtomicU32};
use url::Url;

static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);
static USER_COUNTER: AtomicI32 = AtomicI32::new(1);

pub(crate) struct TestDbPool {
    name: String,
    pool: DbPool,
    postgres_url: Url,
    leak: bool,
}

impl TestDbPool {
    pub(crate) async fn new() -> Result<Self, &'static str> {
        let name = format!(
            "test_db_{}_{}",
            std::process::id(),
            TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );

        log::info!("creating test database: {}", name);

        let postgres_url = Url::parse(
            &std::env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL environment variable not set")?,
        )
        .map_err(|_| "failed to parse DATABASE_URL")?;
        let manager = ConnectionManager::<PgConnection>::new(postgres_url.clone());
        let pool = Pool::new(manager).map_err(|_| "failed to create connection pool")?;

        sql_query(format!("CREATE DATABASE {};", name))
            .execute(&pool)
            .await
            .map_err(|_| "failed to create database")?;

        let mut db_url = postgres_url.clone();
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
            postgres_url,
            leak: false,
        })
    }

    pub(crate) fn pool(&self) -> DbPool {
        self.pool.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn leak(&mut self) {
        self.leak = false;
    }
}

impl Drop for TestDbPool {
    fn drop(&mut self) {
        if self.leak || std::thread::panicking() {
            log::warn!("leaking test database: {}", self.name);
            return;
        }

        let mut conn = PgConnection::establish(&self.postgres_url.to_string())
            .expect("failed to connect to database");
        ExecuteDsl::execute(
            sql_query(format!("DROP DATABASE {} WITH (FORCE)", self.name)),
            &mut conn,
        )
        .expect("failed to drop database");
    }
}

pub(crate) fn gen_config() -> crate::auth::Config {
    Config {
        token_to_id_function: std::sync::Arc::new(move |token| -> crate::FuncReturn {
            Box::pin(test_token_to_id(token))
        }),
    }
}

async fn test_token_to_id(token: String) -> Result<u32, Box<dyn std::error::Error>> {
    token.parse().map_err(|e: std::num::ParseIntError| e.into())
}

pub(crate) trait Service:
    actix_service::Service<
    Request,
    Response = actix_web::dev::ServiceResponse<actix_http::body::BoxBody>,
    Error = actix_web::Error,
>
{
}

impl<T> Service for T where
    T: actix_service::Service<
        Request,
        Response = actix_web::dev::ServiceResponse<actix_http::body::BoxBody>,
        Error = actix_web::Error,
    >
{
}

pub(crate) async fn init_test_service() -> (impl Service, TestDbPool) {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Warn)
        .try_init();
    let pool = crate::test::TestDbPool::new()
        .await
        .expect("failed to create db pool");
    (
        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(pool.pool()))
                .app_data(gen_config())
                .app_data(actix_web::web::Data::new(crate::auth::Root(0)))
                .configure(crate::api::config),
        )
        .await,
        pool,
    )
}

pub(crate) async fn create_user<T>(app: &T) -> User
where
    T: Service,
{
    let id = USER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let req = TestRequest::post()
        .uri("/users")
        .insert_header((header::AUTHORIZATION, format!("Bearer {}", id).as_str()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    log::info!("created user with id {}", id);

    User(id)
}

pub(crate) async fn create_subject_area<T, U>(
    app: &T,
    name: U,
    admin_id: User,
) -> Id<id::SubjectArea>
where
    T: Service,
    U: Into<String>,
{
    let req = TestRequest::post()
        .uri("/subject_areas")
        .insert_header((header::AUTHORIZATION, "Bearer 0"))
        .set_json(crate::api::subject_area::AddSubjectArea {
            name: name.into(),
            admin: admin_id,
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    test::read_body_json::<crate::api::subject_area::AddSubjectAreaResponse, _>(resp)
        .await
        .id
}

impl User {
    pub(crate) async fn request<T>(
        &self,
        app: &T,
        request: TestRequest,
    ) -> actix_web::dev::ServiceResponse
    where
        T: Service,
    {
        test::call_service(
            &app,
            request
                .insert_header((header::AUTHORIZATION, format!("Bearer {}", self.0)))
                .to_request(),
        )
        .await
    }
}

// TODO: Test functions in this module
