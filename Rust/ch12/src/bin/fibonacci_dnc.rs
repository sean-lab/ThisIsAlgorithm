#[derive(Clone, Copy)]
struct Matrix2x2 {
    data: [[u64; 2]; 2],
}

fn matrix2x2_multiply(a: Matrix2x2, b: Matrix2x2) -> Matrix2x2 {
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

fn matrix2x2_power(mut a: Matrix2x2, n: i32) -> Matrix2x2 {
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

fn fibonacci(n: i32) -> u64 {
    let a = Matrix2x2 {
        data: [[1, 1], [1, 0]],
    };
    let a = matrix2x2_power(a, n);
    a.data[0][1]
}

fn main() {
    let n = 46;
    let result = fibonacci(n);
    println!("Fibonacci({}) = {}", n, result);
}
