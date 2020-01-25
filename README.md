# enum-repr-derive

[![Build Status](https://www.travis-ci.org/ssalonen/enum-repr-derive.svg?branch=master)](https://www.travis-ci.org/ssalonen/enum-repr-derive)
[![Crate](https://img.shields.io/crates/v/enum-repr-derive.svg)](https://crates.io/enum-repr-derive)
[![Documentation](https://docs.rs/enum-repr-derive/badge.svg)](https://docs.rs/enum-repr-derive)

Procedural derive macro for converting fieldless enums to (`Into`) and from (`TryFrom`) its repr type.

See the [Nomicon section on `repr`](https://doc.rust-lang.org/nomicon/other-reprs.html#repru-repri) for more details on fieldless enums.

## Example code

By using this library the following code just works:

```rust
#[macro_use]
extern crate enum_repr_derive;
use enum_repr_derive::{Into, TryFrom};
use std::convert::TryFrom;

#[repr(i8)]
#[derive(TryFrom, Into, Copy, Clone, Debug, PartialEq)]
enum Foo {
    VAR1 = -1,
    VAR2 = -3,
}
assert_eq!(Foo::try_from(-1), Ok(Foo::VAR1));
assert_eq!(Foo::try_from(-3), Ok(Foo::VAR2));
assert_eq!(Foo::try_from(-9), Err(-9));
assert_eq!(Into::<i8>::into(Foo::VAR1), -1);
assert_eq!(Into::<i8>::into(Foo::VAR2), -3);
```

## License

Licensed under MIT. See `LICENSE` file.

## For developers

Release: `cargo release`
