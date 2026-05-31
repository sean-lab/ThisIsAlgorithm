use std::io::{self, Read};

fn count_coins(amount: i32, coin_unit: i32) -> i32 {
    let mut coin_count = 0;
    let mut current_amount = amount;

    while current_amount >= coin_unit {
        coin_count += 1;
        current_amount -= coin_unit;
    }

    coin_count
}

fn get_change(price: i32, pay: i32, coin_units: &[i32], change: &mut [i32], size: usize) {
    let mut change_amount = pay - price;

    for i in 0..size {
        change[i] = count_coins(change_amount, coin_units[i]);
        change_amount -= coin_units[i] * change[i];
    }
}

fn print_change(coin_units: &[i32], change: &[i32], size: usize) {
    for i in 0..size {
        println!("{:8}원 : {}개", coin_units[i], change[i]);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let mut next = || tokens.next().unwrap().parse::<i32>().unwrap();

    print!("동전의 가짓수를 입력하세요 :");
    let unit_count = next() as usize;

    let mut coin_units = vec![0i32; unit_count];
    let mut change = vec![0i32; unit_count];

    for i in 0..unit_count {
        print!("[{}] 번째 동전의 단위를 입력하세요 : ", i);
        coin_units[i] = next();
    }

    // qsort with the book's comparator sorts in descending order.
    coin_units.sort_by(|a, b| b.cmp(a));

    print!("물건 가격을 입력하세요 : ");
    let price = next();

    print!("손님이 지불한 돈은 얼마입니까? : ");
    let pay = next();

    get_change(price, pay, &coin_units, &mut change, unit_count);

    print_change(&coin_units, &change, unit_count);
}
