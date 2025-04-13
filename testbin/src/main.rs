use staticrypt::{sc, use_staticrypt};

use_staticrypt!();

fn main() {
    println!("My name is {}", sc!("Voldemort"));
}
