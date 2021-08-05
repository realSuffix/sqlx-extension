use sqlx_extension_macros::{Describe, Entity};

#[derive(Describe, Entity, sqlx::FromRow, Debug)]
#[table("test_table")]
#[ident("(String,)")]
pub struct Test {
    #[p_key]
    name: String,
    #[rename("newTest")]
    test: String,
}

#[tokio::main]
async fn main() {}
