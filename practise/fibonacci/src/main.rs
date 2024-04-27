use std::io;

fn main() {
    println!("Enter n:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");
    println!("n = {}", n);
    println!("Fibonacci number at position {} is {}", n, fibo(n));
}

fn fibo(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 || n == 2 {
        return 1;
    } else {
        return fibo(n - 1) + fibo(n - 2);
    }
}
