// 빠른 거듭제곱 (Fast Exponentiation) - 로직은 ch12::fast_exponentiation 모듈에 있다.
use ch12::fast_exponentiation::power;

fn main() {
    let base = 2;
    let exponent = 30;
    println!("{}^{} = {}", base, exponent, power(base, exponent));
}
