#![allow(non_snake_case)]

pub mod environment;
pub mod client;
// pub mod ask_bid;
// pub mod orderbook;
// pub mod vector;
// use ligen::marshalling::{MarshalFrom, MarshalInto};

// #[repr(C)]
// pub struct Test {
//     pub value: i32,
//     pub environment: Environment,
// }
//
// // pub type Callback = Box<dyn Fn(Environment)>;
//
// pub struct Person {
//     first_name: String,
//     last_name: String
// }
//
//
// impl Test {
//     pub fn hello() {
//         println!("Hello from Test");
//     }
//
//     pub fn create(value: i32) -> Self {
//         Self { value, environment: Environment::Production }
//     }
//
//     pub fn print(value: String) {
//         println!("{}", value);
//     }
//
//     pub fn display(person: Person) {
//         println!("{} {}", person.first_name, person.last_name);
//     }
//
//     // pub fn set_callback(callback: Callback) {
//     //     callback(Environment::Production);
//     // }
// }
