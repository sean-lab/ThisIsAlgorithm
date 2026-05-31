// 재귀적 합계 (Recurrence Sum)
// Clang/11/RecurrenceSum 포팅.

pub fn recurrence_sum(data: &[i32]) -> i32 {
    if data.len() == 1 {
        data[0]
    } else {
        recurrence_sum(&data[1..]) + data[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element_returns_itself() {
        assert_eq!(recurrence_sum(&[42]), 42);
    }

    #[test]
    fn sums_all_elements() {
        assert_eq!(recurrence_sum(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn matches_main_program_sum_of_1_to_55() {
        let data: Vec<i32> = (1..=150).collect();
        // 본문 프로그램은 처음 55개(1..=55)의 합을 구한다: 55*56/2 = 1540.
        assert_eq!(recurrence_sum(&data[..55]), 1540);
    }
}
