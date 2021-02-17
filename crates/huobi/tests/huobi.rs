cross_test::configure!();

#[cfg(test)]
mod test {
    use cross_test::prelude::*;

    use huobi::prelude::*;
    use huobi::{HuobiParameters, Environment};
    use huobi::Huobi;
    use exchange::prelude::*;
    use exchange::message::subscription::Subscription;
    use exchange::market::MarketPair;
    use exchange::market::symbol::Symbol;

    #[cross_test::test]
    async fn huobi() {
        let mut huobi = Huobi::new(HuobiParameters::new(Environment::Production)).await.expect("Couldn't connect to Huobi");
        let mut subscription = huobi.subscribe(&Subscription::Tick(MarketPair(Symbol::ETH, Symbol::BTC))).await.expect("Couldn't subscribe");
        while let Some(message) = subscription.next().await {
            println!("{:#?}", message);
        }
        println!("Ended.")
    }
}
