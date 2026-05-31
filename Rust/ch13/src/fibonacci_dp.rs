// 동적 계획법 피보나치 (Fibonacci - Dynamic Programming)
// Clang/13/FibonacciDP 포팅.

pub fn fibonacci(n: i32) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn small_values() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn matches_main_program() {
        assert_eq!(fibonacci(46), 1836311903);
    }
}
