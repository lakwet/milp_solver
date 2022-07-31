use std::cmp::PartialOrd;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum InitializationResult {
    Done,
    Unfeasible,
}

#[derive(Debug, PartialEq)]
pub enum Leaving {
    Unbounded,
    Info(f32, usize, usize), // delta, row index, basic index
}

#[derive(Debug, PartialEq)]
pub enum SimplexRound {
    Finished,
    Unbounded,
    Switch(usize, usize), // Column, Row
}

#[derive(Debug, PartialEq)]
pub enum SimplexResult {
    Unbounded,
    Unfeasible,
    Optimal(Vec<f32>),
}

/// Linear Programming, Slack form.
/// ```ignore
///     s = b_i - Sum(a_ij . x_j) for j = 1 to n
///     s >= 0
/// ```
/// We use x_n+i instead of s_i, so we have:
/// ```ignore
///     x_n+i = b_i - Sum(a_ij . x_j) for j = 1 to n
///     x_n+i >= 0
/// ```
/// With
/// ```ignore
///     z = v + Sum(c_j . x_j) for j = 1 to n
/// ```
/// Example:
/// ```ignore
///     max z = v + c_1 . x_1 + c_2 . x_2
///     with
///     x_3 = b_1 - a_11 . x_1 - a_12 . x_2
///     x_4 = b_2 - a_21 . x_1 - a_22 . x_2
///     x_5 = b_3 - a_31 . x_1 - a_32 . x_2
///     x_1, x_2, x_3, x_4, x_5 >= 0.0
///     for i = 1 to 3, j = 1 to 2
/// ```
/// Notation: (N, B, A, b, c, v)
/// ```ignore
///     N is the set of indices of x in the right part of the equality
///     B is the set of indices of x in the left part of the equality
/// ```
/// Example:
/// ```ignore
///     z = 28 - x_3 / 6 - x_5 / 6 - 2 . x_6 / 3
///     x_1 = 8 + x_3 / 6 + x_5 / 6 - x_6 / 3
///     x_2 = 4 - 8 . x_3 / 3 - 2 . x_5 / 3 + x_6 / 3
///     x_4 = 18 - x_3 / 2 + x_5 / 2
///     =>  N = { 3, 5, 6 }
///         B = { 1, 2, 4 }
///         A = Matrix ( a_13, a_15, a_16
///                      a_23, a_25, a_26
///                      a_43, a_45, a_46 )
///         b = ( 8
///               4
///               18 )
///         c = ( -1/6, -1/6 + 1/3)
///         v = 28
/// ```
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SlackFormLP {
    N: Vec<usize>, // Non Basic variables
    B: Vec<usize>, // Basic variables
    A: Vec<Vec<f32>>,
    b: Vec<f32>,
    c: Vec<f32>,
    v: f32,
    n: usize, // dimension problem (number of cols)
}

impl SlackFormLP {
    pub fn new(
        A: Vec<Vec<f32>>,
        b: Vec<f32>,
        c: Vec<f32>,
    ) -> Result<SlackFormLP, String> {
        // The first instance must comply to these following constraints:
        if A.is_empty() {
            return Err("Matrix 'A' should not be empty".into());
        }
        if b.is_empty() {
            return Err("Vector 'b' should not be empty".into());
        }
        if c.is_empty() {
            return Err("Vector 'c' should not be empty".into());
        }
        if c.len() != A[0].len() {
            return Err(format!(
                "Matrix 'A' rows count does not match with vector 'c' size \
                 ({} != {})",
                A[0].len(),
                c.len(),
            ));
        }
        for line in A.iter() {
            if line.len() != c.len() {
                return Err("Each and every row in the matrix 'A' should \
                            have the same size"
                    .into());
            }
        }
        if A.len() != b.len() {
            return Err(format!(
                "Matrix 'A' columns count does not match with vector 'b' size \
                 ({} != {})",
                A.len(),
                b.len(),
            ));
        }

        let N: Vec<usize> = (0..c.len()).into_iter().collect();
        let B: Vec<usize> = (c.len()..c.len() + b.len()).into_iter().collect();
        let v = 0.;
        let n = c.len();

        Ok(SlackFormLP { N, B, A, b, c, v, n })
    }

