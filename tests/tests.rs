#[macro_use]
extern crate enum_repr_derive;

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    #[repr(i8)]
    #[derive(TryFrom, Into, Copy, Clone, Debug, PartialEq)]
    enum Foo {
        VAR1 = -1,
        VAR2 = -3,
    }

    #[repr(u32)]
    #[derive(TryFrom, Into, Copy, Clone, Debug, PartialEq)]
    enum Foo32 {
        VAR1 = 1024,
    }

    #[test]
    fn test_i8() {
        assert_eq!(Foo::try_from(-1), Ok(Foo::VAR1));
        assert_eq!(Foo::try_from(-3), Ok(Foo::VAR2));
        assert_eq!(Foo::try_from(-9), Err(-9));

        assert_eq!(Into::<i8>::into(Foo::VAR1), -1);
        assert_eq!(Into::<i8>::into(Foo::VAR2), -3);
    }
    #[test]
    fn test_u32() {
        assert_eq!(Foo32::try_from(1024), Ok(Foo32::VAR1));
        assert_eq!(Foo32::try_from(50), Err(50));

        assert_eq!(Into::<u32>::into(Foo32::VAR1), 1024);
    }
}
