use std::env;

fn main() {
    let x = env::args().nth(1).expect("Invalid");
    println!("{}", x);
}
