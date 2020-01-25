extern crate enum_repr_derive;
use enum_repr_derive::{Into, TryFrom};

#[repr(i8)]
#[derive(Into, TryFrom, Copy, Clone, Debug, PartialEq)]
enum Foo {
    VAR1 = -1,
    Error = -3,
}

fn main() {}
