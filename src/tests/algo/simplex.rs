use super::super::super::algo::simplex::simplex_lp_chvatal;
use super::super::super::forms::slack::SimplexResult;
use super::super::super::forms::standard::StandardFormLP;

#[test]
fn algo_simplex_simplex_lp_chvatal_feasible_case_1() {
    let c = vec![1., 1.];
    let a = vec![vec![4., -1.], vec![2., 1.], vec![-5., 2.]];
    let b = vec![8., 10., 2.];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    let mut slack_form = standard_form.into_slack_form().unwrap();

    let simplex_result = simplex_lp_chvatal(&mut slack_form).unwrap();

    assert_eq!(simplex_result, SimplexResult::Optimal(vec![2., 6.]));
}

#[test]
fn algo_simplex_simplex_lp_chvatal_feasible_case_2() {
    let c = vec![3., 1., 2.];
    let a = vec![vec![1., 1., 3.], vec![2., 2., 5.], vec![4., 1., 2.]];
    let b = vec![30., 24., 36.];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    let mut slack_form = standard_form.into_slack_form().unwrap();

    let simplex_result = simplex_lp_chvatal(&mut slack_form).unwrap();

    assert_eq!(simplex_result, SimplexResult::Optimal(vec![8., 4., 0.]));
}

#[test]
fn algo_simplex_simplex_lp_chvatal_unbounded_case_1() {
    let c = vec![1., -1.];
    let a = vec![vec![-2., 1.], vec![-1., -2.]];
    let b = vec![-1., -2.];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    let mut slack_form = standard_form.into_slack_form().unwrap();

    let simplex_result = simplex_lp_chvatal(&mut slack_form).unwrap();

    assert_eq!(simplex_result, SimplexResult::Unbounded);
}

#[test]
fn algo_simplex_simplex_lp_chvatal_unbounded_case_2() {
    let c = vec![1., 3.];
    let a = vec![vec![-1., 1.], vec![-1., -1.], vec![-1., 4.]];
    let b = vec![-1., -3., 2.];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    let mut slack_form = standard_form.into_slack_form().unwrap();

    let simplex_result = simplex_lp_chvatal(&mut slack_form).unwrap();

    assert_eq!(simplex_result, SimplexResult::Unbounded);
}

#[test]
fn algo_simplex_simplex_lp_chvatal_unfeasible_case_1() {
    let c = vec![3., -2.];
    let a = vec![vec![1., 1.], vec![-2., -2.]];
    let b = vec![-2., -10.];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    let mut slack_form = standard_form.into_slack_form().unwrap();

    let simplex_result = simplex_lp_chvatal(&mut slack_form).unwrap();

    assert_eq!(simplex_result, SimplexResult::Unfeasible);
}

#[test]
fn algo_simplex_simplex_lp_chvatal_unfeasible_case_2() {
    let c = vec![1., -2.];
    let a = vec![vec![1., 2.], vec![-2., -6.], vec![0., 1.]];
    let b = vec![4., -12., 1.];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    let mut slack_form = standard_form.into_slack_form().unwrap();

    let simplex_result = simplex_lp_chvatal(&mut slack_form).unwrap();

    assert_eq!(simplex_result, SimplexResult::Unfeasible);
}

#[test]
fn algo_simplex_simplex_lp_chvatal_non_basic_feasible_case_1() {
    let c = vec![2., -1.];
    let a = vec![vec![2., -1.], vec![1., -5.]];
    let b = vec![2., -4.];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    let mut slack_form = standard_form.into_slack_form().unwrap();

    let simplex_result = simplex_lp_chvatal(&mut slack_form).unwrap();

    assert_eq!(
        simplex_result,
        SimplexResult::Optimal(vec![1.5555555555555554, 1.1111111111111112])
    );
}

#[test]
fn algo_simplex_simplex_lp_chvatal_non_basic_feasible_case_2() {
    let c = vec![1., 3.];
    let a = vec![vec![1., -1.], vec![-1., -1.], vec![-1., 4.]];
    let b = vec![8., -3., 2.];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    let mut slack_form = standard_form.into_slack_form().unwrap();

    let simplex_result = simplex_lp_chvatal(&mut slack_form).unwrap();

    assert_eq!(
        simplex_result,
        SimplexResult::Optimal(vec![11.333333333333334, 3.3333333333333335])
    );
}
