use sqlx_extension_common::traits::sql_describe::SqlDescribe;
use sqlx_extension_macros::Describe;

#[derive(Describe)]
#[table("test_table")]
pub struct Test {
    #[p_key]
    name: String,
    #[p_key]
    #[rename("newTest")]
    test: i32,
}

fn main() {
    println!("{}", Test::delete_by_pk());
}
