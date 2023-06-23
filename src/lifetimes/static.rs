pub fn explain() {
    //The 'static lifetime denotes a reference can live for the entire duration of the program
    //All string literals have 'static implicitly
    let a: &'static str = "Hello, World!";

    //Sometimes an error will ask you to make a reference 'static
    //Before you do that, make sure you actually can/want to have that specific reference live forever
}