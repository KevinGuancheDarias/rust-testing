use std::fmt;

mod traits;

#[derive(Debug)]
struct Foo {
    data: u8
}

struct TestPartialMove {
    foo: String,
    bar: String
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Destroying Foo with data value {}", self.data);
    }
}

impl fmt::Display for TestPartialMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value is : foo = {}, bar = {}", self.foo, self.bar)
    }
}

fn acquire_ownership_of_foo(foo: Foo) -> () {
    println!("I'm the owner of foo: {}", foo.data);
}

fn borrow_of_foo(foo: &Foo) -> () {
    // foo.data = 4; Can't do because is not a mutable borrow
    println!("I got a borrow of foo {}", foo.data);
}

fn mut_borrow_of_foo(baz: &mut Foo) -> () {
    let old_value = Foo { data: baz.data};
    baz.data = 71;
    println!("I have muted the value of baz from {:?} to {:?}", old_value,  baz);
}


fn test_partial_move() -> () {
    let foo_struct = TestPartialMove { 
        foo: String::from("Hello"),
        bar: String::from("World")
    };
    let TestPartialMove { ref foo, bar} = foo_struct;
    println!("Val of moved prop is = {}", bar);
    // println!("Struct is moved, would fail: {}", foo_struct);
    println!("Struct prop data is accessible, as has been borrowed/referenced {} = {}", foo, foo_struct.foo);
    // println!("Would fail as value was moved {}", foo_struct.bar);
}

fn main() {
    let foo = Foo {data: 8};
    acquire_ownership_of_foo(foo);
    // println!("Access foo: {} , Would fail as foo was destroyed in the acquire_ownership_of_foo function", foo.data);
    let bar = Foo { data: 4};
    borrow_of_foo(&bar);
    // mut_borrow_of_foo(&mut bar); Would not work as bar is not declared as mutable
    let mut baz = Foo { data: 0 };
    mut_borrow_of_foo(&mut baz);
    println!("The value of baz is now {:?}", baz);
    println!("Access bar: {} works, as borrow_of_foo doesn't acquire the ownership", bar.data);
    test_partial_move();
    traits::run_traits();
    println!("Will invoke the drop of bar after this");
}
