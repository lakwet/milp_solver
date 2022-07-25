use super::super::shared::utils::is_sorted;
use super::standard::StandardFormLP;

/// Builder for the standard form.
pub struct StandardFormBuilder {
    c: Option<Vec<f32>>,
    a: Option<Vec<Vec<f32>>>,
    b: Vec<f32>,
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

    fn check_dimension_size(&mut self, current: usize) -> Result<(), String> {
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

    /// Add min objective function:
    ///
    /// min c_1 . x_1 + ... + c_n . x_n
    pub fn add_min_objective(mut self, c: Vec<f32>) -> Result<Self, String> {
        self.check_objective_added()?;
        self.check_dimension_size(c.len())?;

        let minus_c = c.into_iter().map(|v| -v).collect();
        self.c = Some(minus_c);

        Ok(self)
    }

    /// Add max objective function
    ///
    /// max c_1 . x_1 + ... + c_n . x_n
    pub fn add_max_objective(mut self, c: Vec<f32>) -> Result<Self, String> {
        self.check_objective_added()?;
        self.check_dimension_size(c.len())?;

        self.c = Some(c);

        Ok(self)
    }

    /// Add non negative indices (sorted in ascending order)
    ///
    /// [i such as (x_i in Real or x_i <= 0.0)]
    /// Indices start at 0
    /// For example: vec![0, 2, 5];
    pub fn add_non_negative_indices(
        mut self,
        nni: Vec<usize>,
    ) -> Result<Self, String> {
        if nni.is_empty() {
            return Err("It is not allowed to add an empty array of non \
                        negative indices"
                .into());
        }
        if !is_sorted(&nni) {
            return Err("Non negative indices vector must be sorted in \
                        ascending order."
                .into());
        }
        if let Some(dim) = self.dimension_size {
            if nni[nni.len() - 1] >= dim {
                return Err("Non negative indices are out of bound of the \
                            dimension size."
                    .into());
            }
        }

        if self.non_negative_indices.is_none() {
            self.non_negative_indices = Some(nni);
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
        a: Vec<f32>,
        b: f32,
    ) -> Result<Self, String> {
        self.check_dimension_size(a.len())?;

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
        a: Vec<f32>,
        b: f32,
    ) -> Result<Self, String> {
        self.check_dimension_size(a.len())?;

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
        a: Vec<f32>,
        b: f32,
    ) -> Result<Self, String> {
        self.check_dimension_size(a.len())?;

        let minus_a = a.into_iter().map(|v| -v).collect();

        if let Some(mat) = &mut self.a {
            mat.push(minus_a);
        } else {
            self.a = Some(vec![minus_a]);
        }

        self.b.push(-b);

        Ok(self)
    }

    pub fn build(self) -> Result<StandardFormLP, String> {
        let mut c = if let Some(c) = self.c {
            c
        } else {
            return Err("Cannot build standard form for LP problem, missing \
                        objective function."
                .into());
        };

        let b = self.b;
        if b.is_empty() {
            return Err("The impossible happened, vector 'b' is empty.".into());
        }

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
        if a.len() != b.len() {
            return Err("The impossible happened, there is a mismatch \
                        between the matrix 'a' and the vector 'b'."
                .into());
        }

        for row in a.iter() {
            if row.len() != c.len() {
                return Err("The impossible happened, there is a mismatch \
                            between row in matrix 'a' and vector 'c'."
                    .into());
            }
        }

        if let Some(nni_vec) = &self.non_negative_indices {
            for (nni, i) in nni_vec.iter().enumerate() {
                c.insert(nni + i, c[nni + i]);

                for row in a.iter_mut() {
                    row.insert(nni + i, row[nni + i]);
                }
            }
        }

        Ok(StandardFormLP::new(c, a, b, self.non_negative_indices)?)
    }
}
