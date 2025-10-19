use enum_ext::{enum_ext, enum_extend};

#[test]
fn discriminant_expression_proc() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        pub enum Test {
            X25 = 5 * 5,
            Y26 = 13 + 13,
            Z100 = 10 * (5 + 5),
        }
    }

    assert_eq!(Test::X25 as usize, 25);
    assert_eq!(Test::Y26 as usize, 26);
    assert_eq!(Test::Z100 as usize, 100);

    assert_eq!(Test::from_usize(25), Some(Test::X25));
    assert_eq!(Test::from_usize(26), Some(Test::Y26));
    assert_eq!(Test::from_usize(100), Some(Test::Z100));
    assert_eq!(Test::from_usize(27), None);
}

#[test]
fn discriminant_expression_attr() {
    #[enum_extend]
    #[derive(Debug, PartialEq)]
    enum Test {
        X25 = 5 * 5,
        Y26 = 13 + 13,
        Z100 = 10 * (5 + 5),
    }

    assert_eq!(Test::X25 as usize, 25);
    assert_eq!(Test::Y26 as usize, 26);
    assert_eq!(Test::Z100 as usize, 100);

    assert_eq!(Test::from_usize(25), Some(Test::X25));
    assert_eq!(Test::from_usize(26), Some(Test::Y26));
    assert_eq!(Test::from_usize(100), Some(Test::Z100));
    assert_eq!(Test::from_usize(27), None);
}

#[test]
fn discriminant_expression_with_inttype() {
    enum_ext! {
        #[derive(Debug, PartialEq)]
        #[enum_def(IntType = "i32")]
        pub enum T {
            A = 3 * 3,
            B = 0x10 + 5,
        }
    }

    assert_eq!(T::from_i32(9), Some(T::A));
    assert_eq!(T::from_i32(21), Some(T::B));
    assert_eq!(T::A.as_i32(), 9);
    assert_eq!(T::B.as_i32(), 21);
}
