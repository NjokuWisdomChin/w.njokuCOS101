use std::io;

fn main() {

    let mut input1 =  String::new();
    let mut input2 =  String::new();

    
    println!("\n Enter age: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let age:i32 = input1.trim().parse().expect("Input not an integer");

    println!("\n Enter your level of experience for 1-10 (6 and above will be considered experienced): ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let experience:i32 = input2.trim().parse().expect("Input not an integer");


    if experience>5 && age>= 40
    {
        println!("\n Incentive of the employee is N1,560,000 per month");
    } 
    else if experience>5 && age<40 && age>=28
    {
        println!("\n Incentive of the employee is N1,480,000 per month");

    }
    else if experience>5 && age<28
    {
        println!("\n Incentive of the employee is N1,300,000 per month");
    }
    else if experience<5
    {
        println!("\n Incentive of the employee is N100,000 per month");
    }
}