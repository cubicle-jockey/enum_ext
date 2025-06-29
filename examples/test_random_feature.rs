use enum_ext::enum_ext;

enum_ext! {
    #[derive(Debug, PartialEq)]
    pub enum TestEnum {
        A,
        B,
        C,
    }
}

fn main() {
    println!("Testing enum without random feature...");

    // These should always work
    println!("Count: {}", TestEnum::count());
    println!("First variant: {:?}", TestEnum::list()[0]);

    // Test that we can access the random methods when feature is enabled
    #[cfg(feature = "random")]
    {
        println!("Testing random methods...");
        let random_variant = TestEnum::random();
        println!("Random variant: {:?}", random_variant);

        let mut rng = rand::rng();
        let random_with_rng = TestEnum::random_with_rng(&mut rng);
        println!("Random with RNG: {:?}", random_with_rng);
    }

    #[cfg(not(feature = "random"))]
    {
        println!("Random feature is disabled - random methods should not be available");
    }

    println!("Test completed successfully!");
}
