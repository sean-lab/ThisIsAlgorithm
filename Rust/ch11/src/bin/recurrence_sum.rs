// 재귀적 합계 (Recurrence Sum) - 로직은 ch11::recurrence_sum 모듈에 있다.
use ch11::recurrence_sum::recurrence_sum;

fn main() {
    let mut data = [0i32; 150];
    for i in 0..150 {
        data[i] = (i + 1) as i32;
    }

    println!("Sum:{}", recurrence_sum(&data[..55]));
}
