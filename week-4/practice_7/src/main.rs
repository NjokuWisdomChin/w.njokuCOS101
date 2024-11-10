use std::io;

fn main() {

    let mut input =  String::new();

    println!("Enter your height (in centimeters): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut num:i32 = input.trim().parse().expect("Input not an integer");

    while num < 10 {
        println!(" inside loop value is {}", num);
        num+=1;
    }
    println!("outside loop number value is {}", num);
}