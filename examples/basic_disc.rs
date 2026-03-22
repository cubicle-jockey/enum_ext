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
    try_from_example();
    disc_expression_example();
    copy_const_example();
    auto_derive_pretty_print_example();
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

fn try_from_example() {
    // `TryFrom<IntType>` is automatically implemented for discriminant enums.
    // This provides the standard Rust trait for fallible integer-to-variant conversion.

    #[enum_extend(IntType = "i32")]
    #[derive(Debug, PartialEq)]
    enum Status {
        Active = 1,
        Inactive = 2,
        Suspended = 3,
    }

    // Successful conversion
    let active: Result<Status, ()> = Status::try_from(1);
    assert_eq!(active, Ok(Status::Active));

    // Invalid discriminant returns Err(())
    let invalid: Result<Status, ()> = Status::try_from(999);
    assert_eq!(invalid, Err(()));

    // Works nicely with pattern matching
    match Status::try_from(2) {
        Ok(status) => assert_eq!(status, Status::Inactive),
        Err(()) => panic!("expected a valid status"),
    }

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //

    print_eq_line();
    println!("TryFrom example");
    print_dash_line();
    println!("{}", Status::pretty_print());
    println!("Status::try_from(1) = {:?}", Status::try_from(1));
    println!("Status::try_from(2) = {:?}", Status::try_from(2));
    println!("Status::try_from(999) = {:?}", Status::try_from(999));
}

fn disc_expression_example() {
    // Discriminant values can be expressions, not just literals.
    // This has been supported since v0.4.5.

    #[enum_extend(IntType = "i32")]
    #[derive(Debug, PartialEq)]
    enum Computed {
        X10 = 10,
        X25 = 5 * 5,
        Y26 = 13 + 13,
        Z100 = 10 * (5 + 5),
    }

    assert_eq!(Computed::X25.as_i32(), 25);
    assert_eq!(Computed::Y26.as_i32(), 26);
    assert_eq!(Computed::Z100.as_i32(), 100);

    assert_eq!(Computed::try_from(25), Ok(Computed::X25));
    assert_eq!(Computed::try_from(100), Ok(Computed::Z100));

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //

    print_eq_line();
    println!("Discriminant expression example");
    print_dash_line();
    println!("{}", Computed::pretty_print());
    println!("Computed::X25.as_i32() = {}", Computed::X25.as_i32());
    println!("Computed::Z100.as_i32() = {}", Computed::Z100.as_i32());
}

fn copy_const_example() {
    // When `Copy` is derived, `as_<IntType>()` becomes a `const fn`.
    // Without `Copy`, it requires `Clone` and is a regular `fn`.

    #[enum_extend(IntType = "u8")]
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Priority {
        Low = 1,
        Medium = 2,
        High = 3,
    }

    // With Copy, as_u8() is const — it can be used in const contexts
    const HIGH_VAL: u8 = Priority::High.as_u8();
    assert_eq!(HIGH_VAL, 3);

    // from_u8() is always const regardless of Copy
    const FROM_TWO: Option<Priority> = Priority::from_u8(2);
    assert_eq!(FROM_TWO, Some(Priority::Medium));

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //

    print_eq_line();
    println!("Copy / const example");
    print_dash_line();
    println!("{}", Priority::pretty_print());
    println!(
        "const HIGH_VAL: u8 = Priority::High.as_u8(); // = {}",
        HIGH_VAL
    );
    println!(
        "const FROM_TWO: Option<Priority> = Priority::from_u8(2); // = {:?}",
        FROM_TWO
    );
}

fn auto_derive_pretty_print_example() {
    // When an enum has discriminants, `enum_ext` automatically adds `Clone` (if not already
    // derived) and `#[repr(IntType)]`. You can see this in the pretty_print() output.

    #[enum_extend(IntType = "i32")]
    #[derive(Debug, PartialEq)] // Note: no Clone here
    enum AutoDerived {
        A = 1,
        B = 2,
        C = 3,
    }

    // pretty_print() shows the auto-added #[derive(Clone)] and #[repr(i32)]
    let pretty = AutoDerived::pretty_print();
    assert!(pretty.contains("#[derive(Clone)]"));
    assert!(pretty.contains("#[repr(i32)]"));

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //

    print_eq_line();
    println!("Auto-derive and pretty_print example");
    print_dash_line();
    println!("Original: #[derive(Debug, PartialEq)] with IntType = \"i32\"");
    println!("pretty_print() shows auto-added derives:");
    println!("{}", pretty);
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

    #[test]
    fn test_try_from_example() {
        try_from_example();
    }

    #[test]
    fn test_disc_expression_example() {
        disc_expression_example();
    }

    #[test]
    fn test_copy_const_example() {
        copy_const_example();
    }

    #[test]
    fn test_auto_derive_pretty_print_example() {
        auto_derive_pretty_print_example();
    }
}
