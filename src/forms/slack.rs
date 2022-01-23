use std::fmt;

/// Linear Programming, Slack form.
///
///     s = b_i - Sum(a_ij . x_j) for j = 1 to n
///     s >= 0
///
/// We use x_n+i instead of s_i, so we have:
///
///     x_n+i = b_i - Sum(a_ij . x_j) for j = 1 to n
///     x_n+i >= 0
///
/// With
///
///     z = v + Sum(c_j . x_j) for j = 1 to n
///
/// Example:
///
///     max z = c_1 . x_1 + c_2 . x_2
///     with
///     x_3 = b_1 - a_11 . x_1 - a_12 . x_2
///     x_4 = b_2 - a_21 . x_1 - a_22 . x_2
///     x_5 = b_3 - a_31 . x_1 - a_32 . x_2
///     x_1, x_2, x_3, x_4, x_5 >= 0.0
///     for i = 1 to 3, j = 1 to 2
///
/// Notation: (N, B, A, b, c, v)
///
///     N is the set of indices of x in the left part of the equality
///     B is the set of indices of x in the right part of the equality
///
/// Example:
///
///     z = 28 - x_3 / 6 - x_5 / 6 - 2 . x_6 / 3
///     x_1 = 8 + x_3 / 6 + x_5 / 6 - x_6 / 3
///     x_2 = 4 - 8 . x_3 / 3 - 2 . x_5 / 3 + x_6 / 3
///     x_4 = 18 - x_3 / 2 + x_5 / 2
///     =>  N = { 1, 2, 4 }
///         B = { 3, 5, 6 }
///         A = Matrix ( a_13, a_15, a_16
///                      a_23, a_25, a_26
///                      a_43, a_45, a_46 )
///         b = ( 8
///               4
///               18 )
///         c = ( -1/6, -1/6 + 1/3)
///         v = 28
///
///
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SlackFormLP {
    N: Vec<usize>,
    B: Vec<usize>,
    A: Vec<Vec<f32>>,
    b: Vec<f32>,
    c: Vec<f32>,
    v: f32,
    x: Vec<f32>,
}

impl SlackFormLP {
    pub fn new(A: Vec<Vec<f32>>, b: Vec<f32>, c: Vec<f32>, v: f32) -> Result<SlackFormLP, String> {
        // The first instance must comply to these following constraints:
        if b.is_empty() {
            return Err("Vector 'b' should not be empty".into());
        }
        if c.is_empty() {
            return Err("Vector 'c' should not be empty".into());
        }
        if c.len() != A[0].len() {
            return Err(format!(
                "Matrix 'A' columns count does not match with vector 'c' size ({} != {})",
                A[0].len(), c.len(),
            ));
        }
        for line in A.iter() {
            if line.len() != c.len() {
                return Err("Each and every row in the matrix 'A' should have the same size".into());
            }
        }
        if A.len() != b.len() {
            return Err(format!(
                "Matrix 'a' rows count does not match with vector 'b' size ({} != {})",
                A.len(), b.len(),
            ));
        }

        let N = (c.len()..c.len() + b.len()).into_iter().collect::<Vec<usize>>();
        let B = (0..c.len()).into_iter().collect::<Vec<usize>>();

        let x = vec![0.0; c.len()];

        Ok(SlackFormLP {
            N,
            B,
            A,
            b,
            c,
            v,
            x,
        })
    }
}

impl fmt::Display for SlackFormLP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("Maximize: ");
        let str = self.c.iter().enumerate()
            .map(|(j, c)| format!("{:.2} . x_{}", c, j))
            .collect::<Vec<String>>()
            .join(" + ");
        println!("\t{}", str);
        println!("Subject to:");
        for (i, line) in self.a.iter().enumerate() {
            let str = line.iter().enumerate()
                .map(|(j, a)| format!("{:.2} . x_{}", a, j))
                .collect::<Vec<String>>()
                .join(" + ");
            println!("\t{} <= {:.2}", str, self.b[i]);
        }
        let str = self.x.iter().enumerate()
            .map(|(j, _)| format!("x_{} >= 0.0", j))
            .collect::<Vec<String>>()
            .join(", ");
        println!("With: {}", str);

        write!(f, "\n")
    }
}
