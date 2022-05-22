use std::sync::Arc;

use crate::FuncReturn;

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) token_to_id_function: Arc<dyn Fn(String) -> FuncReturn + Send + Sync>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl Default for Config {
    fn default() -> Self {
        unreachable!("No ID extractor specified");
    }
}
