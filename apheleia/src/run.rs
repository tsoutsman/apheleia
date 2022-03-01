pub fn run<Func, Fut>(token_to_id_function: Func) -> crate::Result<()>
where
    Func: Fn(String) -> Fut + 'static + Send + Sync + Clone,
    Fut: std::future::Future<Output = std::result::Result<String, Box<dyn std::error::Error>>>
        + 'static
        + Send,
{
    actix_web::rt::System::new().block_on(async move { crate::serve(token_to_id_function).await })
}
