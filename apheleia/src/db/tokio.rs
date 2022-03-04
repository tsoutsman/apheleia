//! Adapted from https://github.com/mehcode/tokio-diesel

use crate::{Result};

use async_trait::async_trait;
use diesel::{
    connection::SimpleConnection,
    dsl::Limit,
    query_dsl::{
        methods::{ExecuteDsl, LimitDsl, LoadQuery},
        RunQueryDsl,
    },
    r2d2::{ConnectionManager, Pool, R2D2Connection},
    result::QueryResult,
    Connection,
};
use tokio::task;

#[async_trait]
pub(crate) trait AsyncSimpleConnection<Conn>
where
    Conn: 'static + SimpleConnection,
{
    async fn batch_execute_async(&self, query: &str) -> Result<()>;
}

#[async_trait]
impl<Conn> AsyncSimpleConnection<Conn> for Pool<ConnectionManager<Conn>>
where
    Conn: 'static + Connection + R2D2Connection,
{
    #[inline]
    async fn batch_execute_async(&self, query: &str) -> Result<()> {
        let self_ = self.clone();
        let query = query.to_string();
        task::block_in_place(move || -> Result<()> {
            let mut conn = self_.get()?;
            conn.batch_execute(&query).map_err(|e| e.into())
        })
    }
}

#[async_trait]
pub(crate) trait AsyncConnection<Conn>: AsyncSimpleConnection<Conn>
where
    Conn: 'static + Connection,
{
    async fn run<R, Func>(&self, f: Func) -> Result<R>
    where
        R: Send,
        Func: FnOnce(&mut Conn) -> QueryResult<R> + Send;

    // async fn transaction<R, Func>(&self, f: Func) -> Result<R>
    // where
    //     R: Send,
    //     Func: FnOnce(&mut Conn) -> QueryResult<R> + Send;
}

#[async_trait]
impl<Conn> AsyncConnection<Conn> for Pool<ConnectionManager<Conn>>
where
    Conn: 'static + Connection + R2D2Connection,
{
    #[inline]
    async fn run<R, Func>(&self, f: Func) -> Result<R>
    where
        R: Send,
        Func: FnOnce(&mut Conn) -> QueryResult<R> + Send,
    {
        let self_ = self.clone();
        task::block_in_place(move || -> Result<R> {
            let mut conn = self_.get()?;
            f(&mut conn).map_err(|e| e.into())
        })
    }

    // #[inline]
    // async fn transaction<R, Func>(&self, f: Func) -> Result<R>
    // where
    //     R: Send,
    //     Func: FnOnce(&mut Conn) -> QueryResult<R> + Send,
    // {
    //     let self_ = self.clone();
    //     task::block_in_place(move || -> Result<R> {
    //         let mut conn = self_.get()?;
    //         conn.transaction(|_| f(&mut conn)).map_err(|e| e.into())
    //     })
    // }
}

#[async_trait]
pub(crate) trait AsyncRunQueryDsl<Conn, AsyncConn>
where
    Conn: 'static + Connection,
{
    async fn execute(self, asc: &AsyncConn) -> Result<usize>
    where
        Self: ExecuteDsl<Conn>;

    async fn load<'a, U>(self, asc: &AsyncConn) -> Result<Vec<U>>
    where
        U: Send,
        Self: LoadQuery<'a, Conn, U>;

    async fn get_result<'a, U>(self, asc: &AsyncConn) -> Result<U>
    where
        U: Send,
        Self: LoadQuery<'a, Conn, U>;

    async fn get_results<'a, U>(self, asc: &AsyncConn) -> Result<Vec<U>>
    where
        U: Send,
        Self: LoadQuery<'a, Conn, U>;

    async fn first<'a, U>(self, asc: &AsyncConn) -> Result<U>
    where
        U: Send,
        Self: LimitDsl,
        Limit<Self>: LoadQuery<'a, Conn, U>;
}

#[async_trait]
impl<T, Conn> AsyncRunQueryDsl<Conn, Pool<ConnectionManager<Conn>>> for T
where
    T: Send + RunQueryDsl<Conn>,
    Conn: 'static + Connection + R2D2Connection,
{
    async fn execute(self, asc: &Pool<ConnectionManager<Conn>>) -> Result<usize>
    where
        Self: ExecuteDsl<Conn>,
    {
        asc.run(|conn| self.execute(conn)).await
    }

    async fn load<'a, U>(self, asc: &Pool<ConnectionManager<Conn>>) -> Result<Vec<U>>
    where
        U: Send,
        Self: LoadQuery<'a, Conn, U>,
    {
        asc.run(|conn| self.load(conn)).await
    }

    async fn get_result<'a, U>(self, asc: &Pool<ConnectionManager<Conn>>) -> Result<U>
    where
        U: Send,
        Self: LoadQuery<'a, Conn, U>,
    {
        asc.run(|conn| self.get_result(conn)).await
    }

    async fn get_results<'a, U>(self, asc: &Pool<ConnectionManager<Conn>>) -> Result<Vec<U>>
    where
        U: Send,
        Self: LoadQuery<'a, Conn, U>,
    {
        asc.run(|conn| self.get_results(conn)).await
    }

    async fn first<'a, U>(self, asc: &Pool<ConnectionManager<Conn>>) -> Result<U>
    where
        U: Send,
        Self: LimitDsl,
        Limit<Self>: LoadQuery<'a, Conn, U>,
    {
        asc.run(|conn| self.first(conn)).await
    }
}
