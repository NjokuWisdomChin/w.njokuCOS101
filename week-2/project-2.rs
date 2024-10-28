fn main() {
    let toshiba:f64 = 450000.0;
    let mac:f64 = 1500000.0;
    let hp:f64 = 750000.0;
    let dell:f64 = 2850000.0;
    let acer:f64 = 250000.0;

    let sum = (2.0*toshiba)+(1.0*mac)+(3.0*hp)+(3.0*dell)+(2.0*acer);
    println!("Sum total is{}", sum);
    
    let totalquality= 2.0+1.0+3.0+3.0+1.0;
    let ave = sum/totalquality;
	println!("Average {}", ave);
}