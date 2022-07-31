use super::super::shared::utils::all_zeroes;
use super::standard::{check_non_negative_indices, StandardFormLP};

/// Builder for the standard form.
#[derive(Debug, PartialEq)]
pub struct StandardFormBuilder {
    c: Option<Vec<f64>>,
    a: Option<Vec<Vec<f64>>>,
    b: Vec<f64>,
    non_negative_indices: Option<Vec<usize>>,
    dimension_size: Option<usize>,
}

impl StandardFormBuilder {
    pub fn new() -> StandardFormBuilder {
        StandardFormBuilder {
            c: None,
            a: None,
            b: Vec::new(),
            non_negative_indices: None,
            dimension_size: None,
        }
    }

    pub fn get_c(&self) -> &Option<Vec<f64>> { &self.c }

    pub fn get_a(&self) -> &Option<Vec<Vec<f64>>> { &self.a }

    pub fn get_b(&self) -> &Vec<f64> { &self.b }

    pub fn get_non_negative_indices(&self) -> &Option<Vec<usize>> {
        &self.non_negative_indices
    }

    pub fn get_dimension_size(&self) -> &Option<usize> { &self.dimension_size }

    fn check_dimension_size(&mut self, current: usize) -> Result<(), String> {
        if current == 0 {
            return Err("It is not possible to add an empty vector.".into());
        }
        if let Some(nni) = &self.non_negative_indices {
            if nni[nni.len() - 1] >= current {
                return Err("Non negative indices are out of bound of the \
                            added vector."
                    .into());
            }
        }
        if let Some(size) = self.dimension_size {
            if size == current {
                Ok(())
            } else {
                Err("The added vector does not match the current dimension \
                     size."
                    .into())
            }
        } else {
            self.dimension_size = Some(current);
            Ok(())
        }
    }

    fn check_objective_added(&self) -> Result<(), String> {
        if self.c.is_some() {
            return Err("Objective function is already added.".into());
        }

        Ok(())
    }

    fn check_objective_not_zeroes(
        &self,
        objective: &Vec<f64>,
    ) -> Result<(), String> {
        if all_zeroes(objective) {
            return Err("It is not possible to add objective with only zero \
                        values."
                .into());
        }

        Ok(())
    }

    fn check_constraint_not_zeroes(&self, a: &Vec<f64>) -> Result<(), String> {
        if all_zeroes(a) {
            return Err("It is not possible to add constraint with only zero \
                        values."
                .into());
        }

        Ok(())
    }

    /// Add min objective function:
    ///
    /// min c_1 . x_1 + ... + c_n . x_n
    pub fn add_min_objective(mut self, c: Vec<f64>) -> Result<Self, String> {
        self.check_objective_added()?;
        self.check_dimension_size(c.len())?;
        self.check_objective_not_zeroes(&c)?;

        let minus_c = c.into_iter().map(|v| -v).collect();
        self.c = Some(minus_c);

        Ok(self)
    }

    /// Add max objective function
    ///
    /// max c_1 . x_1 + ... + c_n . x_n
    pub fn add_max_objective(mut self, c: Vec<f64>) -> Result<Self, String> {
        self.check_objective_added()?;
        self.check_dimension_size(c.len())?;
        self.check_objective_not_zeroes(&c)?;

        self.c = Some(c);

        Ok(self)
    }

    /// Add non negative indices (sorted in ascending order)
    ///
    /// [i such as x_i in Real]
    /// Indices start at 0
    /// For example: vec![0, 2, 5];
    pub fn add_non_negative_indices(
        mut self,
        nni: Vec<usize>,
    ) -> Result<Self, String> {
        let nni_opt = Some(nni);
        check_non_negative_indices(&nni_opt, self.dimension_size)?;

        if self.non_negative_indices.is_none() {
            self.non_negative_indices = nni_opt;
            Ok(self)
        } else {
            Err("Non negative indices have already been set.".into())
        }
    }

    /// Add equality constraint
    ///
    /// a_1 . x_1 + ... + a_n . x_n = b
    pub fn add_equality_constraint(
        mut self,
        a: Vec<f64>,
        b: f64,
    ) -> Result<Self, String> {
        self.check_dimension_size(a.len())?;
        self.check_constraint_not_zeroes(&a)?;

        let minus_a = a.iter().map(|v| -v).collect();

        if let Some(mat) = self.a {
            self.a = Some(vec![mat, vec![a, minus_a]].concat());
        } else {
            self.a = Some(vec![a, minus_a]);
        }

        self.b.append(&mut vec![b, -b]);

        Ok(self)
    }

    /// Add less than or equal constraint
    ///
    /// a_1 . x_1 + ... + a_n . x_n <= b
    pub fn add_less_than_or_equal_constraint(
        mut self,
        a: Vec<f64>,
        b: f64,
    ) -> Result<Self, String> {
        self.check_dimension_size(a.len())?;
        self.check_constraint_not_zeroes(&a)?;

        if let Some(mat) = &mut self.a {
            mat.push(a);
        } else {
            self.a = Some(vec![a]);
        }

        self.b.push(b);

        Ok(self)
    }

    /// Add greater than or equal constraint
    ///
    /// a_1 . x_1 + ... + a_n . x_n >= b
    pub fn add_greater_than_or_equal_constraint(
        mut self,
        a: Vec<f64>,
        b: f64,
    ) -> Result<Self, String> {
        self.check_dimension_size(a.len())?;
        self.check_constraint_not_zeroes(&a)?;

        let minus_a = a.into_iter().map(|v| -v).collect();

        if let Some(mat) = &mut self.a {
            mat.push(minus_a);
        } else {
            self.a = Some(vec![minus_a]);
        }

        self.b.push(-b);

        Ok(self)
    }

    /// Build the current linear program into the standard form.
    pub fn build(self) -> Result<StandardFormLP, String> {
        let mut c = if let Some(c) = self.c {
            c
        } else {
            return Err("Cannot build standard form for LP problem, missing \
                        objective function."
                .into());
        };

        let mut a = if let Some(a) = self.a {
            a
        } else {
            return Err("Cannot build standard form for LP problem, missing \
                        constraints."
                .into());
        };
        if a.is_empty() {
            return Err("The impossible happened, matrix 'a' is empty.".into());
        }
        if a[0].is_empty() {
            return Err(
                "The impossible happened, row is matrix 'a' is empty.".into()
            );
        }

        for row in a.iter() {
            if row.len() != c.len() {
                return Err("The impossible happened, there is a mismatch \
                            between row in matrix 'a' and vector 'c'."
                    .into());
            }
        }

        let b = self.b;
        if b.is_empty() {
            return Err("The impossible happened, vector 'b' is empty.".into());
        }

        if a.len() != b.len() {
            return Err("The impossible happened, there is a mismatch \
                        between the matrix 'a' and the vector 'b'."
                .into());
        }

        if let Some(nni) = &self.non_negative_indices {
            for i in nni.iter() {
                c.push(c[*i]);

                for row in a.iter_mut() {
                    row.push(row[*i]);
                }
            }
        }

        Ok(StandardFormLP::new(c, a, b, self.non_negative_indices)?)
    }
}
