use sqlx_extension_macros::Describe;

#[derive(Describe)]
pub struct Test {
    #[rename("test")]
    name: String
}

fn main() {
}
