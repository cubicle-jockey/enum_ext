#![allow(unused, dead_code)]
use enum_ext::{enum_ext, enum_extend};

fn main() {
    // Both the `enum_ext` and `enum_extend` macros provide the same functionality. They only differ in the way they are called.

    // For demonstration purposes, here is one example of using the `enum_ext` macro. The rest of the examples will use the enum_extend macro.
    enum_ext! {
        #[derive(Debug, PartialEq)]
        enum MyEnum {
            One,
            Two,
            Three,
        }
    }

    // See the following examples demonstrating utility functions
    ordinal_example();
    list_and_count_example();
    iter_example();
    pascal_spaced_example();
    string_case_example();
    navigation_and_validation_example();
    filtering_batch_and_metadata_example();
    pretty_print_example();
}

// Examples demonstrating utility functions

fn ordinal_example() {
    // Why is `ordinal` useful?
    // In most cases, non-discriminant enums are serialized as integers based on their index.
    // These functions make it easy to convert between the enum and the integer.

    #[enum_extend]
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        One,   // ordinal 0
        Two,   // ordinal 1
        Three, // ordinal 2
    }

    let two = MyEnum::Two;
    // `ordinal()` returns the index of the variant in the enum, starting from 0.
    assert_eq!(two.ordinal(), 1);

    // The inverse of `ordinal()` are `ref_from_ordinal()` and `from_ordinal()`, which return the variant from the index.
    assert_eq!(MyEnum::ref_from_ordinal(1), Some(&MyEnum::Two));
    assert_eq!(MyEnum::ref_from_ordinal(3), None);

    // NOTE: `from_ordinal()` is only available if the enum derives `Clone`.
    {
        #[enum_extend]
        #[derive(Debug, PartialEq, Clone)] // <- 'Clone' has been added
        enum MyEnumWithClone {
            One,   // ordinal 0
            Two,   // ordinal 1
            Three, // ordinal 2
        }

        // Now `from_ordinal()` is available.
        let two = MyEnumWithClone::from_ordinal(1); // <- now this works
        assert_eq!(two, Some(MyEnumWithClone::Two));
    }

    // `valid_ordinal()` returns true if the index is valid.
    assert_eq!(MyEnum::valid_ordinal(1), true);
    assert_eq!(MyEnum::valid_ordinal(3), false);

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //
    print_eq_line();
    println!("ordinal example");
    print_dash_line();
    println!("{}", MyEnum::pretty_print());
    println!("let two = MyEnum::Two;");
    println!("two.ordinal() = {}", two.ordinal());
    print_dash_line();
    for x in 0..=MyEnum::count() {
        if let Some(variant) = MyEnum::ref_from_ordinal(x) {
            // only doing it this way because {:?} doesn't print the '&' ref when printing Option<&T>
            println!("MyEnum::ref_from_ordinal({}) = Some(&{:?})", x, variant);
        } else {
            println!("MyEnum::ref_from_ordinal({}) = None", x);
        }
    }
    print_dash_line();
    for x in 0..=MyEnum::count() {
        println!(
            "MyEnum::valid_ordinal({}) = {}",
            x,
            MyEnum::valid_ordinal(x)
        );
    }
    print_dash_line();
    println!("with derive(Clone)...");
    #[enum_extend]
    #[derive(Debug, PartialEq, Clone)] // <- notice 'Clone' has been added
    enum MyEnumWithClone {
        One,   // ordinal 0
        Two,   // ordinal 1
        Three, // ordinal 2
    }
    println!("{}", MyEnumWithClone::pretty_print());
    // now `from_ordinal()` is available.
    for x in 0..=MyEnumWithClone::count() {
        println!(
            "MyEnumWithClone::from_ordinal({}) = {:?}",
            x,
            MyEnumWithClone::from_ordinal(x)
        );
    }
}

fn list_and_count_example() {
    // Why is `list` useful?
    // Now that `enum_ext` has added `iter()`, `list()` and `count()` are not as useful
    // as they once were, but may still add value in some cases.

    #[enum_extend]
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        One,
        Two,
        Three,
    }

    // `list()` returns an array of all the variants in the enum
    let list = MyEnum::list();
    assert_eq!(list, [MyEnum::One, MyEnum::Two, MyEnum::Three]);

    // `count()` returns the number of variants in the enum.
    // In this example, MyEnum::count() returns 3.
    let count = MyEnum::count();
    for inx in 0..count {
        let variant = &MyEnum::list()[inx];
        assert_eq!(variant.ordinal(), inx);
    }

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //
    print_eq_line();
    println!("list and count example");
    print_dash_line();
    println!("{}", MyEnum::pretty_print());
    println!("MyEnum::list() = {:?}", list);
    println!("MyEnum::count() = {}", count);
}

