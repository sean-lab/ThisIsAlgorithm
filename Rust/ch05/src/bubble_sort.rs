// 버블 정렬 (Bubble Sort)
// Clang/05/BubbleSort 포팅.

pub fn bubble_sort(data_set: &mut [i32]) {
    let length = data_set.len();
    if length == 0 {
        return;
    }
    for i in 0..length - 1 {
        for j in 0..length - (i + 1) {
            if data_set[j] > data_set[j + 1] {
                data_set.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_sample() {
        let mut d = [6, 4, 2, 3, 1, 5];
        bubble_sort(&mut d);
        assert_eq!(d, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn handles_edge_cases() {
        let mut empty: [i32; 0] = [];
        bubble_sort(&mut empty);
        assert_eq!(empty, [] as [i32; 0]);

        let mut single = [42];
        bubble_sort(&mut single);
        assert_eq!(single, [42]);

        let mut sorted = [1, 2, 3, 4];
        bubble_sort(&mut sorted);
        assert_eq!(sorted, [1, 2, 3, 4]);

        let mut rev = [5, 4, 3, 2, 1];
        bubble_sort(&mut rev);
        assert_eq!(rev, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicates_and_negatives() {
        let mut d = [3, -1, 3, 0, -1, 2];
        bubble_sort(&mut d);
        assert_eq!(d, [-1, -1, 0, 2, 3, 3]);
    }
}