    pub fn get_dim(&self) -> usize { self.n }

    pub fn initialize_simplex(
        &mut self,
    ) -> Result<InitializationResult, String> {
        // Find the smallest b_j and its index
        let mut smallest_b = self.b[0];
        let mut smallest_row = 0;
        for (j, b) in self.b.iter().enumerate().skip(1) {
            if *b < smallest_b {
                smallest_b = *b;
                smallest_row = j;
            }
        }

        // If all b_j >= 0. there is a basic feasible solution, nothing to do
        // then
        if smallest_b >= 0. {
            return Ok(InitializationResult::Done);
        }

        // Otherwise, we have to find a non basic feasible solution
        // Thus, we add a new variable, increasing dimension by 1

        // Let's save the current objective
        let saved_c = self.c.clone();

        // And update c accordingly with the algorithm to find a non basic
        // solution
        self.c = vec![0.; self.n];
        self.c.push(-1.); // Add sup x variable

        // Update Basic and Non Basic indices
        self.N.push(self.n); // Add sup x variable
        for elem in self.B.iter_mut() {
            *elem += 1;
        }

        // Increase dimension by 1
        self.n += 1;

        // Update A
        for row in self.A.iter_mut() {
            row.push(1.); // Add sup x variable
        }

        // First pivot to make the auxilliary lp problem feasible
        self.pivot(self.n - 1, smallest_row);

        let result = match self.find_optimal()? {
            SimplexResult::Optimal(solution) if solution[self.n - 1] == 0. => {
                InitializationResult::Done
            },
            _ => InitializationResult::Unfeasible,
        };

        if result == InitializationResult::Unfeasible {
            return Ok(InitializationResult::Unfeasible);
        }

        let sup_x_index_row =
            self.B.iter().enumerate().find(|(_, elem)| **elem == self.n - 1);

        if let Some((row_index, _)) = sup_x_index_row {
            // Perform one more pivot in order to put the sup x in non basic
            // where a_row_index_i != 0.0
            let col_degenerate_opt =
                self.A[row_index].iter().enumerate().find(|(_, a)| **a != 0.);
            if let Some((i, _)) = col_degenerate_opt {
                self.pivot(i, row_index);
            } else {
                return Err("All values at degenerated row equal zero, this \
                            should not happen."
                    .into());
            }
        }

        // Otherwise, restore initial lp problem with updates for the non basic
        // feasible solution

        let sup_x_index_col =
            self.N.iter().enumerate().find(|(_, elem)| **elem == self.n - 1);

        let x_col = if let Some((col, _)) = sup_x_index_col {
            col
        } else {
            return Err("Sup x variable has not been found in non basic \
                        variable, this should not happen"
                .into());
        };

        // Remove sup var in N
        self.N.swap_remove(x_col);

        // Remove colum at index 'col'
        for row in self.A.iter_mut() {
            row.swap_remove(x_col);
        }

        // Remove element in objective at index 'col'
        self.c.swap_remove(x_col);

        // Update dimension
        self.n -= 1;

        // Update B and N
        for elem in self.N.iter_mut() {
            if *elem >= self.n {
                *elem -= 1;
            }
        }
        for elem in self.B.iter_mut() {
            if *elem >= self.n {
                *elem -= 1;
            }
        }

        // Recompute the objective function
        self.recompute_objective_for_initialization(saved_c);

        Ok(InitializationResult::Done)
    }

    fn recompute_objective_for_initialization(&mut self, init_c: Vec<f32>) {
        let mut objective = vec![0.; self.n];

        for (row, basic) in self.B.iter().enumerate() {
            if *basic < self.n {
                let coef = init_c[*basic];
                self.v += self.b[row] * coef;
                for (j, elem) in self.A[row].iter().enumerate() {
                    objective[j] += *elem * coef;
                }
            }
        }

        for (col, non_basic) in self.N.iter().enumerate() {
            if *non_basic < self.n {
                objective[col] += init_c[*non_basic];
            }
        }

        self.c = objective;
    }

