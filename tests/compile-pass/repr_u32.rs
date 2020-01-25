extern crate enum_repr_derive;
use enum_repr_derive::{Into, TryFrom};

#[repr(u32)]
#[derive(Into, TryFrom, Copy, Clone, Debug, PartialEq)]
enum Foo {
    VAR1 = 1024,
}

fn main() {}
