#![allow(unused, dead_code)]
use enum_ext::{enum_ext, enum_extend};
#[test]
fn simple_1() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        pub enum Simple {
            A,
            B,
            C,
        }
    }

    assert_eq!(Simple::A as usize, 0);
    assert_eq!(Simple::B as usize, 1);
    assert_eq!(Simple::C as usize, 2);

    assert_eq!(Simple::A.ordinal(), 0);
    assert_eq!(Simple::B.ordinal(), 1);
    assert_eq!(Simple::C.ordinal(), 2);

    let mut ord = 0;
    for x in Simple::list() {
        assert_eq!(x.ordinal(), ord);
        ord += 1;
    }

    for (i, v) in Simple::iter().enumerate() {
        match i {
            0 => assert_eq!(v, &Simple::A),
            1 => assert_eq!(v, &Simple::B),
            2 => assert_eq!(v, &Simple::C),
            _ => unreachable!(),
        }
    }
}

#[test]
fn simple_2() {
    enum_ext! {
        #[derive(Debug, PartialEq, Clone)]
        pub enum Simple {
            A,
            B,
            C,
        }
    }
    // Clone is required for from_ordinal
    assert_eq!(Simple::from_ordinal(0), Some(Simple::A));
    assert_eq!(Simple::from_ordinal(1), Some(Simple::B));
    assert_eq!(Simple::from_ordinal(2), Some(Simple::C));
    assert_eq!(Simple::from_ordinal(3), None);
}

#[test]
fn simple_3() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        pub enum Simple {
            A,
            B,
            C,
        }
    }
    // Clone is not present, so we can only get a reference
    assert_eq!(Simple::ref_from_ordinal(0), Some(&Simple::A));
    assert_eq!(Simple::ref_from_ordinal(1), Some(&Simple::B));
    assert_eq!(Simple::ref_from_ordinal(2), Some(&Simple::C));
    assert_eq!(Simple::ref_from_ordinal(3), None);
}

#[test]
fn discriminant_1() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        pub enum Variant {
            A = 10,
            B = 20,
            C = 30,
        }
    }

    assert_eq!(Variant::A as usize, 10);
    assert_eq!(Variant::B as usize, 20);
    assert_eq!(Variant::C as usize, 30);

    assert_eq!(Variant::A.as_usize(), 10);
    assert_eq!(Variant::B.as_usize(), 20);
    assert_eq!(Variant::C.as_usize(), 30);

    assert_eq!(Variant::A.ordinal(), 0);
    assert_eq!(Variant::B.ordinal(), 1);
    assert_eq!(Variant::C.ordinal(), 2);

    let mut ord = 0;
    for x in Variant::list() {
        assert_eq!(x.ordinal(), ord);
        ord += 1;
    }

    for (i, v) in Variant::iter().enumerate() {
        match i {
            0 => assert_eq!(v, &Variant::A),
            1 => assert_eq!(v, &Variant::B),
            2 => assert_eq!(v, &Variant::C),
            _ => unreachable!(),
        }
    }

    for x in Variant::list() {
        assert_eq!(Some(x.clone()), Variant::from_usize(x.as_usize()));
    }

    // Clone is required for from_ordinal (always is present for discriminants)
    assert_eq!(Variant::from_ordinal(0), Some(Variant::A));
    assert_eq!(Variant::from_ordinal(1), Some(Variant::B));
    assert_eq!(Variant::from_ordinal(2), Some(Variant::C));
    assert_eq!(Variant::from_ordinal(3), None);

    // Clone is not present, so we can only get a reference
    assert_eq!(Variant::ref_from_ordinal(0), Some(&Variant::A));
    assert_eq!(Variant::ref_from_ordinal(1), Some(&Variant::B));
    assert_eq!(Variant::ref_from_ordinal(2), Some(&Variant::C));
    assert_eq!(Variant::ref_from_ordinal(3), None);
}

