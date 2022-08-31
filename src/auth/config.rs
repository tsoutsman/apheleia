use std::sync::Arc;

use crate::FuncReturn;

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) token_to_id_function: Arc<dyn Fn(String) -> FuncReturn + Send + Sync>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config").finish()
    }
}

impl Default for Config {
    fn default() -> Self {
        unreachable!("No ID extractor specified");
    }
}
