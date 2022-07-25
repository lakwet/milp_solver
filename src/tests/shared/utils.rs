use super::super::super::shared::utils::is_sorted;

#[test]
fn shared_utils_is_sorted_empty_vec() {
    assert!(is_sorted(&Vec::<usize>::new()));
}

#[test]
fn shared_utils_is_sorted_with_one_item_vec() {
    assert!(is_sorted(&vec![1]));
}

#[test]
fn shared_utils_is_sorted_with_two_sorted_items_vec() {
    assert!(is_sorted(&vec![1, 2]));
}

#[test]
fn shared_utils_is_sorted_with_two_unsorted_items_vec() {
    assert!(!is_sorted(&vec![3, 2]));
}

#[test]
fn shared_utils_is_sorted_with_sorted_vec() {
    assert!(is_sorted(&vec![1, 2, 3, 3, 4, 4]));
}

#[test]
fn shared_utils_is_sorted_with_unsorted_vec_case_1() {
    assert!(!is_sorted(&vec![1, 2, 1, 3, 4, 4]));
}

#[test]
fn shared_utils_is_sorted_with_unsorted_vec_case_2() {
    assert!(!is_sorted(&vec![3, 2, 2, 3, 4, 4]));
}

#[test]
fn shared_utils_is_sorted_with_unsorted_vec_case_3() {
    assert!(!is_sorted(&vec![1, 2, 2, 3, 4, 3]));
}
