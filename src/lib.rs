pub mod client;
pub mod exchange;
pub mod binance;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut post = Binance::new();
    }
}

