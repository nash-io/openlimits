cross_test::configure!();

#[cfg(test)]
mod test {
    use cross_test::prelude::*;

    use huobi::prelude::*;
    use huobi::{HuobiParameters, Environment, Subscription, Symbol, MarketPair};
    use huobi::Huobi;

    #[cross_test::test]
    async fn huobi() {
        let mut huobi = Huobi::new(HuobiParameters::new(Environment::Production)).await.expect("Couldn't connect to Huobi");
        let mut subscription = huobi.subscribe(Subscription::OrderBook(MarketPair(Symbol::ETH, Symbol::BTC))).await.expect("Couldn't subscribe");
        while let Some(message) = subscription.next().await {
            println!("{:#?}", message);
        }
        println!("Ended.")
    }
}
