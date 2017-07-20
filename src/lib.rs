#[macro_use]
extern crate custom_derive;

macro_rules! numberalize {
    ($n:expr) => {
        $n
    }
}

macro_rules! birth_foo {
    ($n:expr) => {
        #[derive(Custom)]
        struct Foo {
            n: [i32; numberalize!(n)]
        }
    }
}

birth_foo!(5);