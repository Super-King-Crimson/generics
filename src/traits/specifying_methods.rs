use std::fmt::Display;

pub fn explain() {
    let mut displayable_boredom = Boredom::new("AAAAAAAAAAAAAAAAAAAAAAAA");
    displayable_boredom.id_rather_be(0, String::from("chewing bubblegum"));
    displayable_boredom.id_rather_be(1, String::from("kicking butt"));
    displayable_boredom.id_rather_be(2, String::from("WATCHING PO-"));

    displayable_boredom.pontificate();

    let mut undisplayable_boredom = Boredom::new(vec![100, 1000, 10000]);
    
    //A vec doesn't impl the Display trait, so no worky!
    // undisplayable_boredom.pontificate(); 
}

pub struct Boredom<T> {
    level: T,
    things_id_rather_be_doing: [String; 3],
    how_much_i_hate_the_rust_book: u128,
}

//No matter what type you are, you can use the new and id_rather_be methods
impl<T> Boredom<T> {
    fn new(boredom: T) -> Self {
        Boredom { 
            level: boredom, 
            things_id_rather_be_doing: [String::new(), String::new(), String::new()], 
            how_much_i_hate_the_rust_book: u128::MAX,
        }
    }

    fn id_rather_be(&mut self, i: usize, msg: String) {
        if i < 3 {
            self.things_id_rather_be_doing[i] = msg;
        }
    }
}

//Hey look, a function that you can only use if the thing in boredom has the Display trait!
impl<T: Display> Boredom<T> {
    fn pontificate(&self) {
        println!("Oh lord, I'm so completely bored, my boredom level is {}", self.level);
        println!("Oh me oh my, why am I doing this, I could be {}, {}, or even {}", 
            self.things_id_rather_be_doing[0], self.things_id_rather_be_doing[1], self.things_id_rather_be_doing[2]);
        println!("Boohoo boohoo boohoo boohoo");
    }
}

//You can also conditionally implement traits when they implement a trait (this is called blanket implementation)
pub trait Killable {
    fn kill(&mut self) {}
}
pub trait Revivable {
    fn revive(&mut self) {}
}

pub struct SomeGuy<T> { age: u8, video_game_stuff_ig: T }

//Don't ask me why you'd do this or why it works, but it does 
//This is apparently the reason why you can to_string numbers
impl <T: Killable> Revivable for T {}