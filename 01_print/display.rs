use std::fmt;

struct Person<'a, 'b> {
    name: &'a str,
    age: u8,
    surname: &'b str
}

impl fmt::Display for Person<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "His name is {}, he is {} old, and his surname is {}", self.name, self.age, self.surname)
    }
}

fn main () {
    let kevin = Person {
        name: "Kevin",
        age: 28,
        surname: "Guanche"
    };
    println!("Display for Person {}", kevin);
}