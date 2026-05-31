// 퀵 정렬 (Quick Sort)
// Clang/05/QuickSort 포팅. 분할(Partition) 로직을 그대로 옮겼다.

fn swap(data_set: &mut [i32], a: usize, b: usize) {
    data_set.swap(a, b);
}

pub fn partition(data_set: &mut [i32], left: i32, right: i32) -> i32 {
    let first = left;
    let pivot = data_set[first as usize];

    let mut left = left + 1;
    let mut right = right;

    while left <= right {
        while data_set[left as usize] <= pivot && left < right {
            left += 1;
        }

        while data_set[right as usize] >= pivot && left <= right {
            right -= 1;
        }

        if left < right {
            swap(data_set, left as usize, right as usize);
        } else {
            break;
        }
    }

    swap(data_set, first as usize, right as usize);

    right
}

pub fn quick_sort(data_set: &mut [i32], left: i32, right: i32) {
    if left < right {
        let index = partition(data_set, left, right);

        quick_sort(data_set, left, index - 1);
        quick_sort(data_set, index + 1, right);
    }
}

// 슬라이스 전체를 정렬하는 편의 함수.
pub fn sort(data_set: &mut [i32]) {
    if data_set.is_empty() {
        return;
    }
    let last = data_set.len() as i32 - 1;
    quick_sort(data_set, 0, last);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_sample() {
        let mut d = [6, 4, 2, 3, 1, 5];
        sort(&mut d);
        assert_eq!(d, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn handles_edge_cases() {
        let mut empty: [i32; 0] = [];
        sort(&mut empty);
        assert_eq!(empty, [] as [i32; 0]);

        let mut single = [42];
        sort(&mut single);
        assert_eq!(single, [42]);

        let mut rev = [5, 4, 3, 2, 1];
        sort(&mut rev);
        assert_eq!(rev, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicates_and_negatives() {
        let mut d = [3, -1, 3, 0, -1, 2];
        sort(&mut d);
        assert_eq!(d, [-1, -1, 0, 2, 3, 3]);
    }

    #[test]
    fn matches_std_sort_on_many_inputs() {
        // 결정적 의사난수로 여러 입력을 표준 정렬과 비교한다.
        let mut seed: u64 = 0x1234_5678;
        for _ in 0..200 {
            let n = (seed % 50) as usize;
            let mut v: Vec<i32> = (0..n)
                .map(|_| {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    (seed >> 33) as i32 % 100 - 50
                })
                .collect();
            let mut expected = v.clone();
            expected.sort();
            sort(&mut v);
            assert_eq!(v, expected);
        }
    }
}
