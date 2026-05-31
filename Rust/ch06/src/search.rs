// 이진 탐색 (Binary Search)
// Clang/06/BinarySearch 포팅.
use crate::points_data::Point;
use std::cmp::Ordering;

pub fn compare_point(a: &Point, b: &Point) -> Ordering {
    if a.point > b.point {
        Ordering::Greater
    } else if a.point < b.point {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

pub fn binary_search(list: &[Point], target: f64) -> Option<&Point> {
    let mut left: i64 = 0;
    let mut right: i64 = list.len() as i64 - 1;

    while left <= right {
        let mid = ((left + right) / 2) as usize;

        if target == list[mid].point {
            return Some(&list[mid]);
        } else if target > list[mid].point {
            left = mid as i64 + 1;
        } else {
            right = mid as i64 - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::points_data::data_set;

    #[test]
    fn finds_existing_value() {
        let mut data = data_set();
        data.sort_by(compare_point);
        let found = binary_search(&data, 671.78);
        assert!(found.is_some());
        assert_eq!(found.unwrap().point, 671.78);
    }

    #[test]
    fn returns_none_for_missing_value() {
        let mut data = data_set();
        data.sort_by(compare_point);
        assert!(binary_search(&data, -1.0).is_none());
    }

    #[test]
    fn compare_point_orders_by_point() {
        let a = Point { id: 1, point: 1.0 };
        let b = Point { id: 2, point: 2.0 };
        assert_eq!(compare_point(&a, &b), Ordering::Less);
        assert_eq!(compare_point(&b, &a), Ordering::Greater);
        assert_eq!(compare_point(&a, &a), Ordering::Equal);
    }
}
