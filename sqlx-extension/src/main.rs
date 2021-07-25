use sqlx_extension_macros::Describe;

#[derive(Describe)]
pub struct Test {
    name: String,
    test: i32
}

fn main() {
}
