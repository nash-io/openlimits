// use hashbrown::HashMap;

// #[derive(Debug, Serialize)]
// pub struct Ticker {
//     pub ask: Option<String>,
//     pub bid: Option<String>,
//     pub last: Option<String>,
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct CurrencyPair {
//     pub base: String,
//     pub quote: String,
// }

// impl ToString for CurrencyPair {
//     fn to_string(&self) -> String {
//         format!("{}_{}", self.quote, self.base)
//     }
// }

// pub struct CurrencyPairList {
//     pub items: HashMap<String, CurrencyPairListItem>,
// }

// impl CurrencyPairList {
//     pub fn new<F>(currency_pairs: &Vec<CurrencyPair>, symbol_predicate: F) -> Self
//     where
//         F: Fn(&CurrencyPair) -> String,
//     {
//         let items = currency_pairs
//             .iter()
//             .map(|pair| {
//                 (
//                     symbol_predicate(pair),
//                     CurrencyPairListItem {
//                         pair: pair.clone(),
//                         is_active: true,
//                     },
//                 )
//             })
//             .collect();

//         CurrencyPairList { items }
//     }

//     pub fn find(&self, symbol: &String) -> Option<&CurrencyPair> {
//         self.items.get(symbol).map(|item| &item.pair)
//     }
// }

// pub struct CurrencyPairListItem {
//     pub pair: CurrencyPair,
//     pub is_active: bool,
// }

// type Callback = fn();

// pub struct Market {

// }

// pub struct Trades {

// }