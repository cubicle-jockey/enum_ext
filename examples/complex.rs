#![allow(unused, dead_code)]
use enum_ext::{enum_ext, enum_extend};

// Examples for working with complex enums (variants with payloads)
//
// Run with:
//   cargo run --example complex
//
// These examples demonstrate both attribute and function-like macros with complex enums,
// showing the available APIs (ordinal and string naming helpers) and integer conversions
// via const match-based `as_<IntType>(&self)` methods.

fn attribute_macro_with_inttype() {
    // IntType is optional; when provided, the as_<IntType>() helper will use that type.
    // For complex enums, all payload-carrying variants must declare an explicit discriminant.
    #[enum_extend(IntType = "u32")]
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Complex {
        AlphaOne(u32) = 4,
        BetaTwo((u32, i16)) = 8,
        CharlieThree { fred: u32, barny: i16 } = 16,
    }

    let a = Complex::AlphaOne(10);
    let b = Complex::BetaTwo((1, -2));
    let c = Complex::CharlieThree { fred: 5, barny: -7 };

    // Integer conversion retains match-based logic and is const
    assert_eq!(a.as_u32(), 4);
    assert_eq!(b.as_u32(), 8);
    assert_eq!(c.as_u32(), 16);

    // Ordinal and naming helpers work with field-ignoring match patterns
    assert_eq!(a.ordinal(), 0);
    assert_eq!(b.ordinal(), 1);
    assert_eq!(c.ordinal(), 2);

    assert_eq!(a.pascal_spaced(), "Alpha One");
    assert_eq!(b.pascal_spaced(), "Beta Two");
    assert_eq!(c.pascal_spaced(), "Charlie Three");

    assert!(a.is_first());
    assert!(c.is_last());

    // Note: Construction-based helpers are intentionally omitted for complex enums:
    // - list()/iter(), from_ordinal()/ref_from_ordinal(), next()/previous(),
    // - from_<IntType>(), impl From<<IntType>>, from_* string conversions, variant_names(),
    // - batch functions (slice/range/first_n/last_n), and random helpers.
}

fn attribute_macro_default_usize() {
    // When IntType is not provided, the default integer type is usize and `as_usize()` is generated.
    #[enum_extend]
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum ComplexDefault {
        A(u32) = 100,
        B((u32, i16)) = 200,
        C { x: u32, y: i16 } = 300,
    }

    let a = ComplexDefault::A(1);
    let b = ComplexDefault::B((2, -3));
    let c = ComplexDefault::C { x: 3, y: -4 };

    assert_eq!(a.as_usize(), 100);
    assert_eq!(b.as_usize(), 200);
    assert_eq!(c.as_usize(), 300);

    assert_eq!(a.ordinal(), 0);
    assert_eq!(b.ordinal(), 1);
    assert_eq!(c.ordinal(), 2);

    assert_eq!(a.snake_case(), "a");
    assert_eq!(b.snake_case(), "b");
    assert_eq!(c.snake_case(), "c");
}

fn proc_macro_with_inttype() {
    // The function-like macro variant. This can be placed in any scope.
    enum_ext! {
        #[enum_def(IntType = "u32")]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ComplexProc {
            AlphaOne(u32) = 4,
            BetaTwo((u32, i16)) = 8,
            CharlieThree { fred: u32, barny: i16 } = 16,
        }
    }

    let a = ComplexProc::AlphaOne(42);
    assert_eq!(a.as_u32(), 4);
    assert_eq!(a.kebab_case(), "alpha-one");
    assert_eq!(a.ordinal(), 0);
}

fn main() {
    attribute_macro_with_inttype();
    attribute_macro_default_usize();
    proc_macro_with_inttype();

    println!("All complex enum examples ran successfully.");
}
