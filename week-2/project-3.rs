fn main() {
    let p: f64 = 510000.0; // Principal
    let r: f64 = 5.0;      // Rate of interest
    let t: f64 = 3.0;      // Time in years

    let a = p * (1.0 + (r / 100.0)).powf(t);

    let ci = a - p;
    println!("Compound Interest is {}", ci);
}