use enum_ext::enum_extend;

#[enum_extend]
pub struct NotAnEnum {
    x: i32,
}

fn main() {}
