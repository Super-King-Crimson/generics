pub fn explain() {
    println!("We have a function that returns the longer of two string slices.");
    let a = String::from("abcde");
    let longest_str: &str;

    //doesn't work because longest_str lives longer than the smallest lifetime (b)
    //Now this code can't cause any undefined behavior because the slice would be of a (cuz its longer)
    //But the compiler doesn't know that
    {
        let b = String::from("def");
        longest_str = longest(a.as_str(), b.as_str());
    }

    // println!("The longest is {longest_str}");
}

//Rust needs to know whether return value is related to a or b (we actually dk either tho)
//To fix this, we use generic lifetime parameters
fn longest_bad(a: &str, b: &str) /* -> &str */  {
    if a.len() > b.len() {a} else {b};
}

//Lifetime annotations don't change how long a reference lives,
//they describe how long they live relative to other references
//Just like a function can accept any type with generic typing parameter,
//a function can accept a reference with any lifetime using generic lifetime parameters

//Hey, it's the angle brackets again! ('a is the reference name: all refnames start with ')
//This function accepts two &strs and returns a &str, and all of them live at least as long as some lifetime 'a
//That means 'a has to be the smaller lifetime of a and b, 
//and the return value MUST be defined over that whole lifetime
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {a} else {b}
}