use enum_ext::enum_ext;

enum_ext!(
    #[enum_def(IntType = "u8")]
    pub enum Complex {
        Unit = 1,
        Payload(u8),
    }
);

fn main() {}
