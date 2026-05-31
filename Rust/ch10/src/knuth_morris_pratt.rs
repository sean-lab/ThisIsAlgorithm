// 커누스-모리스-프랫 (Knuth-Morris-Pratt)
// Clang/10/KnuthMorrisPratt 포팅.
pub fn preprocess(pattern: &[u8], pattern_size: i32, border: &mut [i32]) {
    let mut i: i32 = 0;
    let mut j: i32 = -1;

    border[0] = -1;

    while i < pattern_size {
        while j > -1 && pattern[i as usize] != pattern[j as usize] {
            j = border[j as usize];
        }
        i += 1;
        j += 1;
        border[i as usize] = j;
    }
}

pub fn knuth_morris_pratt(
    text: &[u8],
    text_size: i32,
    start: i32,
    pattern: &[u8],
    pattern_size: i32,
) -> i32 {
    let mut i = start;
    let mut j: i32 = 0;
    let mut position = -1;

    let mut border = vec![0i32; (pattern_size + 1) as usize];
    preprocess(pattern, pattern_size, &mut border);

    while i < text_size {
        while j >= 0 && text[i as usize] != pattern[j as usize] {
            j = border[j as usize];
        }
        i += 1;
        j += 1;
        if j == pattern_size {
            position = i - j;
            break;
        }
    }

    position
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find(text: &str, pattern: &str) -> i32 {
        knuth_morris_pratt(
            text.as_bytes(),
            text.len() as i32,
            0,
            pattern.as_bytes(),
            pattern.len() as i32,
        )
    }

    #[test]
    fn finds_pattern_position() {
        assert_eq!(find("hello world", "world"), 6);
        assert_eq!(find("abcabcabd", "abcabd"), 3);
        assert_eq!(find("aaaaa", "aaa"), 0);
    }

    #[test]
    fn returns_minus_one_when_absent() {
        assert_eq!(find("hello", "xyz"), -1);
    }

    #[test]
    fn preprocess_border_for_ababaca() {
        let pattern = b"ababaca";
        let mut border = vec![0i32; pattern.len() + 1];
        preprocess(pattern, pattern.len() as i32, &mut border);
        // 표준 KMP 실패 함수(failure) 결과.
        assert_eq!(border, vec![-1, 0, 0, 1, 2, 3, 0, 1]);
    }
}
