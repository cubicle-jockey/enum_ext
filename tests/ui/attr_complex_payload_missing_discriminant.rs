use enum_ext::enum_extend;

#[enum_extend(IntType = "u8")]
pub enum ComplexAttr {
    Unit = 1,
    Payload(u8),
}

fn main() {}
