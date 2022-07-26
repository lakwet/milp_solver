use std::fmt;

use super::super::shared::utils::is_uniq_sorted;
use super::slack::SlackFormLP;

/// Linear Programming, Standard form.
///
/// Maximize Sum(c_j . x_j) for j = 1 to n
///
/// Subject to:
/// ```ignore
///     Sum(a_ij . x_j) <= b_i for i = 1 to m, j = 1 to n
///     x_ij >= 0.0
/// ```
/// Example:
/// ```ignore
///     max c_1 . x_1 + c_2 . x_2
///     with
///     a_11 . x_1 + a_12 . x_2 <= b_1
///     a_21 . x_1 + a_22 . x_2 <= b_2
///     a_31 . x_1 + a_32 . x_2 <= b_3
///     x_1, x_2 >= 0.0
///     for i = 1 to 3, j = 1 to 2
/// ```
///
/// non_negative_indices stands for "Without non negative indices":
/// [i such as x_i in Real]
/// Indices start at 0
/// For example: vec![0, 2, 5];
/// Caution: x_i = x_i' - x_i'' transformation is already done!
/// non_negative_indices are used to retrieve the solution
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct StandardFormLP {
    c: Vec<f32>,
    x: Vec<f32>,
    a: Vec<Vec<f32>>,
    b: Vec<f32>,
    non_negative_indices: Option<Vec<usize>>,
}

impl StandardFormLP {
    pub fn new(
        c: Vec<f32>,
        a: Vec<Vec<f32>>,
        b: Vec<f32>,
        non_negative_indices: Option<Vec<usize>>,
    ) -> Result<StandardFormLP, String> {
        if a.is_empty() {
            return Err("Matrix 'a' should not be empty".into());
        }
        if b.is_empty() {
            return Err("Vector 'b' should not be empty".into());
        }
        if c.is_empty() {
            return Err("Vector 'c' should not be empty".into());
        }
        if c.len() != a[0].len() {
            return Err(format!(
                "Matrix 'a' rows count does not match with vector 'c' size \
                 ({} != {})",
                a[0].len(),
                c.len(),
            ));
        }
        for line in a.iter() {
            if line.len() != c.len() {
                return Err("Each and every row in the matrix 'a' should \
                            have the same size"
                    .into());
            }
        }
        if a.len() != b.len() {
            return Err(format!(
                "Matrix 'a' columns count does not match with vector 'b' size \
                 ({} != {})",
                a.len(),
                b.len(),
            ));
        }

        let non_negative_indices = if let Some(nni) = non_negative_indices {
            let nni_len = nni.len();
            if 2 * nni_len > c.len() {
                return Err("Cannot be more non negative indices than the \
                            dimension size."
                    .into());
            }

            let nni_opt = Some(nni);
            check_non_negative_indices(&nni_opt, Some(c.len() - nni_len))?;

            nni_opt
        } else {
            None
        };

        let x = vec![0.0; c.len()]; // Solution vector initialization

        Ok(StandardFormLP { c, x, a, b, non_negative_indices })
    }

    pub fn into_slack_form(&self) -> SlackFormLP {
        unimplemented!();
    }
}

impl fmt::Display for StandardFormLP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("Maximize: ");
        let str = self
            .c
            .iter()
            .enumerate()
            .map(|(j, c)| format!("{:.2} . x_{}", c, j))
            .collect::<Vec<String>>()
            .join(" + ");
        println!("\t{}", str);
        println!("Subject to:");
        for (i, line) in self.a.iter().enumerate() {
            let str = line
                .iter()
                .enumerate()
                .map(|(j, a)| format!("{:.2} . x_{}", a, j))
                .collect::<Vec<String>>()
                .join(" + ");
            println!("\t{} <= {:.2}", str, self.b[i]);
        }
        let str = self
            .x
            .iter()
            .enumerate()
            .map(|(j, _)| format!("x_{} >= 0.0", j))
            .collect::<Vec<String>>()
            .join(", ");
        println!("With: {}", str);

        write!(f, "\n")
    }
}

pub fn check_non_negative_indices(
    non_negative_indices: &Option<Vec<usize>>,
    max_dim: Option<usize>,
) -> Result<(), String> {
    if let Some(nni) = &non_negative_indices {
        if nni.is_empty() {
            return Err("It is not allowed to add an empty array of non \
                        negative indices."
                .into());
        }
        if !is_uniq_sorted(nni) {
            return Err("Non negative indices vector must be unique and \
                        sorted in ascending order."
                .into());
        }
        if let Some(dim) = max_dim {
            if nni[nni.len() - 1] >= dim {
                return Err("Non negative indices are out of bound of the \
                            dimension size."
                    .into());
            }
        }
    }

    Ok(())
}
