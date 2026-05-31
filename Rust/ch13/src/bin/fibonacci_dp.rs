// 동적 계획법 피보나치 (Fibonacci - DP) - 로직은 ch13::fibonacci_dp 모듈에 있다.
use ch13::fibonacci_dp::fibonacci;

fn main() {
    let n = 46;
    let result = fibonacci(n);
    println!("Fibonacci({})  = {} ", n, result);
}