    fn find_leaving(&self, col: usize) -> Result<Leaving, String> {
        if col >= self.A[0].len() {
            return Err(
                "Out of bound error while finding leaving variable.".into()
            );
        }
        let deltas =
            self.b.iter().enumerate().fold(Vec::new(), |mut acc, (row, b)| {
                if *b >= 0. && self.A[row][col] < 0. {
                    let delta = b / -self.A[row][col];
                    acc.push((delta, row));
                    acc
                } else {
                    acc
                }
            });

        if deltas.is_empty() {
            return Ok(Leaving::Unbounded);
        }

        // If there are more than one minimal value, the first one is picked,
        // this can be changed according to the desired rule.
        let min_delta = deltas
            .into_iter()
            .min_by(|(d1, _), (d2, _)| d1.partial_cmp(&d2).unwrap());

        if let Some((delta, row)) = min_delta {
            Ok(Leaving::Info(delta, row, self.B[row]))
        } else {
            Err("No delta result left, it should not happen.".into())
        }
    }

    pub fn find_entering_and_leaving(&self) -> Result<SimplexRound, String> {
        let mut cols = Vec::new();

        for (col, c) in self.c.iter().enumerate() {
            if *c > 0. {
                let leaving = self.find_leaving(col)?;

                if let Leaving::Info(delta, row, basic) = leaving {
                    // We could compute the objective gain in order to choose a
                    // pivoting rule. So far, we use the Band's rule, so we
                    // don't need it.
                    cols.push((delta, col, self.N[col], row, basic));
                } else {
                    return Ok(SimplexRound::Unbounded);
                }
            }
        }

        if cols.is_empty() {
            return Ok(SimplexRound::Finished);
        }

        // We choose the smallest non basic index according to Band's rule
        let min_non_basic_index = cols
            .into_iter()
            .min_by(|col1, col2| col1.2.partial_cmp(&col2.2).unwrap());

        if let Some((_, col, _, row, _)) = min_non_basic_index {
            Ok(SimplexRound::Switch(col, row))
        } else {
            Err("No column result left, it should not happen.".into())
        }
    }

    pub fn pivot(&mut self, col: usize, row: usize) {
        // Switch basic and non_basic
        let tmp = self.N[col];
        self.N[col] = self.B[row];
        self.B[row] = tmp;

        // Create new row to replace row at index 'row'
        let minus_a_rc = -self.A[row][col]; // which is != 0.0
        self.b[row] = self.b[row] / minus_a_rc;
        let inv_row: Vec<f32> =
            self.A[row]
                .iter()
                .enumerate()
                .map(|(i, elem)| {
                    if i != col {
                        *elem / minus_a_rc
                    } else {
                        -1. / minus_a_rc
                    }
                })
                .collect();

        // Update rows except row at index 'row'
        for (j, one_row) in self.A.iter_mut().enumerate() {
            if j != row {
                let a_rc = one_row[col];
                self.b[j] = self.b[j] + self.b[row] * a_rc;
                for (i, elem) in one_row.iter_mut().enumerate() {
                    if i != col {
                        *elem = *elem + inv_row[i] * a_rc;
                    } else {
                        *elem = inv_row[i] * a_rc;
                    }
                }
            }
        }

        // Update objective function
        let c_c = self.c[col];
        self.v = self.v + self.b[row] * c_c;
        for (i, elem) in self.c.iter_mut().enumerate() {
            if i != col {
                *elem = *elem + inv_row[i] * c_c;
            } else {
                *elem = inv_row[i] * c_c;
            }
        }

        // Update row at index 'row'
        self.A[row] = inv_row;
    }

