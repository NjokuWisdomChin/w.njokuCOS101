use std::io;

fn main() {

    let mut input1 =  String::new();
    let mut input2 =  String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    println!("\n Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:f32 = input2.trim().parse().expect("Input not an integer");

    if age >= 18.0 {
        println!("{}, welcome to the party!", input1);
    } else {
        println!("\n Oops, you are not of age to enter this party {}", input1)

    }
    
}