use std::fmt;

/// Linear Programming, Standard form.
///
/// Maximize Sum(c_j . x_j) for j = 1 to n
///
/// Subject to:
///
///     Sum(a_ij . x_j) <= b_i for i = 1 to m, j = 1 to n
///     x_ij >= 0.0
///
/// Example:
///
///     max c_1 . x_1 + c_2 . x_2
///     with
///     a_11 . x_1 + a_12 . x_2 <= b_1
///     a_21 . x_1 + a_22 . x_2 <= b_2
///     a_31 . x_1 + a_32 . x_2 <= b_3
///     x_1, x_2 >= 0.0
///     for i = 1 to 3, j = 1 to 2
///
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct StandardFormLP {
    c: Vec<f32>,
    x: Vec<f32>,
    a: Vec<Vec<f32>>,
    b: Vec<f32>,
}

impl StandardFormLP {
    pub fn new(c: Vec<f32>, a: Vec<Vec<f32>>, b: Vec<f32>) -> Result<StandardFormLP, String> {
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
                "Matrix 'a' columns count does not match with vector 'c' size ({} != {})",
                a[0].len(), c.len(),
            ));
        }
        for line in a.iter() {
            if line.len() != c.len() {
                return Err("Each and every row in the matrix 'a' should have the same size".into());
            }
        }
        if a.len() != b.len() {
            return Err(format!(
                "Matrix 'a' rows count does not match with vector 'b' size ({} != {})",
                a.len(), b.len(),
            ));
        }

        let x = vec![0.0; c.len()];

        Ok(StandardFormLP {
            c,
            x,
            a,
            b,
        })
    }
}

impl fmt::Display for StandardFormLP {
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
