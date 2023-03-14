fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

#[cfg_attr(test, automock)]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

// #[cfg_attr(test, automock)]
// fn foo1(x: u32) -> u32 {
//     if x == 1 {
//         return 1;
//     }
//     return 2;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mytest() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .with(eq(4))
            .times(1)
            .returning(|x| x + 1);
        assert_eq!(5, mock.foo(4));
    }

    // #[test]
    // fn mytest1() {
    //     let mut mock = Mockfoo1;
    //     mock.expect_foo()
    //         .with(eq(4))
    //         .times(1)
    //         .returning(|x| x + 1);
    //     assert_eq!(5, mock.foo(4));
    // }
}