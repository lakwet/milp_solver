use super::super::super::shared::utils::{all_zeroes, is_uniq_sorted};

#[test]
fn shared_utils_all_zeroes_with_empty_vec() {
    assert!(all_zeroes(&Vec::new()));
}

#[test]
fn shared_utils_all_zeroes_with_zero_vec() {
    assert!(all_zeroes(&vec![0.0]));
}

#[test]
fn shared_utils_all_zeroes_with_zeroes_vec() {
    assert!(all_zeroes(&vec![0.0, 0.0]));
}

#[test]
fn shared_utils_all_zeroes_with_not_zero_vec() {
    assert!(!all_zeroes(&vec![0.2]));
}

#[test]
fn shared_utils_all_zeroes_with_not_zeroes_vec_case_1() {
    assert!(!all_zeroes(&vec![1.0, 0.0]));
}

#[test]
fn shared_utils_all_zeroes_with_not_zeroes_vec_case_2() {
    assert!(!all_zeroes(&vec![0.0, 0.3]));
}

#[test]
fn shared_utils_all_zeroes_with_not_zeroes_vec_case_3() {
    assert!(!all_zeroes(&vec![1.2, 0.0, 0.3]));
}

#[test]
fn shared_utils_is_uniq_sorted_empty_vec() {
    assert!(is_uniq_sorted(&Vec::<usize>::new()));
}

#[test]
fn shared_utils_is_uniq_sorted_with_one_item_vec() {
    assert!(is_uniq_sorted(&vec![1]));
}

#[test]
fn shared_utils_is_uniq_sorted_with_two_sorted_items_vec() {
    assert!(is_uniq_sorted(&vec![1, 2]));
}

#[test]
fn shared_utils_is_uniq_sorted_with_two_unsorted_items_vec() {
    assert!(!is_uniq_sorted(&vec![3, 2]));
}

#[test]
fn shared_utils_is_uniq_sorted_with_two_same_items() {
    assert!(!is_uniq_sorted(&vec![2, 2]));
}

#[test]
fn shared_utils_is_uniq_sorted_with_sorted_vec() {
    assert!(is_uniq_sorted(&vec![1, 2, 3, 4]));
}

#[test]
fn shared_utils_is_uniq_sorted_with_unsorted_vec_case_1() {
    assert!(!is_uniq_sorted(&vec![1, 2, 1, 3, 4]));
}

#[test]
fn shared_utils_is_uniq_sorted_with_unsorted_vec_case_2() {
    assert!(!is_uniq_sorted(&vec![3, 2, 4, 5, 6]));
}

#[test]
fn shared_utils_is_uniq_sorted_with_unsorted_vec_case_3() {
    assert!(!is_uniq_sorted(&vec![1, 2, 3, 5, 4]));
}

#[test]
fn shared_utils_is_uniq_sorted_non_unique() {
    assert!(!is_uniq_sorted(&vec![1, 2, 2, 4, 5]));
}
