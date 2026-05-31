// 거스름돈 만들기 (Making Change, 그리디)
// Clang/14/MakingChange 포팅.

pub fn count_coins(amount: i32, coin_unit: i32) -> i32 {
    let mut coin_count = 0;
    let mut current_amount = amount;

    while current_amount >= coin_unit {
        coin_count += 1;
        current_amount -= coin_unit;
    }

    coin_count
}

pub fn get_change(price: i32, pay: i32, coin_units: &[i32], change: &mut [i32], size: usize) {
    let mut change_amount = pay - price;

    for i in 0..size {
        change[i] = count_coins(change_amount, coin_units[i]);
        change_amount -= coin_units[i] * change[i];
    }
}

pub fn print_change(coin_units: &[i32], change: &[i32], size: usize) {
    for i in 0..size {
        println!("{:8}원 : {}개", coin_units[i], change[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_coins_divides() {
        assert_eq!(count_coins(2600, 1000), 2);
        assert_eq!(count_coins(600, 500), 1);
        assert_eq!(count_coins(100, 500), 0);
    }

    #[test]
    fn get_change_breaks_down_greedily() {
        // 단위는 내림차순으로 주어진다(프로그램이 내림차순 정렬).
        let units = [1000, 500, 100, 50, 10];
        let mut change = [0i32; 5];
        // 가격 7400, 지불 10000 -> 거스름돈 2600
        get_change(7400, 10000, &units, &mut change, 5);
        assert_eq!(change, [2, 1, 1, 0, 0]);
    }

    #[test]
    fn exact_payment_yields_no_change() {
        let units = [1000, 500, 100];
        let mut change = [0i32; 3];
        get_change(1500, 1500, &units, &mut change, 3);
        assert_eq!(change, [0, 0, 0]);
    }
}
