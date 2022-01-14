use std::future::Future;

use crate::{db, graphql::SubjectArea};

use clap::{Parser, Subcommand};

pub fn run<Func, Fut>(token_to_id_function: Func) -> crate::Result<()>
where
    Func: Fn(String) -> Fut + 'static + Send + Sync + Clone,
    Fut: Future<Output = std::result::Result<String, Box<dyn std::error::Error>>> + 'static + Send,
{
    let args = Args::parse();

    actix_web::rt::System::new().block_on(async move {
        match args.command {
            Command::Sync => {
                let pool = db::pool().await?;
                for area in SubjectArea::iter_all() {
                    let schema_exists = db::schema_exists(area, &pool).await?;
                    if !schema_exists {
                        db::init_schema(area, &pool).await?;
                    }
                }
                Ok(())
            }
            Command::Serve => crate::serve(token_to_id_function).await,
        }
    })
}

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    // TODO: Do we delete data if a thing was removed from the config, or force them to do it
    // manually as an additional check.
    /// Sync the database with any changes made in the config
    Sync,
    /// Serve the API
    Serve,
}
