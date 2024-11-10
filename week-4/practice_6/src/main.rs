use std::io;

fn main() {

    let mut input1 =  String::new();
    let mut input2 =  String::new();

    println!("Enter lower bound: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lowerb:i32 = input1.trim().parse().expect("Input not an integer");

    println!("Enter lower bound: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upperb:i32 = input2.trim().parse().expect("Input not an integer");

    for  x in lowerb..upperb 
    {
        println!("Count Level is {}", x);
    }

    
}