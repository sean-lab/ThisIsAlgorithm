fn power(base: i32, exponent: i32) -> u64 {
    if exponent == 1 {
        return base as u64;
    } else if base == 0 {
        return 1;
    }

    if exponent % 2 == 0 {
        let new_base = power(base, exponent / 2);
        new_base.wrapping_mul(new_base)
    } else {
        let new_base = power(base, (exponent - 1) / 2);
        new_base.wrapping_mul(new_base).wrapping_mul(base as u64)
    }
}

fn main() {
    let base = 2;
    let exponent = 30;
    println!("{}^{} = {}", base, exponent, power(base, exponent));
}
