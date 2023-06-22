struct Favorites<T, U> {
    favorite_number: T,
    favorite_thing: U,
}

enum Thing<T, U, V> {
    Person(Favorites<T, U>),
    Animal {age: V},
}

//Notice we have to put the generics in front of the impl block
impl<T, U> Favorites<T, U> {
    fn get_favorites(&self) -> (&T, &U) {
        (&self.favorite_number, &self.favorite_thing)
    }
}

//This function only exists if T in favorites is an f64
//Notice how we only had to put one type in front of the impl block
impl<U> Favorites<f64, U> {
    fn print_decimal(&self) {
        println!("{}", &self.favorite_number - &self.favorite_number.floor());
    }
}

pub fn explain() {
    let i32s_and_dogs = Favorites {
        favorite_number: 3,
        favorite_thing: String::from("EL PERRO"),
    };
     
    let pi_man = Favorites {
        favorite_number: 3.14,
        favorite_thing: "pie",
    };
}