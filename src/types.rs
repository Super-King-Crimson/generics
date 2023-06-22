pub mod in_structs_and_enums;

use std::cmp::PartialOrd;

pub fn explain() {
    println!("First off, why generics?");
    //Let's say we have a function that gets the largest i32 in a vector
    let vec = vec![40, 239, 1, 2100, -190, 2];
    let largest = get_largest_i32_in_vec(&vec);

    //And a function that gets the largest char in a vector
    let vec2 = vec!['I', '❤', 'r', 'u', 's', 't'];
    let largest2 = get_largest_char_in_vec(&vec2);

    //These are literally the same function but with types swapped out!

    //To fix, let's make a generic function with generic type name T
    let vec3 = vec![1u16, 10, 50, 921, 201, 3];
    let largest3 = get_largest(&vec3);

    println!("The biggest ones are {}, {}, and {}", largest, largest2, largest3);

    in_structs_and_enums::explain();

    //Oh, by the way Rust generics don't have a performance impact!
    //The compiler uses something called monomorphization to specify our code based on generics
}

fn get_largest_i32_in_vec(slice: &[i32]) -> &i32 {
    let mut largest: &i32 = &slice[0];

    for i in slice {
        if i > largest {largest = i};
    }

    largest
}

fn get_largest_char_in_vec(slice: &[char]) -> &char {
    let mut largest: &char = &slice[0];

    for i in slice {
        if i > largest {largest = i};
    }

    largest
}

//This function is generic over some type T
//It has a parameter (slice), a slice that holds values of type T, and returns a reference to a value with type T
fn get_largest<T: PartialOrd>(slice: &[T]) -> &T {
    let mut largest = &slice[0];

    for v in slice {
        //Rust makes no assumptions about this type: for all it knows, it can't even be printed
        //There are no assumptions, so who knows if T CAN be greater or less than?
        if (v > largest) {largest = v};
    }

    largest
}
//A generic can be specified with traits (that's the T: thing that we'll learn about later)