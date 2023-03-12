#![allow(dead_code)]

extern crate core;

use std::convert::Infallible;
use std::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    println!("Hello, world!");
}

#[test]
fn test01() {
    // assert_eq!(1u32, 1i32);
    let i = 2;
    let b = i - 1i32;
}

#[test]
fn test_tuple() {
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // `let` can be used to bind the members of a tuple to variables
        let (int_param, bool_param) = pair;
        (bool_param, int_param)
    }

    let (a, b) = reverse((1, false));
    println!("{a} {b}")
}

#[test]
fn test_slice() {
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    fn reverse(slice: &mut [i32]) {
        slice.reverse();
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    let mut xs: [i32; 5] = [1, 2, 3, 4, 5];
    analyze_slice(&xs);
    reverse(&mut xs);
}

#[test]
fn test_struct() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.name, self.age)
        }
    }

    let mut p = Person {
        name: String::from("a"),
        age: 1,
    };
    p.name = String::from("b");
    println!("{p}");

    enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }

    use Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    enum Number {
        Zero,
        One,
        Two,
    }

    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

#[test]
fn test_shadowed() {
    let mut a = 1;
    {
        a = 2;
        println!("{a}");
        let a = 3;
        println!("{a}");
    }
    println!("{a}")
}

#[test]
fn test_type_cast() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;
    println!("{integer} {character}")
}

#[test]
fn test_type_alias() {
    type U32 = u32;
    type U32Too = u32;

    let a: U32 = 2;
    let b: U32Too = 2;
    assert_eq!(a, b);
    let c = a - b;
    println!("{c}")
}

#[test]
fn test_conv() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    impl Display for Number {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let num = Number::from(30);
    println!("My number is {num}");

    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {num}");

    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

#[test]
fn test_loop() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}

#[test]
fn test_trait() {
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait NonAnimal {
        fn haha(&self);
    }
    trait Animal {
        // Associated function signature; `Self` refers to the implementor type.
        fn new(name: &'static str) -> Self;

        // Method signatures; these will return a string.
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // Traits can provide default method definitions.
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // Implementor methods can use the implementor's trait methods.
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        // `Self` is the implementor type: `Sheep`.
        fn new(name: &'static str) -> Sheep {
            Sheep {
                name: name,
                naked: false,
            }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }

        // Default trait methods can be overridden.
        fn talk(&self) {
            // For example, we can add some quiet contemplation.
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    impl NonAnimal for Sheep {
        fn haha(&self) {
            println!("{} {}", self.name, "impl NonAnimal for Sheep")
        }
    }
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();
    dolly.haha();
    println!("{}", dolly.name());
}

#[test]
fn test_macro() {
    // This is a simple macro named `say_hello`.
    macro_rules! say_hello {
        // `()` indicates that the macro takes no argument.
        () => {
            // The macro will expand into the contents of this block.
            println!("Hello!");
        };
        ($e:literal) => {
            // The macro will expand into the contents of this block.
            println!("Hello! {}", $e);
        };
    }
    say_hello!();
    say_hello!("hhh")
}

#[test]
fn test_str(){
    let mut a = String::from("123a");
    let mut b = "123b";
    b = a.as_str();
    println!("{a} {b}")
}