    pub fn find_optimal(&mut self) -> Result<SimplexResult, String> {
        let mut round_count = 0;

        loop {
            round_count += 1;

            match self.find_entering_and_leaving() {
                Ok(SimplexRound::Unbounded) => {
                    return Ok(SimplexResult::Unbounded);
                },
                Ok(SimplexRound::Finished) => {
                    break;
                },
                Ok(SimplexRound::Switch(col, row)) => {
                    self.pivot(col, row);
                },
                Err(msg) => {
                    return Err(msg);
                },
            }
        }

        println!("Round count: {}", round_count);

        println!("end: {}", self);

        Ok(self.compute_solution_vector())
    }

    pub fn compute_solution_vector(&self) -> SimplexResult {
        // Each non basic variables are set to 0
        // Each basic variables 'j' are set with the value of the respective b_j
        let mut x = vec![0.; self.n];
        for (j, basic) in self.B.iter().enumerate() {
            if *basic < self.n {
                x[*basic] = self.b[j];
            }
        }

        SimplexResult::Optimal(x)
    }
}

impl fmt::Display for SlackFormLP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("LP - Slack form:");
        println!("N: {:?}", self.N);
        println!("B: {:?}", self.B);
        println!("c: {:?}", self.c);
        println!("A:");
        for row in self.A.iter() {
            println!("\t{:?}", row);
        }
        println!("b: {:?}", self.b);
        println!("v: {:?}", self.v);
        println!("n: {}", self.n);
        println!("Solution vector: {:?}", self.compute_solution_vector());

        write!(f, "\n")
    }
}

#[test]
fn non_public_forms_slack_slackformlp_find_leaving() {
    let lp_slack = SlackFormLP {
        N: vec![2, 1, 0, 3, 4],
        B: vec![6, 7, 5],
        A: vec![
            vec![-0.1, 0.2, 0.5, 0.4, -1.5],
            vec![-1., -2., -0.5, 4., -2.5],
            vec![-10., 20., -0.1, 40., -2.],
        ],
        b: vec![1.5, 2.5, -3.5],
        c: vec![3., -2., 5., -6.],
        v: 0.,
        n: 5,
    };

    assert_eq!(lp_slack.find_leaving(0), Ok(Leaving::Info(2.5, 1, 7)));
    assert_eq!(lp_slack.find_leaving(1), Ok(Leaving::Info(1.25, 1, 7)));
    assert_eq!(lp_slack.find_leaving(2), Ok(Leaving::Info(5., 1, 7)));
    assert_eq!(lp_slack.find_leaving(3), Ok(Leaving::Unbounded));
    assert_eq!(lp_slack.find_leaving(4), Ok(Leaving::Info(1.0, 0, 6)));
    assert_eq!(
        lp_slack.find_leaving(5),
        Err("Out of bound error while finding leaving variable.".to_string())
    );
}

#[test]
fn non_public_forms_slack_slackformlp_find_entering_and_leaving() {
    let lp_slack = SlackFormLP {
        N: vec![2, 1, 0, 3, 4],
        B: vec![6, 7, 5],
        A: vec![
            vec![-0.1, 0.2, 0.5, 0.4, -1.5],
            vec![-1., -2., -0.5, 4., -2.5],
            vec![-10., 20., -0.1, 40., -2.],
        ],
        b: vec![1.5, 2.5, -3.5],
        c: vec![3., -2., 5., -6.],
        v: 0.,
        n: 5,
    };

    let simplex_round = lp_slack.find_entering_and_leaving().unwrap();

    assert_eq!(simplex_round, SimplexRound::Switch(2, 1));
}

