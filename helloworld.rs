use std::env;

fn main() {
    println!("Please enter your Name");

    let args: Vec<String> = env::args().collect();

    let name = &args[1];
    let filename = &args[2];

    println!("Name is {}", name);
}
