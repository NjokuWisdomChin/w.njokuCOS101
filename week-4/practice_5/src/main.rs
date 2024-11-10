use std::io;

fn main() {

    let mut input =  String::new();

    println!("Enter your height (in centimeters): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let height:f32 = input.trim().parse().expect("Input not an integer");

    if height >= 150.0 && height <=170.0
    {
        println!("You are an average height person");
    } 
    else if  height >= 170.0 && height <=195.0
    {
        println!("\n You are tall");

    } 
    else if height < 150.0 && height > 100.0
    {
        println!("\n You are a dwarf");
    }
    else
    {
        println!("\n Abnormal height");
    }

    
}