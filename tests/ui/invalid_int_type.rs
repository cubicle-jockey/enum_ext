use enum_ext::enum_ext;

enum_ext!(
    #[enum_def(IntType = "f32")]
    pub enum BadIntType {
        A,
        B,
    }
);

fn main() {}
