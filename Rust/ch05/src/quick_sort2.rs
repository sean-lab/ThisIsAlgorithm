// qsort 표준 라이브러리 정렬 (QuickSort2)
// Clang/05/QuickSort2 포팅. C의 qsort + 비교 함수를 sort_by + Ordering 으로 옮겼다.
use std::cmp::Ordering;

// C의 ComparePoint 와 동일한 비교 함수.
pub fn compare_point(elem1: &i32, elem2: &i32) -> Ordering {
    if *elem1 > *elem2 {
        Ordering::Greater
    } else if *elem1 < *elem2 {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_point_orders_correctly() {
        assert_eq!(compare_point(&3, &5), Ordering::Less);
        assert_eq!(compare_point(&5, &3), Ordering::Greater);
        assert_eq!(compare_point(&4, &4), Ordering::Equal);
    }

    #[test]
    fn sort_by_compare_point_sorts() {
        let mut d = [6, 4, 2, 3, 1, 5];
        d.sort_by(compare_point);
        assert_eq!(d, [1, 2, 3, 4, 5, 6]);
    }
}
