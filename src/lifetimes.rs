pub mod generics;
pub mod how_to_use;
pub mod r#static;

pub fn explain() {
    //lifetimes prevent undefined behavior by making it impossible to have dangling references
    let x: u8 = 0;  //birth of x

    {
        //y doesn't live long enough
        let y = 8u8; //birth of y
        // x = &y; x points to y
    }  //death of y

    println!("{x}"); //x points to dead (dropped) value

    generics::explain();
    how_to_use::explain();
    r#static::explain();
}   //death of x