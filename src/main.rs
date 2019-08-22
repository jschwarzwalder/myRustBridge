fn main() {
//    println!("Hello, world from Portland");

    println!("Read the Docs at doc.rust-lang.org");
    println!("or");
    println!("rustup doc --std");

    my_arrays();
}

fn who_am_i(){

    let name = "Jami";
    let mut age: 1;
    
    println!("Hi, {}! You are {} years old.", name,  age);
}

fn add_fifty(n: i32) -> i32 {
    n + 50
}

fn roller_coaster(){
    let height = 167u32;
    match height {
        0...150 => println!("You're too small to go on the rollercoaster."),
        150...200 => println!("You may go on the rollercoaster!"),
        _ => {
            println!("You're too tall to go on the rollercoaster.");
        },
    }
}

fn my_arrays(){
    let mut color = [255, 0, 255];
    color[0] = 100;
    println!("The color is {:?}", color);
    println!("Normally use Vectors");
    println!("Pretty print {:#}, color")
}