pub struct Cat<'a> {
    name: &'a str,
    age: u8
}

impl Cat<'_> {
    pub fn new<'a>(name: &'a str, age: u8) -> Cat<'a> {
        Cat { name, age }
    }

    pub fn meow(&self) -> () {
        println!("Cat {} Meeeeowwww", self.name)
    }

    pub fn say_age(&self) -> () {
        println!("I'm {} years old", self.age)
    }
}