fn iter_example() {
    // Why is `iter` useful?
    // It's an iterator!.

    #[enum_extend]
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        One,
        Two,
        Three,
    }

    // `iter()` returns an iterator over the variants in the enum.
    let mut iter = MyEnum::iter();
    assert_eq!(iter.next(), Some(&MyEnum::One));
    assert_eq!(iter.next(), Some(&MyEnum::Two));
    assert_eq!(iter.next(), Some(&MyEnum::Three));
    assert_eq!(iter.next(), None);

    // A real-world use case is more like this:
    for variant in MyEnum::iter() {
        // Do something with each variant
    }

    // Since it's an iterator, you can use all iterator functions like `map()`, `filter()`, `fold()`, etc.
    for (inx, variant) in MyEnum::iter().enumerate() {
        assert_eq!(inx, variant.ordinal());
    }

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //
    print_eq_line();
    println!("iter example");
    print_dash_line();
    println!("{}", MyEnum::pretty_print());
    println!("let mut iter = MyEnum::iter();");
    for x in MyEnum::iter() {
        println!("iter.next() = {:?}", x);
    }
    println!("iter.next() = None");
}

fn pascal_spaced_example() {
    // Why is `pascal_spaced` useful?
    // `pascal_spaced()` returns the variant name in spaced PascalCase.
    // This can be useful for displaying the enum variant in a more readable format,
    // as well as for parsing input.  For example, to and from display in UIs.

    #[enum_extend]
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        Backlog,
        InDev,
        InQA,
        CodeReview,
        FinalQA,
        FinalCodeReview,
        Accepted,
        Done,
    }

    // `pascal_spaced()` returns the variant name in spaced PascalCase.
    // MyEnum::InQA.pascal_spaced() returns "In QA"
    let in_qa = MyEnum::InQA;
    assert_eq!(in_qa.pascal_spaced(), "In QA");

    // `from_pascal_spaced()` is the inverse of `pascal_case().
    let in_qa_2 = MyEnum::from_pascal_spaced("In QA");
    assert_eq!(in_qa_2, Some(MyEnum::InQA));

    for variant in MyEnum::iter() {
        let pas_case = variant.pascal_spaced();
        let from_pas_case = MyEnum::from_pascal_spaced(pas_case);
        assert_eq!(from_pas_case.as_ref(), Some(variant));
    }

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //
    print_eq_line();
    println!("pascal spaced example");
    print_dash_line();
    println!("{}", MyEnum::pretty_print());
    println!("let in_qa = MyEnum::InQA;");
    println!("in_qa.pascal_spaced() = \"{}\"", in_qa.pascal_spaced());
    println!(
        "MyEnum::from_pascal_spaced(\"In QA\") = {:?}",
        MyEnum::from_pascal_spaced("In QA")
    );
}

fn string_case_example() {
    #[enum_extend]
    #[derive(Debug, PartialEq)]
    enum TicketStatus {
        Backlog,
        InDev,
        InQA,
        FinalCodeReview,
    }

    // snake_case + reverse conversion
    assert_eq!(TicketStatus::InQA.snake_case(), "in_qa");
    assert_eq!(
        TicketStatus::FinalCodeReview.snake_case(),
        "final_code_review"
    );
    assert_eq!(
        TicketStatus::from_snake_case("final_code_review"),
        Some(TicketStatus::FinalCodeReview)
    );

    // kebab-case + reverse conversion
    assert_eq!(TicketStatus::InQA.kebab_case(), "in-qa");
    assert_eq!(
        TicketStatus::FinalCodeReview.kebab_case(),
        "final-code-review"
    );
    assert_eq!(
        TicketStatus::from_kebab_case("final-code-review"),
        Some(TicketStatus::FinalCodeReview)
    );

    // variant_name() returns the exact variant identifier as a string
    assert_eq!(
        TicketStatus::FinalCodeReview.variant_name(),
        "FinalCodeReview"
    );

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //

    print_eq_line();
    println!("string case example");
    print_dash_line();
    println!("{}", TicketStatus::pretty_print());
    for v in TicketStatus::iter() {
        println!(
            "{:?} => snake: \"{}\", kebab: \"{}\", variant_name: \"{}\"",
            v,
            v.snake_case(),
            v.kebab_case(),
            v.variant_name()
        );
    }
}

