// nth fibonacci number generator using the recursion

// compiler version: rustc 1.52.1

use std::io;

fn fib( n : i64 ) -> i64
{
    if n == 0
    {
        0
    }
    else if n == 1 
    {
        1
    }
    else 
    {
        fib(n - 1) + fib (n -2)
    }
}


fn main()
{

    let mut number = String::new();
    println!("enter the nth number to know the fibonacci: ");

    // Taking input from user
    io::stdin().read_line(&mut number).expect("Failed to read lines");

    // Converting flag to integer
    let number: i64 = number.trim().parse().expect("Invalid input, please check the input");
    //println!("{}", flag);

    println!("the {}th fibonacci number is {} ", number ,fib(number));
}