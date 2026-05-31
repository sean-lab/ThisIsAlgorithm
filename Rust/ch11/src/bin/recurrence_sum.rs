fn recurrence_sum(data: &[i32]) -> i32 {
    if data.len() == 1 {
        data[0]
    } else {
        recurrence_sum(&data[1..]) + data[0]
    }
}

fn main() {
    let mut data = [0i32; 150];
    for i in 0..150 {
        data[i] = (i + 1) as i32;
    }

    println!("Sum:{}", recurrence_sum(&data[..55]));
}