#[test]
fn discriminant_2() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        #[enum_def(IntType = "u8")]
        pub enum Variant {
            A = 10,
            B = 20,
            C = 30,
        }
    }

    assert_eq!(Variant::A as u8, 10);
    assert_eq!(Variant::B as u8, 20);
    assert_eq!(Variant::C as u8, 30);

    assert_eq!(Variant::A.as_u8(), 10);
    assert_eq!(Variant::B.as_u8(), 20);
    assert_eq!(Variant::C.as_u8(), 30);

    assert_eq!(Variant::A.ordinal(), 0);
    assert_eq!(Variant::B.ordinal(), 1);
    assert_eq!(Variant::C.ordinal(), 2);

    let mut ord = 0;
    for x in Variant::list() {
        assert_eq!(x.ordinal(), ord);
        ord += 1;
    }

    for (i, v) in Variant::iter().enumerate() {
        match i {
            0 => assert_eq!(v, &Variant::A),
            1 => assert_eq!(v, &Variant::B),
            2 => assert_eq!(v, &Variant::C),
            _ => unreachable!(),
        }
    }

    for x in Variant::list() {
        assert_eq!(Some(x.clone()), Variant::from_u8(x.as_u8()));
    }

    // Clone is required for from_ordinal (always is present for discriminants)
    assert_eq!(Variant::from_ordinal(0), Some(Variant::A));
    assert_eq!(Variant::from_ordinal(1), Some(Variant::B));
    assert_eq!(Variant::from_ordinal(2), Some(Variant::C));
    assert_eq!(Variant::from_ordinal(3), None);

    // Clone is not present, so we can only get a reference
    assert_eq!(Variant::ref_from_ordinal(0), Some(&Variant::A));
    assert_eq!(Variant::ref_from_ordinal(1), Some(&Variant::B));
    assert_eq!(Variant::ref_from_ordinal(2), Some(&Variant::C));
    assert_eq!(Variant::ref_from_ordinal(3), None);
}

#[test]
fn discriminant_neg1() {
    enum_ext! {
        #[enum_def(IntType = "i32")]
        #[derive(Debug, PartialEq)]
        pub enum Variant {
            A = -10,
            B = -20,
            C = -30,
        }
    }

    assert_eq!(Variant::A as i32, -10);
    assert_eq!(Variant::B as i32, -20);
    assert_eq!(Variant::C as i32, -30);

    assert_eq!(Variant::A.as_i32(), -10);
    assert_eq!(Variant::B.as_i32(), -20);
    assert_eq!(Variant::C.as_i32(), -30);

    assert_eq!(Variant::A.ordinal(), 0);
    assert_eq!(Variant::B.ordinal(), 1);
    assert_eq!(Variant::C.ordinal(), 2);

    let mut ord = 0;
    for x in Variant::list() {
        assert_eq!(x.ordinal(), ord);
        ord += 1;
    }

    for (i, v) in Variant::iter().enumerate() {
        match i {
            0 => assert_eq!(v, &Variant::A),
            1 => assert_eq!(v, &Variant::B),
            2 => assert_eq!(v, &Variant::C),
            _ => unreachable!(),
        }
    }

    for x in Variant::list() {
        assert_eq!(Some(x.clone()), Variant::from_i32(x.as_i32()));
    }

    // Clone is required for from_ordinal (always is present for discriminants)
    assert_eq!(Variant::from_ordinal(0), Some(Variant::A));
    assert_eq!(Variant::from_ordinal(1), Some(Variant::B));
    assert_eq!(Variant::from_ordinal(2), Some(Variant::C));
    assert_eq!(Variant::from_ordinal(3), None);

    // Clone is not present, so we can only get a reference
    assert_eq!(Variant::ref_from_ordinal(0), Some(&Variant::A));
    assert_eq!(Variant::ref_from_ordinal(1), Some(&Variant::B));
    assert_eq!(Variant::ref_from_ordinal(2), Some(&Variant::C));
    assert_eq!(Variant::ref_from_ordinal(3), None);
}

