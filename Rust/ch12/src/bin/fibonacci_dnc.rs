// 분할 정복 피보나치 (Fibonacci - Divide and Conquer) - 로직은 ch12::fibonacci_dnc 모듈에 있다.
use ch12::fibonacci_dnc::fibonacci;

fn main() {
    let n = 46;
    let result = fibonacci(n);
    println!("Fibonacci({}) = {}", n, result);
}
