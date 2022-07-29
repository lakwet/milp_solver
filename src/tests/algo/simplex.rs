use super::super::super::forms::slack::SimplexResult;
use super::super::super::forms::standard::StandardFormLP;
use super::super::super::algo::simplex::simplex_lp_chvatal;

#[test]
fn algo_simplex_simplex_lp_chvatal_should_succeed_case_1() {
    let c = vec![1., 1.];
    let a = vec![vec![4., -1.], vec![2., 1.], vec![-5., 2.]];
    let b = vec![8., 10., 2.];

    let standard_form = StandardFormLP::new(c, a, b, None).unwrap();
    let mut slack_form = standard_form.into_slack_form().unwrap();

    let simplex_result = simplex_lp_chvatal(&mut slack_form).unwrap();

    assert_eq!(simplex_result, SimplexResult::Optimal(vec![2., 6.]));
}

#[test]
fn algo_simplex_simplex_lp_chvatal_should_succeed_case_2() {
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

#[ignore] // Because not implemented yet!
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
