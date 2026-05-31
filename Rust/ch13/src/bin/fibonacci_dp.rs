fn fibonacci(n: i32) -> u64 {
    if n == 0 || n == 1 {
        return n as u64;
    }

    let mut fibonacci_table = vec![0u64; (n + 1) as usize];

    fibonacci_table[0] = 0;
    fibonacci_table[1] = 1;

    for i in 2..=n as usize {
        fibonacci_table[i] = fibonacci_table[i - 1] + fibonacci_table[i - 2];
    }

    fibonacci_table[n as usize]
}

fn main() {
    let n = 46;
    let result = fibonacci(n);
    println!("Fibonacci({})  = {} ", n, result);
}