fn navigation_and_validation_example() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum Stage {
        One,
        Two,
        Three,
    }

    // wrapping navigation
    assert_eq!(Stage::One.next(), &Stage::Two);
    assert_eq!(Stage::Three.next(), &Stage::One);
    assert_eq!(Stage::One.previous(), &Stage::Three);

    // linear navigation
    assert_eq!(Stage::One.next_linear(), Some(&Stage::Two));
    assert_eq!(Stage::Three.next_linear(), None);
    assert_eq!(Stage::One.previous_linear(), None);
    assert_eq!(Stage::Three.previous_linear(), Some(&Stage::Two));

    // validation/comparison helpers
    assert!(Stage::One.is_first());
    assert!(Stage::Three.is_last());
    assert!(Stage::One.comes_before(&Stage::Three));
    assert!(Stage::Three.comes_after(&Stage::One));

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //

    print_eq_line();
    println!("navigation and validation example");
    print_dash_line();
    println!("{}", Stage::pretty_print());
    println!("Stage::One.next() = {:?}", Stage::One.next());
    println!("Stage::Three.next() = {:?} (wraps)", Stage::Three.next());
    println!(
        "Stage::Three.next_linear() = {:?}",
        Stage::Three.next_linear()
    );
    println!("Stage::One.is_first() = {}", Stage::One.is_first());
    println!(
        "Stage::One.comes_before(&Stage::Three) = {}",
        Stage::One.comes_before(&Stage::Three)
    );
}

fn filtering_batch_and_metadata_example() {
    #[enum_extend]
    #[derive(Debug, Clone, PartialEq)]
    enum Workflow {
        TestOne,
        TestTwo,
        SampleOne,
        SampleTwo,
        ExampleTest,
    }

    // filtering helpers
    let containing_test = Workflow::variants_containing("Test");
    assert_eq!(containing_test.len(), 3);

    let starting_with_test = Workflow::variants_starting_with("Test");
    assert_eq!(starting_with_test.len(), 2);

    let ending_with_one = Workflow::variants_ending_with("One");
    assert_eq!(ending_with_one.len(), 2);

    // batch helpers
    assert_eq!(
        Workflow::slice(1, 4),
        &[Workflow::TestTwo, Workflow::SampleOne, Workflow::SampleTwo]
    );
    assert_eq!(
        Workflow::range(0..3),
        &[Workflow::TestOne, Workflow::TestTwo, Workflow::SampleOne]
    );
    assert_eq!(
        Workflow::first_n(2),
        &[Workflow::TestOne, Workflow::TestTwo]
    );
    assert_eq!(
        Workflow::last_n(2),
        &[Workflow::SampleTwo, Workflow::ExampleTest]
    );

    // metadata helpers
    assert_eq!(Workflow::ExampleTest.variant_name(), "ExampleTest");
    assert_eq!(
        Workflow::variant_names(),
        vec![
            "TestOne",
            "TestTwo",
            "SampleOne",
            "SampleTwo",
            "ExampleTest"
        ]
    );
}

fn pretty_print_example() {
    // Why is pretty_print useful?
    // `pretty_print()` returns a formatted string displaying the enum name and its variants.
    // This can be beneficial for debugging, testing, and logging.
    // For instance, logging the enum name with all its variants during startup can help
    // identify serialization mismatches between services. It's not uncommon for one
    // service to have an outdated version of an enum compared to another service.

    #[enum_extend]
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        One,
        Two,
        Three,
    }

    // `pretty_print()` returns a string that shows the enum name and all the variants.
    let pretty = MyEnum::pretty_print();
    assert_eq!(
        pretty,
        r##"#[derive(Debug, PartialEq)]
enum MyEnum {
    One,
    Two,
    Three,
}"##
    );

    // ************************************************************************** //
    // *** From here down just prints a summary of what this example covered. *** //
    // ************************************************************************** //
    print_eq_line();
    println!("pretty print example");
    print_dash_line();
    println!("{}", pretty);
    println!("^^^ this (above) is what MyEnum::pretty_print() returns");
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
    fn ordinal_example_test() {
        ordinal_example();
    }
    #[test]
    fn list_and_count_example_test() {
        list_and_count_example();
    }
    #[test]
    fn pascal_spaced_example_test() {
        pascal_spaced_example();
    }
    #[test]
    fn iter_example_test() {
        iter_example();
    }
    #[test]
    fn string_case_example_test() {
        string_case_example();
    }
    #[test]
    fn navigation_and_validation_example_test() {
        navigation_and_validation_example();
    }
    #[test]
    fn filtering_batch_and_metadata_example_test() {
        filtering_batch_and_metadata_example();
    }
    #[test]
    fn pretty_print_example_test() {
        pretty_print_example();
    }
}
