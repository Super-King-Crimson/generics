pub mod as_parameters;
pub mod specifying_methods;

pub fn explain() {
    println!("FINALLY WE'RE LEARNING ABOUT TRAITS!");
    //Traits are Rust's version of interfaces (with some differences)
    //They allow you to specify generic types (abstract shared behavior)
    let tweet = Tweet {
        contents: String::from("Just had a dream that I had huge boobs LOL"),
        poster: String::from("SomeGovTeacher"),
        reply: None,
        retweet: None,
    };

    let article = NewsArticle {
        headline: String::from("All 5 aboard Titanic sub presumed dead after evidence of 'catastrophic implosion'"),
        author: String::from("NBC News"),
        contents: String::from("blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah ")
    };

    println!("Here are some summaries: \nFrom Twitter: {}\nFrom a local news station: {}", tweet.summarize(), article.summarize());
    as_parameters::explain();
    specifying_methods::explain();
}

//We don't provide implementation (if you do that's the default behavior when no impl provided)
pub trait Summary {
    fn summarize(&self) -> String {
        "Check out the original content!".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub contents: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("According to {}, {}", &self.author, &self.headline)
    }
}
pub struct Tweet {
    pub contents: String,
    pub poster: String,
    pub reply: Option<Box<Tweet>>,
    pub retweet: Option<Box<Tweet>>,
}

//This uses the default implementation of summarize
impl Summary for Tweet {}
//A trait can implemented for a type only if either the trait or type is local to our crate
//We can't implement Display on Vec<T>, for example, because neither is defined in the Generics crate