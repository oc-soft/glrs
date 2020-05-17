use crate::matrixi::MatrixI;
pub(crate) use error::MatrixError;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::ops::Add;
use std::ops::Mul;
use std::rc::Rc;

mod error;

/// matrix instance
#[derive(Debug)]
pub struct Matrix {
    /// component
    component: Rc<RefCell<Vec<f64>>>,
    /// dimension
    dim: usize,
}

/// matrix implementatoin
impl Matrix {
    /// constructor
    fn new_with_source_component_0(
        dim: usize,
        component: &Vec<f64>,
    ) -> Matrix {
        let vec_count = dim.pow(2);
        let mut comp = Vec::with_capacity(vec_count);
        for i in 0..vec_count {
            comp.push(component[i]);
        }
        Matrix {
            component: Rc::new(RefCell::new(comp)),
            dim: dim,
        }
    }
    /// constructor
    pub fn new_with_source_component(component: &Vec<f64>) -> Matrix {
        Self::new_with_source_component_0(
            (component.len() as f64).sqrt().floor() as usize,
            component,
        )
    }

    /// constructor
    pub fn new(dim: usize) -> Result<Self, MatrixError> {
        if dim != 0 {
            let vec_count = dim.pow(2);
            let comp = Rc::new(RefCell::new(Vec::with_capacity(vec_count)));

            {
                let mut matop =
                    MatrixI::bind_with_col_count(comp.clone(), dim).unwrap();
                for i in 0..matop.row_count() {
                    matop.set_component(i, i, 1.0).unwrap();
                }
            }
            let matrix = Matrix {
                component: comp,
                dim: dim,
            };
            Ok(matrix)
        } else {
            Err(MatrixError)
        }
    }

    /// get component
    pub fn get_component(
        &self,
        row_idx: usize,
        col_idx: usize,
    ) -> Result<f64, MatrixError> {
        let matop =
            MatrixI::bind_with_col_count(self.component.clone(), self.dim)
                .unwrap();
        matop.get_component(row_idx, col_idx)
    }

    /// set component
    fn set_component(
        &mut self,
        row_idx: usize,
        col_idx: usize,
        val: f64,
    ) -> Result<(), MatrixError> {
        let mut matop =
            MatrixI::bind_with_col_count(self.component.clone(), self.dim)
                .unwrap();
        matop.set_component(row_idx, col_idx, val)
    }

    /// get row count
    fn get_row_count(&self) -> usize {
        return self.dim;
    }

    /// get column count
    fn get_col_count(&self) -> usize {
        return self.dim;
    }
}

impl Matrix {
    /// create minor matrix
    pub fn minor(
        &self,
        row_idx: usize,
        col_idx: usize,
    ) -> Result<Matrix, MatrixError> {
        if self.dim > 1 && row_idx < self.dim && col_idx < self.dim {
            let mut res = Self::new(self.dim - 1).unwrap();
            let mut ridx = 0;
            for ridx_0 in 0..self.dim {
                if ridx_0 != row_idx {
                    let mut cidx = 0;
                    for cidx_0 in 0..self.dim {
                        if cidx_0 != col_idx {
                            let val =
                                self.get_component(ridx_0, cidx_0).unwrap();
                            res.set_component(ridx, cidx, val).unwrap();
                            cidx += 1;
                        }
                    }
                    ridx += 1;
                }
            }
            Ok(res)
        } else {
            Err(MatrixError)
        }
    }

    /// query all component in row are zero  
    pub fn is_zero_row(&self, row_index: usize) -> Result<bool, MatrixError> {
        if row_index < self.dim {
            let mut result = true;
            for i in 0..self.get_col_count() {
                let cmp = self.get_component(row_index, i).unwrap();
                result = cmp == 0.0 || cmp == -0.0;
                if !result {
                    break;
                }
            }
            Ok(result)
        } else {
            Err(MatrixError)
        }
    }
    /// query all component in column are zero  
    pub fn is_zero_column(
        &self,
        col_index: usize,
    ) -> Result<bool, MatrixError> {
        if col_index < self.dim {
            let mut result = true;
            for i in 0..self.get_row_count() {
                let cmp = self.get_component(i, col_index).unwrap();
                result = cmp == 0.0 || cmp == -0.0;
                if !result {
                    break;
                }
            }
            Ok(result)
        } else {
            Err(MatrixError)
        }
    }

    /// query whether this matrix has all zero row or column.
    pub fn has_zero_line(&self) -> bool {
        let mut result = false;
        for i in 0..self.get_row_count() {
            result = self.is_zero_row(i).unwrap();
            if result {
                break;
            }
        }
        if !result {
            for i in 0..self.get_col_count() {
                result = self.is_zero_column(i).unwrap();
                if result {
                    break;
                }
            }
        }
        result
    }

