extern crate enum_repr_derive;
use enum_repr_derive::{Into, TryFrom};

#[repr[i8]]
#[derive(TryFrom, Into, Copy, Clone, Debug, PartialEq)]
enum Foo {
    VAR1 = -1,
    VAR2 = -3,
}

fn main() {}
