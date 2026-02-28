use enum_ext::enum_ext;

enum_ext!(
    #[enum_def(IntType = "u8")]
    #[enum_def(IntType = "i32")]
    pub enum DuplicateDef {
        A,
        B,
    }
);

fn main() {}
