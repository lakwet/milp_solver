use super::super::super::forms::standard::{
    check_non_negative_indices, StandardFormLP,
};

#[test]
fn forms_standard_check_non_negative_indices_ok() {
    let result = check_non_negative_indices(&None, None);
    assert!(result.is_ok());

    let result = check_non_negative_indices(&None, Some(2));
    assert!(result.is_ok());

    let result = check_non_negative_indices(&Some(vec![1, 2, 3, 4]), None);
    assert!(result.is_ok());

    let result = check_non_negative_indices(&Some(vec![1, 2, 3, 4]), Some(5));
    assert!(result.is_ok());
}

#[test]
fn forms_standard_check_non_negative_indices_empty_vec() {
    let err = check_non_negative_indices(&Some(Vec::new()), None).unwrap_err();

    assert_eq!(
        err,
        "It is not allowed to add an empty array of non negative indices."
            .to_string()
    );
}

#[test]
fn forms_standard_check_non_negative_indices_not_uniq() {
    let err =
        check_non_negative_indices(&Some(vec![0, 1, 1]), None).unwrap_err();

    assert_eq!(
        err,
        "Non negative indices vector must be unique and sorted in ascending \
         order."
            .to_string()
    );
}

#[test]
fn forms_standard_check_non_negative_indices_not_sorted() {
    let err =
        check_non_negative_indices(&Some(vec![0, 2, 1]), None).unwrap_err();

    assert_eq!(
        err,
        "Non negative indices vector must be unique and sorted in ascending \
         order."
            .to_string()
    );
}

#[test]
fn forms_standard_check_non_negative_indices_bad_dim_case_1() {
    let err =
        check_non_negative_indices(&Some(vec![0, 2]), Some(2)).unwrap_err();

    assert_eq!(
        err,
        "Non negative indices are out of bound of the dimension size."
            .to_string()
    );
}

#[test]
fn forms_standard_check_non_negative_indices_bad_dim_case_2() {
    let err =
        check_non_negative_indices(&Some(vec![0, 2]), Some(1)).unwrap_err();

    assert_eq!(
        err,
        "Non negative indices are out of bound of the dimension size."
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_empty_vector_c() {
    let c = vec![];
    let b = vec![6.2, 8.12];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let err = StandardFormLP::new(c, a, b, None).unwrap_err();

    assert_eq!(err, "Vector 'c' should not be empty".to_string());
}

#[test]
fn forms_standard_standardformlp_empty_vector_b() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let err = StandardFormLP::new(c, a, b, None).unwrap_err();

    assert_eq!(err, "Vector 'b' should not be empty".to_string());
}

#[test]
fn forms_standard_standardformlp_empty_matrix_a() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12];
    let a = vec![];

    let err = StandardFormLP::new(c, a, b, None).unwrap_err();

    assert_eq!(err, "Matrix 'a' should not be empty".to_string());
}

#[test]
fn forms_standard_standardformlp_mismatch_matrix_a() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4]];

    let err = StandardFormLP::new(c, a, b, None).unwrap_err();

    assert_eq!(
        err,
        "Each and every row in the matrix 'a' should have the same size"
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_mismatch_matrix_a_vector_b() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12, 18.0];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let err = StandardFormLP::new(c, a, b, None).unwrap_err();

    assert_eq!(
        err,
        "Matrix 'a' columns count does not match with vector 'b' size (2 != 3)"
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_mismatch_matrix_a_vector_c() {
    let c = vec![10.2, 13.3, 14.18, 10.2];
    let b = vec![6.2, 8.12];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let err = StandardFormLP::new(c, a, b, None).unwrap_err();

    assert_eq!(
        err,
        "Matrix 'a' rows count does not match with vector 'c' size (3 != 4)"
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_empty_non_negative_indices() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12, 1.2];
    let a = vec![
        vec![1.1, 2.289719871981, 3.3],
        vec![4.4, 5.5, 6.6188918901],
        vec![4.2, 8.5, 7.6188918901],
    ];

    let err = StandardFormLP::new(c, a, b, Some(Vec::new())).unwrap_err();

    assert_eq!(
        err,
        "It is not allowed to add an empty array of non negative indices."
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_unsorted_non_negative_indices() {
    let c = vec![10.2, 13.3, 14.18, 1., 2.];
    let b = vec![6.2, 8.12, 1.2];
    let a = vec![
        vec![1.1, 2.289719871981, 3.3, 1., 2.],
        vec![4.4, 5.5, 6.6188918901, 1., 2.],
        vec![4.2, 8.5, 7.6188918901, 1., 2.],
    ];

    let err = StandardFormLP::new(c, a, b, Some(vec![2, 1])).unwrap_err();

    assert_eq!(
        err,
        "Non negative indices vector must be unique and sorted in ascending \
         order."
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_non_uniq_non_negative_indices() {
    let c = vec![10.2, 13.3, 14.18, 1., 2., 3., 4., 8., 9.];
    let b = vec![6.2, 8.12, 1.2];
    let a = vec![
        vec![1.1, 2.289719871981, 3.3, 1., 2., 3., 4., 8., 9.],
        vec![4.4, 5.5, 6.6188918901, 1., 2., 3., 4., 8., 9.],
        vec![4.2, 8.5, 7.6188918901, 1., 2., 3., 4., 8., 9.],
    ];

    let err = StandardFormLP::new(c, a, b, Some(vec![0, 1, 2, 2])).unwrap_err();

    assert_eq!(
        err,
        "Non negative indices vector must be unique and sorted in ascending \
         order."
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_more_indices_non_negative_indices() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12, 1.2];
    let a = vec![
        vec![1.1, 2.289719871981, 3.3],
        vec![4.4, 5.5, 6.6188918901],
        vec![4.2, 8.5, 7.6188918901],
    ];

    let err = StandardFormLP::new(c, a, b, Some(vec![0, 1, 2, 2])).unwrap_err();

    assert_eq!(
        err,
        "Cannot be more non negative indices than the dimension size."
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_out_of_bound_non_negative_indices() {
    let c = vec![10.2, 13.3, 14.18, 1., 2., 3.];
    let b = vec![6.2, 8.12, 1.2];
    let a = vec![
        vec![1.1, 2.289719871981, 3.3, 1., 2., 3.],
        vec![4.4, 5.5, 6.6188918901, 1., 2., 3.],
        vec![4.2, 8.5, 7.6188918901, 1., 2., 3.],
    ];

    let err = StandardFormLP::new(c, a, b, Some(vec![0, 1, 3])).unwrap_err();

    assert_eq!(
        err,
        "Non negative indices are out of bound of the dimension size."
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_should_be_created() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let standard_form = StandardFormLP::new(c, a, b, None);

    assert!(standard_form.is_ok());
}

#[test]
fn forms_standard_standardformlp_with_non_negative_indices_should_be_created() {
    let c = vec![10.2, 13.3, 14.18, 1., 2.];
    let b = vec![6.2, 8.12];
    let a = vec![
        vec![1.1, 2.289719871981, 3.3, 1., 2.],
        vec![4.4, 5.5, 6.6188918901, 1., 2.],
    ];
    let nni = vec![0, 2];

    let standard_form = StandardFormLP::new(c, a, b, Some(nni));

    assert!(standard_form.is_ok());
}

#[ignore]
#[test]
fn forms_standard_standardformlp_just_print() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    println!("{}", standard_form);
}
