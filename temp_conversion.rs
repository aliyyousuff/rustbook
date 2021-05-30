
// Convert temperatures between Fahrenheit and Celsius.

// compiler version: rustc 1.52.1
use std::io;

fn farenheit_celcius(farenheit : f64) -> f64
{
    ((farenheit - 32.0) * 32.0 ) / 9.0
}

fn celcius_farenheit(celcius : f64) -> f64
{
    (celcius * 1.8) + 32.0
}
fn main() 
{
    println!("This is a temperature converting program that converts temperature from farenhit to Celsius or vice-versa.");

    let mut flag = String::new();
    println!("Type 1 for Farenheit to Celcius or type 2 for celcius to Farenheit:");

    // Taking input from user
    io::stdin().read_line(&mut flag).expect("Failed to read lines");

    // Converting flag to integer
    let flag: i8 = flag.trim().parse().expect("Invalid input, please check the input");
    //println!("{}", flag);

    if flag == 1
    {
        println!("Please your temperature in Farenheit: ");
        let mut farenheit = String::new();
        io::stdin().read_line(&mut farenheit).expect("Failed to readline");
        let farenheit: f64 = farenheit.trim().parse().expect("Invalid input");

        println!("Temperature is {} {}", farenheit_celcius(farenheit), "degree celcius");

    }
    else if flag == 2
    {
        println!("Please your temperature in Celcius: ");
        let mut celcius = String::new();
        io::stdin().read_line(&mut celcius).expect("Failed to readline");
        let celcius: f64 = celcius.trim().parse().expect("Invalid input");

        println!("Temperature is {} {}", celcius_farenheit(celcius), "degree farenheit.");
    }

}
   