#[test]
fn non_public_forms_slack_slackformlp_pivot_case_1() {
    let mut lp_slack = SlackFormLP {
        N: vec![0, 1, 2],
        B: vec![3, 4, 5],
        A: vec![vec![-1., -1., -3.], vec![-2., -2., -5.], vec![-4., -1., -2.]],
        b: vec![30., 24., 36.],
        c: vec![3., 1., 2.],
        v: 0.,
        n: 3,
    };

    lp_slack.pivot(0, 2);

    let expected = SlackFormLP {
        N: vec![5, 1, 2],
        B: vec![3, 4, 0],
        A: vec![
            vec![0.25, -0.75, -2.5],
            vec![0.5, -1.5, -4.],
            vec![-0.25, -0.25, -0.5],
        ],
        b: vec![21., 6., 9.],
        c: vec![-0.75, 0.25, 0.5],
        v: 27.,
        n: 3,
    };

    assert_eq!(lp_slack, expected);

    // This should undo the previous "pivoting"
    lp_slack.pivot(0, 2);

    let expected2 = SlackFormLP {
        N: vec![0, 1, 2],
        B: vec![3, 4, 5],
        A: vec![vec![-1., -1., -3.], vec![-2., -2., -5.], vec![-4., -1., -2.]],
        b: vec![30., 24., 36.],
        c: vec![3., 1., 2.],
        v: 0.,
        n: 3,
    };

    assert_eq!(lp_slack, expected2);
}

#[test]
fn non_public_forms_slack_slackformlp_pivot_case_2() {
    let mut lp_slack = SlackFormLP {
        N: vec![5, 1, 2],
        B: vec![3, 4, 0],
        A: vec![
            vec![0.25, -0.75, -2.5],
            vec![0.5, -1.5, -4.],
            vec![-0.25, -0.25, -0.5],
        ],
        b: vec![21., 6., 9.],
        c: vec![-0.75, 0.25, 0.5],
        v: 27.,
        n: 3,
    };

    lp_slack.pivot(2, 1);

    let expected = SlackFormLP {
        N: vec![5, 1, 4],
        B: vec![3, 2, 0],
        A: vec![
            vec![-0.0625, 0.1875, 0.625],
            vec![0.125, -0.375, -0.25],
            vec![-0.3125, -0.0625, 0.125],
        ],
        b: vec![17.25, 1.5, 8.25],
        c: vec![-0.6875, 0.0625, -0.125],
        v: 27.75,
        n: 3,
    };

    assert_eq!(lp_slack, expected);
}

#[test]
fn non_public_forms_slack_slackformlp_pivot_case_3() {
    let mut lp_slack = SlackFormLP {
        N: vec![5, 1, 4],
        B: vec![3, 2, 0],
        A: vec![
            vec![-0.0625, 0.1875, 0.625],
            vec![0.125, -0.375, -0.25],
            vec![-0.3125, -0.0625, 0.125],
        ],
        b: vec![17.25, 1.5, 8.25],
        c: vec![-0.6875, 0.0625, -0.125],
        v: 27.75,
        n: 3,
    };

    lp_slack.pivot(1, 1);

    let expected = SlackFormLP {
        N: vec![5, 2, 4],
        B: vec![3, 1, 0],
        A: vec![
            vec![0.0, -0.5, 0.5],
            vec![0.33333334, -2.6666667, -0.6666667],
            vec![-0.33333334, 0.16666667, 0.16666667],
        ],
        b: vec![18.0, 4.0, 8.0],
        c: vec![-0.6666667, -0.16666667, -0.16666667],
        v: 28.0,
        n: 3,
    };

    assert_eq!(lp_slack, expected);
}

#[test]
fn non_public_forms_slack_slackformlp_initialize_simplex_case_1() {
    let mut lp_slack = SlackFormLP {
        N: vec![0, 1],
        B: vec![2, 3],
        A: vec![vec![-2., 1.], vec![-1., 5.]],
        b: vec![2., -4.],
        c: vec![2., -1.],
        v: 0.,
        n: 2,
    };

    let result = lp_slack.initialize_simplex().unwrap();
    assert_eq!(result, InitializationResult::Done);

    let expected = SlackFormLP {
        N: vec![0, 3],
        B: vec![2, 1],
        A: vec![vec![-1.8, 0.19999999], vec![0.2, 0.2]],
        b: vec![2.8, 0.8],
        c: vec![1.8, -0.2],
        v: -0.8,
        n: 2,
    };

    assert_eq!(lp_slack, expected);
}
