// 분할 정복 피보나치 (Fibonacci - Divide and Conquer, 행렬 거듭제곱)
// Clang/12/FibonacciDnC 포팅.

#[derive(Clone, Copy)]
pub struct Matrix2x2 {
    pub data: [[u64; 2]; 2],
}

pub fn matrix2x2_multiply(a: Matrix2x2, b: Matrix2x2) -> Matrix2x2 {
    let mut c = Matrix2x2 { data: [[0; 2]; 2] };

    c.data[0][0] = a.data[0][0]
        .wrapping_mul(b.data[0][0])
        .wrapping_add(a.data[0][1].wrapping_mul(b.data[1][0]));
    c.data[0][1] = a.data[0][0]
        .wrapping_mul(b.data[1][0])
        .wrapping_add(a.data[0][1].wrapping_mul(b.data[1][1]));

    c.data[1][0] = a.data[1][0]
        .wrapping_mul(b.data[0][0])
        .wrapping_add(a.data[1][1].wrapping_mul(b.data[1][0]));
    c.data[1][1] = a.data[1][0]
        .wrapping_mul(b.data[1][0])
        .wrapping_add(a.data[1][1].wrapping_mul(b.data[1][1]));

    c
}

pub fn matrix2x2_power(mut a: Matrix2x2, n: i32) -> Matrix2x2 {
    if n > 1 {
        a = matrix2x2_power(a, n / 2);
        a = matrix2x2_multiply(a, a);

        if n & 1 != 0 {
            let b = Matrix2x2 {
                data: [[1, 1], [1, 0]],
            };
            a = matrix2x2_multiply(a, b);
        }
    }

    a
}

pub fn fibonacci(n: i32) -> u64 {
    let a = Matrix2x2 {
        data: [[1, 1], [1, 0]],
    };
    let a = matrix2x2_power(a, n);
    a.data[0][1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_fibonacci_values() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn matches_main_program() {
        assert_eq!(fibonacci(46), 1836311903);
    }

    #[test]
    fn matrix_power_identity_for_n_le_1() {
        // n <= 1 이면 행렬이 그대로 반환된다.
        let a = Matrix2x2 {
            data: [[1, 1], [1, 0]],
        };
        let r = matrix2x2_power(a, 1);
        assert_eq!(r.data, [[1, 1], [1, 0]]);
    }
}
