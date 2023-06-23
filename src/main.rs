#![allow(unused, dead_code)]

mod types;
mod traits;
mod lifetimes;
use std::fmt::Display;

fn main() {
    introduce();
    types::explain();
    traits::explain();
    lifetimes::explain();
}


pub fn introduce() {
    println!("
    ----------------------CHAPTER 10: GENERIC TYPES, TRAITS, AND LIFETIMES----------------------
    ");
}

//After you read this lesson, see if you can understand this!
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//Rewrite this so it does the same thing!