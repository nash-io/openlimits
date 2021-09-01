ligen_macro::ignore!();

use messaging::Subscriber;
use async_trait::async_trait;

#[async_trait]
pub trait Exchange: Subscriber {
    type InitializationParameters;

    fn endpoint_url(environment: Environment) -> &'static str;
    async fn new(parameters: Self::InitializationParameters) -> Result<Self, Self::Error> where Self: Sized;
}

pub enum Environment {
    Production,
    Sandbox
}
