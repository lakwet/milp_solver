use super::super::super::forms::standard::StandardFormLP;

#[test]
fn forms_standard_standardformlp_empty_vector_c() {
    let c = vec![];
    let b = vec![6.2, 8.12];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let err = StandardFormLP::new(c, a, b).unwrap_err();

    assert_eq!(err, "Vector 'c' should not be empty".to_string());
}

#[test]
fn forms_standard_standardformlp_empty_vector_b() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let err = StandardFormLP::new(c, a, b).unwrap_err();

    assert_eq!(err, "Vector 'b' should not be empty".to_string());
}

#[test]
fn forms_standard_standardformlp_empty_matrix_a() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12];
    let a = vec![];

    let err = StandardFormLP::new(c, a, b).unwrap_err();

    assert_eq!(err, "Matrix 'a' should not be empty".to_string());
}

#[test]
fn forms_standard_standardformlp_mismatch_matrix_a() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4]];

    let err = StandardFormLP::new(c, a, b).unwrap_err();

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

    let err = StandardFormLP::new(c, a, b).unwrap_err();

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

    let err = StandardFormLP::new(c, a, b).unwrap_err();

    assert_eq!(
        err,
        "Matrix 'a' rows count does not match with vector 'c' size (3 != 4)"
            .to_string()
    );
}

#[test]
fn forms_standard_standardformlp_should_be_created() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let standard_form = StandardFormLP::new(c, a, b);

    assert!(standard_form.is_ok());
}

#[test]
fn forms_standard_standardformlp_just_print() {
    let c = vec![10.2, 13.3, 14.18];
    let b = vec![6.2, 8.12];
    let a = vec![vec![1.1, 2.289719871981, 3.3], vec![4.4, 5.5, 6.6188918901]];

    let standard_form = StandardFormLP::new(c, a, b).unwrap();
    println!("{}", standard_form);
}
