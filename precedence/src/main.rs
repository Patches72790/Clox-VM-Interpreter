extern crate precedence_macro;
use precedence_macro::make_precedence;

#[make_precedence(42)]
struct TestStruct(u8);

fn main() {
    let t = TestStruct(12);

    println!("Here is my attribute precedence num: {}", *t);
}
