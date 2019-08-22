fn main() {
//    println!("Hello, world from Portland");

    println!("Read the Docs at doc.rust-lang.org");
    println!("or");
    println!("rustup doc --std");

    my_arrays();
    who_am_i();
}

fn who_am_i(){

    let name = "Jami";
    let mut age= 1;
    for i in 0..50 {
        age += i;
        if age > 1250 {
             panic!("aaaaa!");
        }
    }
    age = add_fifty(age);
    println!("Hi, {}! You are {} years old.", name,  age);
}

fn add_fifty(n: i32) -> i32 {
    n + 50
}



fn my_arrays(){
    let mut color = [255, 0, 255];
    color[0] = 100;
    println!("The color is {:?}", color);
    println!("Normally use Vectors");
    println!("Pretty print {:#?}", color);

    let mut prices = vec![30, 100, 2];
    prices[0] = 25;
    prices.push(40);
    println!("All the prices are: {:#?}", prices);


}

#[derive(PartialEq, Debug)] // necessary to compare enum values
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
#[test]
fn new_person_setup() {
    let light = TrafficLight::Yellow;
    assert!(light == TrafficLight::Yellow);
    assert_eq!(light, TrafficLight::Yellow);
    assert_ne!(light, TrafficLight::Red);
    assert_ne!(light, TrafficLight::Green);
}

#[test]
fn array_mutable(){
    let mut color = [255, 0, 255];
    color[0] = 100;
    assert_eq!(color[0],100);

}