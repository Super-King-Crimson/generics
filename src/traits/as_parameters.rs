use std::fmt::Display;

use super::{Tweet, NewsArticle, Summary};

pub fn explain() {
    println!("It's still so surreal that we're actually learning about traits.")
}

//Now we can be a little more generic: 
//it's not something specific, it's something that implements the properties of a Summary
pub fn notify(thing: &impl Summary) {
    println!("THIS JUST IN: {}!", thing.summarize().to_ascii_uppercase());
}

//In addition, that's actually syntax sugar: this is what that function's signature actually is:
pub fn notify_desugared<T: Summary>(thing: &T) {
    //We're back to generic types! This is called a trait bound
    println!("THIS JUST IN: {}!", thing.summarize().to_ascii_uppercase());
    //If you have multiple arguments and you want to ensure they're the same type and impl the same trait, 
    //they're a must use
}

//We can also check for a double trait bound like this
pub fn summarize_and_print(thing: &(impl Summary + Display)) {}
pub fn summarize_and_print_desugared<T: Summary + Display>(thing: &T) {}

//You can also use ANOTHER syntax for declaring trait bound with where syntax:
pub fn sum_stuff<T: Summary + Display, U: Iterator + Display>(x: &T, y: &U) {}
pub fn sum_stuff_where<T, U>(x: &T, y: &U) where
    T: Summary + Display,
    U: Iterator + Display,
{}

//As for returning a value that has a specific trait, it works
//This will be very useful in a few chapters when we learn about closures
//Do note we actually cannot use the desugared version of this
pub fn get_summarizable() -> impl Summary {
    Tweet {
        contents: String::from("I am Johnny"),
        poster: String::from("Johnny"),
        reply: Some(Box::new(
            Tweet {
                contents: String::from("Who are you"), 
                poster: String::from("ConfusedMan194"),
                reply: None,
                retweet: None,
            }
        )),
        retweet: None
    }
}
//Also we cannot use this to return two different types (it's because of how the compiler works with impl Trait)
//It's possible with other syntax (we'll talk about that later)