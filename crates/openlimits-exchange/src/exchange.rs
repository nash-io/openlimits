// use messaging::Subscriber;
// use async_trait::async_trait;
//
// #[async_trait]
// pub trait Exchange: Subscriber {
//     type InitializationParameters;
//
//     fn endpoint_url(environment: Environment) -> &'static str;
//     async fn new(parameters: Self::InitializationParameters) -> Result<Self, Self::Error> where Self: Sized;
// }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Environment {
    Production,
    Sandbox
}

impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}
