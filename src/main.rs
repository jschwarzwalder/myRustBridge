fn main() {
//    println!("Hello, world from Portland");

    println!("Read the Docs at doc.rust-lang.org");
    println!("or");
    println!("rustup doc --std");


    let name = "Jami";
    let age: i32 = 50;
    println!("Hi, {}! You are {} years old.", name,  add_fifty(age));
}

fn add_fifty(n: i32) -> i32 {
    n + 50
}