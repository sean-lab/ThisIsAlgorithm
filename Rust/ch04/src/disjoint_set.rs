// 분리 집합 (Disjoint Set)
// Clang/04/DisjointSet 포팅. 노드의 포인터 동일성(==)을 사용하므로 raw pointer로 옮겼다.
use std::ptr;

pub struct DisjointSet {
    pub parent: *mut DisjointSet,
    // C 원본의 void* Data 를 옮긴 필드.
    pub data: i32,
}

pub unsafe fn ds_make_set(new_data: i32) -> *mut DisjointSet {
    Box::into_raw(Box::new(DisjointSet {
        parent: ptr::null_mut(),
        data: new_data,
    }))
}

pub unsafe fn ds_find_set(mut set: *mut DisjointSet) -> *mut DisjointSet {
    while !(*set).parent.is_null() {
        set = (*set).parent;
    }
    set
}

pub unsafe fn ds_union_set(set1: *mut DisjointSet, set2: *mut DisjointSet) {
    let root2 = ds_find_set(set2);
    (*root2).parent = set1;
}

pub unsafe fn ds_destroy_set(set: *mut DisjointSet) {
    drop(Box::from_raw(set));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separate_sets_are_disjoint() {
        unsafe {
            let s1 = ds_make_set(1);
            let s2 = ds_make_set(2);
            assert!(ds_find_set(s1) != ds_find_set(s2));
            ds_destroy_set(s1);
            ds_destroy_set(s2);
        }
    }

    #[test]
    fn union_merges_sets() {
        unsafe {
            let s1 = ds_make_set(1);
            let s2 = ds_make_set(2);
            let s3 = ds_make_set(3);
            ds_union_set(s1, s3);
            assert!(ds_find_set(s1) == ds_find_set(s3));
            // s2 는 여전히 분리되어 있다.
            assert!(ds_find_set(s1) != ds_find_set(s2));
            ds_destroy_set(s1);
            ds_destroy_set(s2);
            ds_destroy_set(s3);
        }
    }

    #[test]
    fn union_is_transitive() {
        unsafe {
            let s1 = ds_make_set(1);
            let s3 = ds_make_set(3);
            let s4 = ds_make_set(4);
            ds_union_set(s1, s3);
            ds_union_set(s3, s4);
            // 1-3-4 가 모두 같은 집합으로 합쳐진다.
            assert!(ds_find_set(s3) == ds_find_set(s4));
            assert!(ds_find_set(s1) == ds_find_set(s4));
            ds_destroy_set(s1);
            ds_destroy_set(s3);
            ds_destroy_set(s4);
        }
    }
}
