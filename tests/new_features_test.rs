use enum_ext::enum_extend;

#[test]
fn test_snake_case_conversion() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum TestEnum {
        SimpleVariant,
        InQA,
        FinalCodeReview,
    }

    assert_eq!(TestEnum::SimpleVariant.snake_case(), "simple_variant");
    assert_eq!(TestEnum::InQA.snake_case(), "in_qa");
    assert_eq!(TestEnum::FinalCodeReview.snake_case(), "final_code_review");

    assert_eq!(
        TestEnum::from_snake_case("simple_variant"),
        Some(TestEnum::SimpleVariant)
    );
    assert_eq!(TestEnum::from_snake_case("in_qa"), Some(TestEnum::InQA));
    assert_eq!(
        TestEnum::from_snake_case("final_code_review"),
        Some(TestEnum::FinalCodeReview)
    );
    assert_eq!(TestEnum::from_snake_case("nonexistent"), None);
}

#[test]
fn test_kebab_case_conversion() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum TestEnum {
        SimpleVariant,
        InQA,
        FinalCodeReview,
    }

    assert_eq!(TestEnum::SimpleVariant.kebab_case(), "simple-variant");
    assert_eq!(TestEnum::InQA.kebab_case(), "in-qa");
    assert_eq!(TestEnum::FinalCodeReview.kebab_case(), "final-code-review");

    assert_eq!(
        TestEnum::from_kebab_case("simple-variant"),
        Some(TestEnum::SimpleVariant)
    );
    assert_eq!(TestEnum::from_kebab_case("in-qa"), Some(TestEnum::InQA));
    assert_eq!(
        TestEnum::from_kebab_case("final-code-review"),
        Some(TestEnum::FinalCodeReview)
    );
    assert_eq!(TestEnum::from_kebab_case("nonexistent"), None);
}

#[test]
fn test_neighbor_methods() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum TestEnum {
        First,
        Second,
        Third,
    }

    // Test next (wrapping)
    assert_eq!(TestEnum::First.next(), &TestEnum::Second);
    assert_eq!(TestEnum::Second.next(), &TestEnum::Third);
    assert_eq!(TestEnum::Third.next(), &TestEnum::First); // wraps around

    // Test previous (wrapping)
    assert_eq!(TestEnum::First.previous(), &TestEnum::Third); // wraps around
    assert_eq!(TestEnum::Second.previous(), &TestEnum::First);
    assert_eq!(TestEnum::Third.previous(), &TestEnum::Second);

    // Test next_linear (no wrapping)
    assert_eq!(TestEnum::First.next_linear(), Some(&TestEnum::Second));
    assert_eq!(TestEnum::Second.next_linear(), Some(&TestEnum::Third));
    assert_eq!(TestEnum::Third.next_linear(), None); // no wrapping

    // Test previous_linear (no wrapping)
    assert_eq!(TestEnum::First.previous_linear(), None); // no wrapping
    assert_eq!(TestEnum::Second.previous_linear(), Some(&TestEnum::First));
    assert_eq!(TestEnum::Third.previous_linear(), Some(&TestEnum::Second));
}

#[test]
fn test_validation_utilities() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum TestEnum {
        First,
        Second,
        Third,
    }

    // Test is_first and is_last
    assert!(TestEnum::First.is_first());
    assert!(!TestEnum::Second.is_first());
    assert!(!TestEnum::Third.is_first());

    assert!(!TestEnum::First.is_last());
    assert!(!TestEnum::Second.is_last());
    assert!(TestEnum::Third.is_last());

    // Test comes_before and comes_after
    assert!(TestEnum::First.comes_before(&TestEnum::Second));
    assert!(TestEnum::First.comes_before(&TestEnum::Third));
    assert!(TestEnum::Second.comes_before(&TestEnum::Third));
    assert!(!TestEnum::Second.comes_before(&TestEnum::First));

    assert!(TestEnum::Second.comes_after(&TestEnum::First));
    assert!(TestEnum::Third.comes_after(&TestEnum::First));
    assert!(TestEnum::Third.comes_after(&TestEnum::Second));
    assert!(!TestEnum::First.comes_after(&TestEnum::Second));
}

#[test]
fn test_filtering_methods() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum TestEnum {
        TestOne,
        TestTwo,
        SampleOne,
        SampleTwo,
        ExampleTest,
    }

    // Test variants_containing
    let containing_test = TestEnum::variants_containing("Test");
    assert_eq!(containing_test.len(), 3);
    assert!(containing_test.contains(&&TestEnum::TestOne));
    assert!(containing_test.contains(&&TestEnum::TestTwo));
    assert!(containing_test.contains(&&TestEnum::ExampleTest));

    // Test variants_starting_with
    let starting_with_test = TestEnum::variants_starting_with("Test");
    assert_eq!(starting_with_test.len(), 2);
    assert!(starting_with_test.contains(&&TestEnum::TestOne));
    assert!(starting_with_test.contains(&&TestEnum::TestTwo));

    // Test variants_ending_with
    let ending_with_one = TestEnum::variants_ending_with("One");
    assert_eq!(ending_with_one.len(), 2);
    assert!(ending_with_one.contains(&&TestEnum::TestOne));
    assert!(ending_with_one.contains(&&TestEnum::SampleOne));
}

#[test]
fn test_batch_operations() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum TestEnum {
        First,
        Second,
        Third,
        Fourth,
        Fifth,
    }

    // Test slice
    let slice_result = TestEnum::slice(1, 4);
    assert_eq!(
        slice_result,
        &[TestEnum::Second, TestEnum::Third, TestEnum::Fourth]
    );

    // Test range
    let range_result = TestEnum::range(0..3);
    assert_eq!(
        range_result,
        &[TestEnum::First, TestEnum::Second, TestEnum::Third]
    );

    // Test first_n
    let first_3 = TestEnum::first_n(3);
    assert_eq!(
        first_3,
        &[TestEnum::First, TestEnum::Second, TestEnum::Third]
    );

    // Test last_n
    let last_2 = TestEnum::last_n(2);
    assert_eq!(last_2, &[TestEnum::Fourth, TestEnum::Fifth]);

    // Test edge cases
    assert_eq!(TestEnum::slice(10, 20), &[]); // out of bounds
    assert_eq!(TestEnum::slice(3, 1), &[]); // start >= end
}

#[test]
fn test_metadata_extraction() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum TestEnum {
        FirstVariant,
        SecondVariant,
        ThirdVariant,
    }

    // Test variant_name
    assert_eq!(TestEnum::FirstVariant.variant_name(), "FirstVariant");
    assert_eq!(TestEnum::SecondVariant.variant_name(), "SecondVariant");
    assert_eq!(TestEnum::ThirdVariant.variant_name(), "ThirdVariant");

    // Test variant_names
    let names = TestEnum::variant_names();
    assert_eq!(names, vec!["FirstVariant", "SecondVariant", "ThirdVariant"]);
}

#[test]
#[cfg(feature = "random")]
fn test_random_selection() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum TestEnum {
        First,
        Second,
        Third,
    }

    // Test that random returns a valid variant
    let random_variant = TestEnum::random();
    assert!(matches!(
        random_variant,
        &TestEnum::First | &TestEnum::Second | &TestEnum::Third
    ));

    // Test with custom RNG
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    let mut rng = StdRng::seed_from_u64(42);
    let random_with_rng = TestEnum::random_with_rng(&mut rng);
    assert!(matches!(
        random_with_rng,
        &TestEnum::First | &TestEnum::Second | &TestEnum::Third
    ));
}
