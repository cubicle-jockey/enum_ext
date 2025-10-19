use enum_ext::enum_extend;

#[test]
fn complex_enum_as_int_and_strings() {
    #[enum_extend(IntType = "u32")]
    #[derive(Clone)]
    enum Complex {
        AlphaOne(u32) = 4,
        BetaTwo((u32, i16)) = 8,
        CharlieThree { fred: u32, barny: i16 } = 16,
    }

    let a = Complex::AlphaOne(10);
    let b = Complex::BetaTwo((1, -2));
    let c = Complex::CharlieThree { fred: 5, barny: -7 };

    // as_u32 via match arms (const fn)
    assert_eq!(a.as_u32(), 4);
    assert_eq!(b.as_u32(), 8);
    assert_eq!(c.as_u32(), 16);

    // ordinal and name conversions should still work with field-ignoring patterns
    assert_eq!(a.ordinal(), 0);
    assert_eq!(b.ordinal(), 1);
    assert_eq!(c.ordinal(), 2);

    assert_eq!(a.pascal_spaced(), "Alpha One");
    assert_eq!(b.pascal_spaced(), "Beta Two");
    assert_eq!(c.pascal_spaced(), "Charlie Three");

    assert_eq!(a.snake_case(), "alpha_one");
    assert_eq!(b.snake_case(), "beta_two");
    assert_eq!(c.snake_case(), "charlie_three");

    assert_eq!(a.kebab_case(), "alpha-one");
    assert_eq!(b.kebab_case(), "beta-two");
    assert_eq!(c.kebab_case(), "charlie-three");

    // is_first/is_last still work
    assert!(a.is_first());
    assert!(c.is_last());
}
