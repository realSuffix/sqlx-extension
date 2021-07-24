use sqlx_extension_macros::Describe;

#[derive(Describe)]
pub struct Test {
    #[rename(new_name = "test")]
    name: String
}

fn main() {
}
