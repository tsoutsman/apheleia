use std::future::Future;

/// Entry point for the server.
///
/// Takes in a single paramater, `token_to_id`, a function which converts an
/// unverified token into some verified ID. For example, an OAuth access token
/// into a user ID.
pub async fn run<F, Fut>(token_to_id: F)
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = Result<String, Box<dyn std::error::Error>>>,
{
    todo!();
}
