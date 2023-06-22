#![allow(unused, dead_code)]

mod types;
mod traits;
mod lifetimes;

fn main() {
    introduce();
    types::explain();
    traits::explain();
}


pub fn introduce() {
    println!("
    ----------------------CHAPTER 10: GENERIC TYPES, TRAITS, AND LIFETIMES----------------------
    ");
}
