mod animal;

use animal::cat::Cat;

fn main() {
    let cat = Cat::new("Moguri", 4);
    cat.meow();
    cat.say_age();
}