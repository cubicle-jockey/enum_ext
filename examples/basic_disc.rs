#![allow(unused, dead_code)]
use enum_ext::{enum_ext, enum_extend};

fn main() {
    // Both the `enum_ext` and `enum_extend` macros provide the same functionality. They only differ in the way they are called.

    // For demonstration purposes, here is one example of using the `enum_ext` macro. The rest of the examples will use the enum_extend macro.
    enum_ext! {
        #[derive(Debug, PartialEq)]
        #[enum_def(IntType = "i32")] // <- Discriminant type is defined here.
        enum MyEnum {
            Ten = 10,
            Twenty = 20,
            Thirty = 30,
        }
    }

    // *** NOTE: Most utility examples are in basic_no_disc.rs. Start there ***

    // The following examples focus exclusively on discriminant utility, which isn't covered in basic_no_disc.
    from_disc_example();
}

// Discriminant utility functions

fn from_disc_example() {
    // Discriminant enums can be converted to and from integers.
    // This is useful for serialization and deserialization.

    #[enum_extend(IntType = "i32")] // <- Discriminant type is defined here.
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        Ten = 10,
        Twenty = 20,
        Thirty = 30,
        Neg60 = -60,
    }

    // `IntType` determines the `from_<>` and `as_<>` functions available.
    // In this case, `IntType = "i32"` means `from_i32()` and `as_i32()` are available.
    assert_eq!(MyEnum::from_i32(20), Some(MyEnum::Twenty));
    assert_eq!(MyEnum::Twenty.as_i32(), 20);

    for variant in MyEnum::iter() {
        let i = variant.as_i32();
        assert_eq!(MyEnum::from_i32(i).as_ref(), Some(variant));
    }

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //

    print_eq_line();
    println!("Discriminant utility functions");
    print_eq_line();
    println!("{}", MyEnum::pretty_print());
    println!("MyEnum::Twenty.as_i32() = {:?}", MyEnum::Twenty.as_i32());
    println!("MyEnum::from_i32(20) = {:?}", MyEnum::from_i32(20));
}

fn print_eq_line() {
    println!("===================================================");
}

fn print_dash_line() {
    println!("---------------------------------------------------");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_disc_example() {
        from_disc_example();
    }
}