#[test]
fn pascal_spaced() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        pub enum PascalNames {
            PascalCase,
            MixedCase,
            MixedCase2,
            MixedCaseThree,
            MixedCaseFour4,
        }
    }

    for v in PascalNames::list() {
        match v {
            PascalNames::PascalCase => assert_eq!(v.pascal_spaced(), "Pascal Case"),
            PascalNames::MixedCase => assert_eq!(v.pascal_spaced(), "Mixed Case"),
            PascalNames::MixedCase2 => assert_eq!(v.pascal_spaced(), "Mixed Case2"),
            PascalNames::MixedCaseThree => assert_eq!(v.pascal_spaced(), "Mixed Case Three"),
            PascalNames::MixedCaseFour4 => assert_eq!(v.pascal_spaced(), "Mixed Case Four4"),
        }
    }

    for v in PascalNames::list() {
        match v {
            PascalNames::PascalCase => {
                assert_eq!(v, PascalNames::from_pascal_spaced("Pascal Case").unwrap())
            }
            PascalNames::MixedCase => {
                assert_eq!(v, PascalNames::from_pascal_spaced("Mixed Case").unwrap())
            }
            PascalNames::MixedCase2 => {
                assert_eq!(v, PascalNames::from_pascal_spaced("Mixed Case2").unwrap())
            }
            PascalNames::MixedCaseThree => {
                assert_eq!(
                    v,
                    PascalNames::from_pascal_spaced("Mixed Case Three").unwrap()
                )
            }
            PascalNames::MixedCaseFour4 => {
                assert_eq!(
                    v,
                    PascalNames::from_pascal_spaced("Mixed Case Four4").unwrap()
                )
            }
        }
    }
}

#[test]
fn pascal_spaced2() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        pub enum PascalNames {
            PascalCase = 10,
            MixedCase = 20,
            MixedCase2 = 30,
            MixedCaseThree = 40,
            MixedCaseFour4 = 50,
        }
    }

    for v in PascalNames::list() {
        match v {
            PascalNames::PascalCase => assert_eq!(v.pascal_spaced(), "Pascal Case"),
            PascalNames::MixedCase => assert_eq!(v.pascal_spaced(), "Mixed Case"),
            PascalNames::MixedCase2 => assert_eq!(v.pascal_spaced(), "Mixed Case2"),
            PascalNames::MixedCaseThree => assert_eq!(v.pascal_spaced(), "Mixed Case Three"),
            PascalNames::MixedCaseFour4 => assert_eq!(v.pascal_spaced(), "Mixed Case Four4"),
        }
    }

    for v in PascalNames::list() {
        match v {
            PascalNames::PascalCase => {
                assert_eq!(v, PascalNames::from_pascal_spaced("Pascal Case").unwrap())
            }
            PascalNames::MixedCase => {
                assert_eq!(v, PascalNames::from_pascal_spaced("Mixed Case").unwrap())
            }
            PascalNames::MixedCase2 => {
                assert_eq!(v, PascalNames::from_pascal_spaced("Mixed Case2").unwrap())
            }
            PascalNames::MixedCaseThree => {
                assert_eq!(
                    v,
                    PascalNames::from_pascal_spaced("Mixed Case Three").unwrap()
                )
            }
            PascalNames::MixedCaseFour4 => {
                assert_eq!(
                    v,
                    PascalNames::from_pascal_spaced("Mixed Case Four4").unwrap()
                )
            }
        }
    }
}

#[test]
fn pretty_print_1() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        pub enum PrettyPrint {
            A,
            B,
            C,
        }
    }
    assert_eq!(
        PrettyPrint::pretty_print(),
        r##"#[derive(Debug, PartialEq)]
pub enum PrettyPrint {
    A,
    B,
    C,
}"##
    );
}

#[test]
fn pretty_print_2() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        pub enum PrettyPrint {
            A = 10,
            B = 20,
            C = 30,
        }
    }

    assert_eq!(
        PrettyPrint::pretty_print(),
        r##"#[derive(Debug, PartialEq)]
#[derive(Clone)]
pub enum PrettyPrint {
    A = 10,
    B = 20,
    C = 30,
}"##
    );
}
