use super::super::super::forms::builder::StandardFormBuilder;
use super::super::super::forms::standard::StandardFormLP;

#[test]
fn forms_builder_standardformbuilder_new_empty() {
    let builder = StandardFormBuilder::new();

    assert_eq!(builder.get_c(), &None);
    assert_eq!(builder.get_a(), &None);
    assert_eq!(builder.get_b(), &Vec::new());
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &None);
}

#[test]
fn forms_builder_standardformbuilder_new() {
    let builder = StandardFormBuilder::new()
        .add_min_objective(vec![1., 2., 3.])
        .unwrap()
        .add_non_negative_indices(vec![1])
        .unwrap()
        .add_equality_constraint(vec![1., 3.3, 4.4], 10.01)
        .unwrap()
        .add_equality_constraint(vec![2., 5.3, 2.4], 12.3)
        .unwrap();

    assert_eq!(builder.get_c(), &Some(vec![-1., -2., -3.]));
    assert_eq!(
        builder.get_a(),
        &Some(vec![
            vec![1., 3.3, 4.4],
            vec![-1., -3.3, -4.4],
            vec![2., 5.3, 2.4],
            vec![-2., -5.3, -2.4],
        ])
    );
    assert_eq!(builder.get_b(), &vec![10.01, -10.01, 12.3, -12.3]);
    assert_eq!(builder.get_non_negative_indices(), &Some(vec![1]));
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective_empty_vec() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_min_objective(Vec::new()).unwrap_err();

    assert_eq!(err, "It is not possible to add an empty vector.".to_string());
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective_already_added() {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_min_objective(vec![10.0])
        .unwrap()
        .add_min_objective(vec![10.0])
        .unwrap_err();

    assert_eq!(err, "Objective function is already added.".to_string());
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective_zero() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_min_objective(vec![0.0]).unwrap_err();

    assert_eq!(
        err,
        "It is not possible to add objective with only zero values."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective_zeroes() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_min_objective(vec![0.0, 0.0, 0.0]).unwrap_err();

    assert_eq!(
        err,
        "It is not possible to add objective with only zero values."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective_bad_dimension_smaller() {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_equality_constraint(vec![1., 2., 3.], 0.6)
        .unwrap()
        .add_min_objective(vec![1.2, 2.3])
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective_bad_dimension_bigger() {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_equality_constraint(vec![1., 2., 3.], 0.6)
        .unwrap()
        .add_min_objective(vec![1.2, 2.3, 3., 10.])
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective_bad_non_negative_indices(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_non_negative_indices(vec![50])
        .unwrap()
        .add_min_objective(vec![1.2, 2.3, 3.])
        .unwrap_err();

    assert_eq!(
        err,
        "Non negative indices are out of bound of the added vector."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective() {
    let builder = StandardFormBuilder::new();
    let result = builder.add_min_objective(vec![1.2, 2.3, 3., 10.]);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![-1.2, -2.3, -3., -10.]));
    assert_eq!(builder.get_a(), &None);
    assert_eq!(builder.get_b(), &Vec::new());
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(4));
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective_with_non_negative_indices(
) {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_non_negative_indices(vec![1])
        .unwrap()
        .add_min_objective(vec![1.2, 2.3, 3., 10.]);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![-1.2, -2.3, -3., -10.]));
    assert_eq!(builder.get_a(), &None);
    assert_eq!(builder.get_b(), &Vec::new());
    assert_eq!(builder.get_non_negative_indices(), &Some(vec![1]));
    assert_eq!(builder.get_dimension_size(), &Some(4));
}

#[test]
fn forms_builder_standardformbuilder_add_min_objective_with_constraint() {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_equality_constraint(vec![1., 2., 3., 4.], 10.)
        .unwrap()
        .add_min_objective(vec![1.2, 2.3, 3., 10.]);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![-1.2, -2.3, -3., -10.]));
    assert_eq!(
        builder.get_a(),
        &Some(vec![vec![1., 2., 3., 4.], vec![-1., -2., -3., -4.],])
    );
    assert_eq!(builder.get_b(), &vec![10., -10.]);
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(4));
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective_empty_vec() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_max_objective(Vec::new()).unwrap_err();

    assert_eq!(err, "It is not possible to add an empty vector.".to_string());
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective_already_added() {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_max_objective(vec![10.0])
        .unwrap()
        .add_max_objective(vec![10.0])
        .unwrap_err();

    assert_eq!(err, "Objective function is already added.".to_string());
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective_zero() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_max_objective(vec![0.0]).unwrap_err();

    assert_eq!(
        err,
        "It is not possible to add objective with only zero values."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective_zeroes() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_max_objective(vec![0.0, 0.0, 0.0]).unwrap_err();

    assert_eq!(
        err,
        "It is not possible to add objective with only zero values."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective_bad_dimension_smaller() {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_equality_constraint(vec![1., 2., 3.], 0.6)
        .unwrap()
        .add_max_objective(vec![1.2, 2.3])
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective_bad_dimension_bigger() {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_equality_constraint(vec![1., 2., 3.], 0.6)
        .unwrap()
        .add_max_objective(vec![1.2, 2.3, 3., 10.])
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective_bad_non_negative_indices(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_non_negative_indices(vec![50])
        .unwrap()
        .add_max_objective(vec![1.2, 2.3, 3.])
        .unwrap_err();

    assert_eq!(
        err,
        "Non negative indices are out of bound of the added vector."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective() {
    let builder = StandardFormBuilder::new();
    let result = builder.add_max_objective(vec![1.2, 2.3, 3., 10.]);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![1.2, 2.3, 3., 10.]));
    assert_eq!(builder.get_a(), &None);
    assert_eq!(builder.get_b(), &Vec::new());
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(4));
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective_with_non_negative_indices(
) {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_non_negative_indices(vec![1])
        .unwrap()
        .add_max_objective(vec![1.2, 2.3, 3., 10.]);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![1.2, 2.3, 3., 10.]));
    assert_eq!(builder.get_a(), &None);
    assert_eq!(builder.get_b(), &Vec::new());
    assert_eq!(builder.get_non_negative_indices(), &Some(vec![1]));
    assert_eq!(builder.get_dimension_size(), &Some(4));
}

#[test]
fn forms_builder_standardformbuilder_add_max_objective_with_constraint() {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_equality_constraint(vec![1., 2., 3., 4.], 10.)
        .unwrap()
        .add_max_objective(vec![1.2, 2.3, 3., 10.]);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![1.2, 2.3, 3., 10.]));
    assert_eq!(
        builder.get_a(),
        &Some(vec![vec![1., 2., 3., 4.], vec![-1., -2., -3., -4.],])
    );
    assert_eq!(builder.get_b(), &vec![10., -10.]);
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(4));
}

#[test]
fn forms_builder_standardformbuilder_add_non_negative_indices_empty() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_non_negative_indices(Vec::new()).unwrap_err();

    assert_eq!(
        err,
        "It is not allowed to add an empty array of non negative indices."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_non_negative_indices_not_uniq() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_non_negative_indices(vec![0, 0]).unwrap_err();

    assert_eq!(
        err,
        "Non negative indices vector must be unique and sorted in ascending \
         order."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_non_negative_indices_not_sorted() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_non_negative_indices(vec![2, 1]).unwrap_err();

    assert_eq!(
        err,
        "Non negative indices vector must be unique and sorted in ascending \
         order."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_non_negative_indices_bad_dimension() {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_max_objective(vec![1.2, 2.3, 3., 10.])
        .unwrap()
        .add_non_negative_indices(vec![1, 10])
        .unwrap_err();

    assert_eq!(
        err,
        "Non negative indices are out of bound of the dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_non_negative_indices_already_set() {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_non_negative_indices(vec![1])
        .unwrap()
        .add_non_negative_indices(vec![2])
        .unwrap_err();

    assert_eq!(err, "Non negative indices have already been set.".to_string());
}

#[test]
fn forms_builder_standardformbuilder_add_non_negative_indices() {
    let builder = StandardFormBuilder::new();
    let result = builder.add_non_negative_indices(vec![1]);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &None);
    assert_eq!(builder.get_a(), &None);
    assert_eq!(builder.get_b(), &Vec::new());
    assert_eq!(builder.get_non_negative_indices(), &Some(vec![1]));
    assert_eq!(builder.get_dimension_size(), &None);
}

#[test]
fn forms_builder_standardformbuilder_add_non_negative_indices_with_dimension() {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_max_objective(vec![1.2, 2.3, 3., 10.])
        .unwrap()
        .add_non_negative_indices(vec![1, 3]);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![1.2, 2.3, 3., 10.]));
    assert_eq!(builder.get_a(), &None);
    assert_eq!(builder.get_b(), &Vec::new());
    assert_eq!(builder.get_non_negative_indices(), &Some(vec![1, 3]));
    assert_eq!(builder.get_dimension_size(), &Some(4));
}

#[test]
fn forms_builder_standardformbuilder_add_equality_constraint_empty() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_equality_constraint(Vec::new(), 10.0).unwrap_err();

    assert_eq!(err, "It is not possible to add an empty vector.".to_string());
}

#[test]
fn forms_builder_standardformbuilder_add_equality_constraint_mismatch_dim_case_1(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_equality_constraint(vec![1., 2., 3.], 10.0)
        .unwrap()
        .add_equality_constraint(vec![1., 2.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_equality_constraint_mismatch_dim_case_2(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_max_objective(vec![1., 2., 3.])
        .unwrap()
        .add_equality_constraint(vec![1., 2.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_equality_constraint_bad_non_negative_indices(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_non_negative_indices(vec![10])
        .unwrap()
        .add_equality_constraint(vec![1., 2.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "Non negative indices are out of bound of the added vector."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_equality_constraint_zeroes() {
    let builder = StandardFormBuilder::new();
    let err = builder.add_equality_constraint(vec![0., 0.], 12.0).unwrap_err();

    assert_eq!(
        err,
        "It is not possible to add constraint with only zero values."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_equality_constraint_once() {
    let builder = StandardFormBuilder::new();
    let result = builder.add_equality_constraint(vec![1., 2., 3.], 12.0);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &None);
    assert_eq!(
        builder.get_a(),
        &Some(vec![vec![1., 2., 3.], vec![-1., -2., -3.],])
    );
    assert_eq!(builder.get_b(), &vec![12.0, -12.0]);
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_add_equality_constraint_twice() {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_equality_constraint(vec![1., 2., 3.], 12.0)
        .unwrap()
        .add_equality_constraint(vec![4., 5., 6.], 7.0);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &None);
    assert_eq!(
        builder.get_a(),
        &Some(vec![
            vec![1., 2., 3.],
            vec![-1., -2., -3.],
            vec![4., 5., 6.],
            vec![-4., -5., -6.],
        ])
    );
    assert_eq!(builder.get_b(), &vec![12.0, -12.0, 7., -7.]);
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_add_equality_constraint_twice_with_all() {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_max_objective(vec![0.1, 0.2, 0.3])
        .unwrap()
        .add_equality_constraint(vec![1., 2., 3.], 12.0)
        .unwrap()
        .add_non_negative_indices(vec![1])
        .unwrap()
        .add_equality_constraint(vec![4., 5., 6.], 7.0);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![0.1, 0.2, 0.3]));
    assert_eq!(
        builder.get_a(),
        &Some(vec![
            vec![1., 2., 3.],
            vec![-1., -2., -3.],
            vec![4., 5., 6.],
            vec![-4., -5., -6.],
        ])
    );
    assert_eq!(builder.get_b(), &vec![12.0, -12.0, 7., -7.]);
    assert_eq!(builder.get_non_negative_indices(), &Some(vec![1]));
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_add_less_than_or_equal_constraint_empty() {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_less_than_or_equal_constraint(Vec::new(), 10.0)
        .unwrap_err();

    assert_eq!(err, "It is not possible to add an empty vector.".to_string());
}

#[test]
fn forms_builder_standardformbuilder_add_less_than_or_equal_constraint_mismatch_dim_case_1(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_less_than_or_equal_constraint(vec![1., 2., 3.], 10.0)
        .unwrap()
        .add_less_than_or_equal_constraint(vec![1., 2.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_less_than_or_equal_constraint_mismatch_dim_case_2(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_max_objective(vec![1., 2., 3.])
        .unwrap()
        .add_less_than_or_equal_constraint(vec![1., 2.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_less_than_or_equal_constraint_bad_non_negative_indices(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_non_negative_indices(vec![10])
        .unwrap()
        .add_less_than_or_equal_constraint(vec![1., 2.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "Non negative indices are out of bound of the added vector."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_less_than_or_equal_constraint_zeroes()
{
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_less_than_or_equal_constraint(vec![0., 0.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "It is not possible to add constraint with only zero values."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_less_than_or_equal_constraint_once() {
    let builder = StandardFormBuilder::new();
    let result =
        builder.add_less_than_or_equal_constraint(vec![1., 2., 3.], 12.0);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &None);
    assert_eq!(builder.get_a(), &Some(vec![vec![1., 2., 3.],]));
    assert_eq!(builder.get_b(), &vec![12.0]);
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_add_less_than_or_equal_constraint_twice() {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_less_than_or_equal_constraint(vec![1., 2., 3.], 12.0)
        .unwrap()
        .add_less_than_or_equal_constraint(vec![4., 5., 6.], 7.0);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &None);
    assert_eq!(
        builder.get_a(),
        &Some(vec![vec![1., 2., 3.], vec![4., 5., 6.],])
    );
    assert_eq!(builder.get_b(), &vec![12.0, 7.]);
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_add_less_than_or_equal_constraint_twice_with_all(
) {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_max_objective(vec![0.1, 0.2, 0.3])
        .unwrap()
        .add_less_than_or_equal_constraint(vec![1., 2., 3.], 12.0)
        .unwrap()
        .add_non_negative_indices(vec![1])
        .unwrap()
        .add_less_than_or_equal_constraint(vec![4., 5., 6.], 7.0);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![0.1, 0.2, 0.3]));
    assert_eq!(
        builder.get_a(),
        &Some(vec![vec![1., 2., 3.], vec![4., 5., 6.],])
    );
    assert_eq!(builder.get_b(), &vec![12.0, 7.]);
    assert_eq!(builder.get_non_negative_indices(), &Some(vec![1]));
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_add_greater_than_or_equal_constraint_empty(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_greater_than_or_equal_constraint(Vec::new(), 10.0)
        .unwrap_err();

    assert_eq!(err, "It is not possible to add an empty vector.".to_string());
}

#[test]
fn forms_builder_standardformbuilder_add_greater_than_or_equal_constraint_mismatch_dim_case_1(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_greater_than_or_equal_constraint(vec![1., 2., 3.], 10.0)
        .unwrap()
        .add_greater_than_or_equal_constraint(vec![1., 2.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_greater_than_or_equal_constraint_mismatch_dim_case_2(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_max_objective(vec![1., 2., 3.])
        .unwrap()
        .add_greater_than_or_equal_constraint(vec![1., 2.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "The added vector does not match the current dimension size."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_greater_than_or_equal_constraint_bad_non_negative_indices(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_non_negative_indices(vec![10])
        .unwrap()
        .add_greater_than_or_equal_constraint(vec![1., 2.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "Non negative indices are out of bound of the added vector."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_greater_than_or_equal_constraint_zeroes(
) {
    let builder = StandardFormBuilder::new();
    let err = builder
        .add_greater_than_or_equal_constraint(vec![0., 0.], 12.0)
        .unwrap_err();

    assert_eq!(
        err,
        "It is not possible to add constraint with only zero values."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_add_greater_than_or_equal_constraint_once()
{
    let builder = StandardFormBuilder::new();
    let result =
        builder.add_greater_than_or_equal_constraint(vec![1., 2., 3.], 12.0);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &None);
    assert_eq!(builder.get_a(), &Some(vec![vec![-1., -2., -3.],]));
    assert_eq!(builder.get_b(), &vec![-12.0]);
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_add_greater_than_or_equal_constraint_twice(
) {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_greater_than_or_equal_constraint(vec![1., 2., 3.], 12.0)
        .unwrap()
        .add_greater_than_or_equal_constraint(vec![4., 5., 6.], 7.0);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &None);
    assert_eq!(
        builder.get_a(),
        &Some(vec![vec![-1., -2., -3.], vec![-4., -5., -6.],])
    );
    assert_eq!(builder.get_b(), &vec![-12.0, -7.]);
    assert_eq!(builder.get_non_negative_indices(), &None);
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_add_greater_than_or_equal_constraint_twice_with_all(
) {
    let builder = StandardFormBuilder::new();
    let result = builder
        .add_max_objective(vec![0.1, 0.2, 0.3])
        .unwrap()
        .add_greater_than_or_equal_constraint(vec![1., 2., 3.], 12.0)
        .unwrap()
        .add_non_negative_indices(vec![1])
        .unwrap()
        .add_greater_than_or_equal_constraint(vec![4., 5., 6.], 7.0);

    assert!(result.is_ok());

    let builder = result.unwrap();
    assert_eq!(builder.get_c(), &Some(vec![0.1, 0.2, 0.3]));
    assert_eq!(
        builder.get_a(),
        &Some(vec![vec![-1., -2., -3.], vec![-4., -5., -6.],])
    );
    assert_eq!(builder.get_b(), &vec![-12.0, -7.]);
    assert_eq!(builder.get_non_negative_indices(), &Some(vec![1]));
    assert_eq!(builder.get_dimension_size(), &Some(3));
}

#[test]
fn forms_builder_standardformbuilder_build_missing_objective() {
    let err = StandardFormBuilder::new().build().unwrap_err();

    assert_eq!(
        err,
        "Cannot build standard form for LP problem, missing objective \
         function."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_build_missing_constraints() {
    let err = StandardFormBuilder::new()
        .add_max_objective(vec![1., 2., 3., 4., 5.])
        .unwrap()
        .build()
        .unwrap_err();

    assert_eq!(
        err,
        "Cannot build standard form for LP problem, missing constraints."
            .to_string()
    );
}

#[test]
fn forms_builder_standardformbuilder_build_equality_constraint_with_max_obj() {
    let c = vec![1., 2., 3., 4., 5.];
    let row = vec![2.2, 3.3, 4.4, 19.2, 10.08];
    let minus_row = vec![-2.2, -3.3, -4.4, -19.2, -10.08];
    let b = 2.2;
    let standard_form = StandardFormBuilder::new()
        .add_max_objective(c.clone())
        .unwrap()
        .add_equality_constraint(row.clone(), b)
        .unwrap()
        .build()
        .unwrap();

    let expected =
        StandardFormLP::new(c, vec![row, minus_row], vec![b, -b], None)
            .unwrap();

    assert_eq!(standard_form, expected);
}

#[test]
fn forms_builder_standardformbuilder_build_lt_eq_constraint_with_min_obj() {
    let c = vec![1., 2., 3., 4., 5.];
    let minus_c = vec![-1., -2., -3., -4., -5.];
    let row = vec![2.2, 3.3, 4.4, 19.2, 10.08];
    let b = 2.2;
    let standard_form = StandardFormBuilder::new()
        .add_min_objective(c.clone())
        .unwrap()
        .add_less_than_or_equal_constraint(row.clone(), b)
        .unwrap()
        .build()
        .unwrap();

    let expected =
        StandardFormLP::new(minus_c, vec![row], vec![b], None).unwrap();

    assert_eq!(standard_form, expected);
}

#[test]
fn forms_builder_standardformbuilder_build_gt_eq_constraint_with_min_obj() {
    let c = vec![1., 2., 3., 4., 5.];
    let minus_c = vec![-1., -2., -3., -4., -5.];
    let row = vec![2.2, 3.3, 4.4, 19.2, 10.08];
    let minus_row = vec![-2.2, -3.3, -4.4, -19.2, -10.08];
    let b = 2.2;
    let standard_form = StandardFormBuilder::new()
        .add_min_objective(c.clone())
        .unwrap()
        .add_greater_than_or_equal_constraint(row, b)
        .unwrap()
        .build()
        .unwrap();

    let expected =
        StandardFormLP::new(minus_c, vec![minus_row], vec![-b], None).unwrap();

    assert_eq!(standard_form, expected);
}

#[test]
fn forms_builder_standardformbuilder_build_equality_constraint_with_nni() {
    let c = vec![1., 2., 3., 4., 5.];
    let row = vec![2.2, 3.3, 4.4, 19.2, 10.08];
    let b = 2.2;
    let expect_minus_c = vec![-1., -2., -3., -4., -5., -3.];
    let expect_row = vec![2.2, 3.3, 4.4, 19.2, 10.08, 4.4];
    let expect_minus_row = vec![-2.2, -3.3, -4.4, -19.2, -10.08, -4.4];
    let standard_form = StandardFormBuilder::new()
        .add_non_negative_indices(vec![2])
        .unwrap()
        .add_min_objective(c.clone())
        .unwrap()
        .add_equality_constraint(row, b)
        .unwrap()
        .build()
        .unwrap();

    let expected = StandardFormLP::new(
        expect_minus_c,
        vec![expect_row, expect_minus_row],
        vec![b, -b],
        Some(vec![2]),
    )
    .unwrap();

    assert_eq!(standard_form, expected);
}
