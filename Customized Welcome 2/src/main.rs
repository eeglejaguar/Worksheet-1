// Create a Rust program that asks the user for their name and then prints a
// personalized greeting

use std::io;

fn main() {

    let mut name=String::new();
    println!("Hey!! What is your name?");
    io::stdin().read_line(&mut name).expect("Can,t take an Input");

    println!("Hey {} Nice to meet u <3!!",name);
}
