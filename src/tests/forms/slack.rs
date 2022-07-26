use super::super::super::forms::slack::SlackFormLP;

#[test]
fn forms_slack_slackformlp_empty_matrix_A() {
    let A = Vec::new();
    let b = vec![3.8];
    let c = vec![1.2];
    let v = 17.3;

    let err = SlackFormLP::new(A, b, c, v).unwrap_err();

    assert_eq!(err, "Matrix 'A' should not be empty".to_string());
}

#[test]
fn forms_slack_slackformlp_empty_vector_b() {
    let A = vec![vec![0.1]];
    let b = Vec::new();
    let c = vec![1.2];
    let v = 17.3;

    let err = SlackFormLP::new(A, b, c, v).unwrap_err();

    assert_eq!(err, "Vector 'b' should not be empty".to_string());
}

#[test]
fn forms_slack_slackformlp_empty_vector_c() {
    let A = vec![vec![0.1]];
    let b = vec![1.2];
    let c = Vec::new();
    let v = 17.3;

    let err = SlackFormLP::new(A, b, c, v).unwrap_err();

    assert_eq!(err, "Vector 'c' should not be empty".to_string());
}

#[test]
fn forms_slack_slackformlp_mismatch_matrix_A_vector_c() {
    let A = vec![Vec::new()];
    let b = vec![1.2];
    let c = vec![12.9, 10.1];
    let v = 17.3;

    let err = SlackFormLP::new(A, b, c, v).unwrap_err();

    assert_eq!(
        err,
        "Matrix 'A' rows count does not match with vector 'c' size (0 != 2)"
            .to_string()
    );
}

#[test]
fn forms_slack_slackformlp_mismatch_matrix_A() {
    let A = vec![vec![12.39, 10.1], vec![1.1]];
    let b = vec![1.2];
    let c = vec![12.9, 10.1];
    let v = 17.3;

    let err = SlackFormLP::new(A, b, c, v).unwrap_err();

    assert_eq!(
        err,
        "Each and every row in the matrix 'A' should have the same size"
            .to_string()
    );
}

#[test]
fn forms_slack_slackformlp_mismatch_matrix_A_vector_b() {
    let A = vec![vec![12.39, 10.1], vec![1.1, 0.9]];
    let b = vec![1.2];
    let c = vec![12.9, 10.1];
    let v = 17.3;

    let err = SlackFormLP::new(A, b, c, v).unwrap_err();

    assert_eq!(
        err,
        "Matrix 'A' columns count does not match with vector 'b' size (2 != 1)"
            .to_string()
    );
}

#[test]
fn forms_slack_slackformlp_should_be_created() {
    let A = vec![vec![12.39, 10.1], vec![1.1, 0.9]];
    let b = vec![1.2, 8.8];
    let c = vec![12.9, 10.1];
    let v = 17.3;

    let slack_form = SlackFormLP::new(A, b, c, v);

    assert!(slack_form.is_ok());
}

#[ignore]
#[test]
fn forms_slack_slackformlp_just_print() {
    let A = vec![vec![12.39, 10.1], vec![1.1, 0.9]];
    let b = vec![1.2, 8.8];
    let c = vec![12.9, 10.1];
    let v = 17.3;

    let slack_form = SlackFormLP::new(A, b, c, v).unwrap();
    println!("{}", slack_form);
}
