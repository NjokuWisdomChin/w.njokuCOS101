use std::io;

fn main() {

    let mut input1 =  String::new();
    let mut input2 =  String::new();
    let mut input3 =  String::new();
    
    println!("\n Enter a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Input not an integer");

    println!("\n Enter b: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Input not an integer");

    println!("\n Enter c:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Input not an integer");
    
    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 
    {  
        let sqrt_discriminant = discriminant.sqrt();
        let root1 = (-b + sqrt_discriminant) / (2.0 * a);
        let root2 = (-b - sqrt_discriminant) / (2.0 * a);
    
        println!("\n The roots of this equation are: {} and {}", root1, root2)
    } 
    else if discriminant == 0.0 
    {
        let sqrt_discriminant = discriminant.sqrt();
        let root = (-b + sqrt_discriminant) / (2.0 * a);

        println!("\n The root of this equation is: {}", root)

    }
    else if discriminant < 0.0 
    {
        println!("\n There is no root for this quadratic equation")
    }

}