
struct Human {
    name: &'static str,
    age: u8
}

struct Cat {
    name: &'static str,
    age: u8
}

trait LivingBeing {
    fn name(&self) -> &'static str;
    fn age(&self) -> u8;
    fn say_hi(&self) -> ();
}

impl LivingBeing for Human {
    fn name(&self) -> &'static str {
        return self.name;
    }
    fn age(&self) -> u8 {
        return self.age;
    }
    fn say_hi(&self) -> () {
        println!("Hello!!!, I'm {}", self.name)
    }
}

impl LivingBeing for Cat {
    fn name(&self) -> &'static str {
        return self.name;
    }
    fn age(&self) -> u8 {
        return self.age;
    }
    fn say_hi(&self) -> () {
        println!("{}: MEEOOOW!!!", self.name)
    }
}


fn runtime_create_factory_instance(being_type: &'static str) -> Box<dyn LivingBeing> {
    if being_type == "HUMAN" {
        Box::new(Human { name: "Paco", age: 21})
    } else if being_type == "CAT" {
        Box::new(Cat { name: "Moguri", age: 7 })
    } else {
        panic!("Unknown type")
    }
}

pub fn factory_pattern_runtime_test() {
    let human = runtime_create_factory_instance("HUMAN");
    let cat = runtime_create_factory_instance("CAT");
    human.say_hi();
    cat.say_hi();
}

pub fn run_traits () {
    factory_pattern_runtime_test();
}