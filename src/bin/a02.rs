extern crate derive_more;
use derive_more::{Add, Display, From, Into};
#[derive(PartialEq, From, Add, Display)]
struct MyInt(i32);

#[derive(PartialEq, From, Into)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(PartialEq, From, Add, Display)]
enum MyEnum {
    #[display(fmt = "int: {}", _0)]
    Int(i32),
    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

fn main() {
    println!("hello");
}
