pub fn explain() {
    //The way lifetime parameters are specified depends on what the function does
    let hi = String::from("hi");
    let bye = String::from("bye");

    let first = return_first_arg(hi.as_str(), bye.as_str());

    let str = String::from("My name is jugemu jugemu");
    let non_owned = FinallyNonOwnedDataInStruct { not_owned: &str };
}

//We don't have to lifetime annotate second, it's annotation wouldn't ever be used  
fn return_first_arg<'a>(first: &'a str, second: &str) -> &'a str {
    first
}

//Make sure the lifetime annotation of a parameter matches the annotation of the return val
fn return_second_arg<'a>(first: &str, second: &str) /* -> &'a str */ {
    second;
}

fn return_local_something<'a>(x: &str, y: &str) /* -> &'a str */ {
    String::from("This is local").as_str();
}
//The lifetime annotation has to match the return value, meaning you can't send out local function variables 
//(as you can't annotate local variables' lifetimes)

//This means that any struct made must live as long as the data not_owned points to
pub struct FinallyNonOwnedDataInStruct<'a> { not_owned: &'a str }

//Rust team built in implicit annotations into the compiler - at first every reference needed a lifetime
//The rules that the compiler follows to allow an implicit annotation are called the lifetime elision rulesd
//The elision rules aren't full inference - if a rule applies but lifetime is unclear, won't compile

//lifetimes on function params are called input lifetimes, lifetimes on return values are output lifetimess

//Rule 1: All INPUT LIFETIMES get a different input lifetime parameter. Structs also need their own lifetime param
//Rule 2: If there is 1 input lifetime parameter, give all OUTPUT LIFETIMES that parameter
//Rule 3: If there is >1 input lifetime parameter, but one is self, give all OUTPUT LIFETIMES self's parameter

//Let's walk through the rules with some examples
fn first_word(s: &str) -> &str { s }
fn first_word_r1<'a>(s: &'a str) -> &str { s } //Apply Rule 1: s gets 'a
fn first_word_r2<'a>(s: &'a str) -> &'a str { s } //Rule 2 applies: all references now have lifetimes

fn return_random(s1: &str, s2: &str) /* -> &str */ { let a = true; if a { s1 } else { s2 }; }
fn return_random_r1<'a, 'b>(s1: &'a str, s2: &'b str) /* -> &str */ { let a = true; if a { s1 } else { s2 }; } 
//Apply Rule 1: s1 gets 'a, s2 gets 'b

//Neither Rule 2 nor Rule 3 apply - compiler can't infer, error

//Oh btw here's how you put lifetimes in method defs:
impl<'a> FinallyNonOwnedDataInStruct<'a> {
    //First elision rule should apply...
    //Don't even ask me what this does, pretend first elision rule applies
    fn new<'b: 'a>(s: &'b str) -> Self {
        FinallyNonOwnedDataInStruct { not_owned: s }
    }

    //Third elision rule applies (one is self, so who cares about b return the ref to self)
    fn get_selfs<'b>(&self, other: &'b str) -> &str {
        println!("Who cares about this thing. {other}? Really?");
        self.not_owned
    }
}