    /// create traspose matrix
    pub fn transpose(&self) -> Self {
        let mut res = Self::new(self.dim).unwrap();
        for idx0 in 0..self.get_row_count() {
            for idx1 in 0..self.get_col_count() {
                res.set_component(
                    idx1,
                    idx0,
                    self.get_component(idx0, idx1).unwrap(),
                )
                .unwrap();
            }
        }
        res
    }

    /// calc cofactor
    pub fn get_cofactor(
        &self,
        row_index: usize,
        col_index: usize,
    ) -> Result<f64, MatrixError> {
        if row_index < self.get_row_count() && col_index < self.get_col_count()
        {
            let minor = self.minor(row_index, col_index).unwrap();
            let flag = (-1.0 as f64).powi((row_index + col_index) as i32);
            Ok(minor.determinant() * flag)
        } else {
            Err(MatrixError)
        }
    }

    /// create cofactor matrix
    pub fn cofactor_matrix(&self) -> Self {
        if self.dim > 1 {
            let mut result = Matrix::new(self.dim).unwrap();
            for ridx in 0..self.get_row_count() {
                for cidx in 0..self.get_col_count() {
                    result
                        .set_component(
                            ridx,
                            cidx,
                            self.get_cofactor(ridx, cidx).unwrap(),
                        )
                        .unwrap();
                }
            }
            result
        } else {
            self.clone()
        }
    }

    /// calc determinant
    pub fn determinant(&self) -> f64 {
        if self.dim > 1 {
            let mut result = 0.0;
            if !self.has_zero_line() {
                for i in 0..self.get_col_count() {
                    let cmp = self.get_component(0, i).unwrap();
                    let cof = self.get_cofactor(0, i).unwrap();
                    result += cmp * cof;
                }
            }
            result
        } else {
            self.get_component(0, 0).unwrap()
        }
    }

    /// get inverse matrix
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det != 0.0 || det != -0.0 {
            let comat = self.cofactor_matrix();
            let comat_t = comat.transpose();
            let mut result = Self::new(self.dim).unwrap();
            for ridx in 0..self.get_row_count() {
                for cidx in 0..self.get_col_count() {
                    result
                        .set_component(
                            ridx,
                            cidx,
                            comat_t.get_component(ridx, cidx).unwrap() / det,
                        )
                        .unwrap();
                }
            }
            Some(result)
        } else {
            None
        }
    }
}

impl Matrix {
    /// apply this matrix to v from left size
    pub fn apply_l(&self, v: Vec<f64>) -> Result<Vec<f64>, MatrixError> {
        let matop =
            MatrixI::bind_with_col_count(self.component.clone(), self.dim)
                .unwrap();
        matop.apply_l(v)
    }

    /// apply this matrix to v from left size
    pub fn apply_r(&self, v: Vec<f64>) -> Result<Vec<f64>, MatrixError> {
        let matop =
            MatrixI::bind_with_col_count(self.component.clone(), self.dim)
                .unwrap();
        matop.apply_r(v)
    }
}

/// clone object
impl Clone for Matrix {
    fn clone(&self) -> Self {
        let comp = self.component.borrow();
        Matrix::new_with_source_component_0(self.dim, &comp)
    }
}

/// add operator
impl Add for Matrix {
    type Output = Self;

    /// it works operator +
    fn add(self, other: Self) -> Self {
        let res = self.clone();
        {
            let mut mat1 =
                MatrixI::bind_with_col_count_0(res.component.clone(), res.dim)
                    .unwrap();
            let mat2 =
                MatrixI::bind_with_col_count_0(other.component, other.dim)
                    .unwrap();
            mat1.add(&mat2).unwrap();
        }
        res
    }
}

/// multiply operator
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let res = self.clone();
        {
            let mut mat1 =
                MatrixI::bind_with_col_count_0(res.component.clone(), res.dim)
                    .unwrap();
            let mat2 = MatrixI::bind_with_col_count_0(rhs.component, rhs.dim)
                .unwrap();
            mat1.multiply(&mat2).unwrap();
        }
        res
    }
}

/// operator ==
impl PartialEq for Matrix {
    /// operator equals
    fn eq(&self, other: &Self) -> bool {
        if self.dim == other.dim {
            let mat1 =
                MatrixI::bind_with_col_count(self.component.clone(), self.dim)
                    .unwrap();
            let mat2 = MatrixI::bind_with_col_count(
                other.component.clone(),
                other.dim,
            )
            .unwrap();
            mat1 == mat2
        } else {
            false
        }
    }
}

// vi: se ts=4 sw=4 et:
