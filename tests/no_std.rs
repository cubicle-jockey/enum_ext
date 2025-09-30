#![no_std]
extern crate alloc;

use alloc::vec::Vec;

use enum_ext::enum_extend;

// simple tests to ensure that the crate compiles with no_std enabled

#[test]
fn test_no_std_1() {
    #[enum_extend]
    #[derive(Debug, PartialEq)]
    pub enum NoStd {
        A,
        B,
        C,
    }
}

#[test]
fn test_no_std_2() {
    #[enum_extend(IntType = "u8")]
    #[derive(Debug, PartialEq)]
    pub enum NoStd2 {
        A = 10,
        B = 11,
        C = 12,
    }
}

#[test]
fn test_no_std_3() {
    #[enum_extend(IntType = "u32")]
    #[derive(Debug, PartialEq, Copy)]
    pub enum NoStd3 {
        A = 10,
        B = 11,
        CodeReview = 12,
    }

    const X: u32 = NoStd3::B.as_u32();
    assert_eq!(X, 11);

    assert_eq!(NoStd3::CodeReview.pascal_spaced(), "Code Review");
    assert_eq!(
        NoStd3::from_pascal_spaced("Code Review"),
        Some(NoStd3::CodeReview)
    );
}
