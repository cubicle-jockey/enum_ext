use enum_ext::enum_extend;

#[enum_extend(IntType = "f32")]
pub enum BadIntTypeAttr {
    A,
    B,
}

fn main() {}
