// 거스름돈 만들기 (Making Change) - 로직은 ch14::making_change 모듈에 있다.
use ch14::making_change::{get_change, print_change};
use std::io::{self, Read};

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
