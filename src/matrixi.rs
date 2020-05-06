use crate::MatrixError;
use std::cell::RefCell;
use std::rc::Rc;

/// represent matrix
#[readonly::make]
pub struct MatrixI {
    component: Rc<RefCell<Vec<f64>>>,
    pub col_count: usize,
}

// Matrix implementatin
impl MatrixI {
    /// create matrix instance to bind vector
    pub(crate) fn bind_with_col_count_0(
        comp: Rc<RefCell<Vec<f64>>>,
        col_count: usize,
    ) -> Result<Self, MatrixError> {
        Ok(MatrixI {
            component: comp,
            col_count: col_count,
        })
    }

    /// create matrix instance to bind vector
    pub fn bind(comp: Rc<RefCell<Vec<f64>>>) -> Result<Self, MatrixError> {
        let len = comp.borrow().len();
        if len == 0 {
            Err(MatrixError)
        } else {
            let flen = len as f64;
            Self::bind_with_col_count_0(comp, (flen.sqrt().floor()) as usize)
        }
    }

    pub fn bind_with_col_count(
        comp: Rc<RefCell<Vec<f64>>>,
        col_count: usize,
    ) -> Result<Self, MatrixError> {
        let comp_size = col_count.pow(2);
        {
            let mut comp = comp.borrow_mut();
            while comp.len() < comp_size {
                comp.push(0.0);
            }
        }
        Self::bind_with_col_count_0(comp, col_count)
    }

    /// get count of component
    pub fn count(&self) -> usize {
        self.component.borrow().len()
    }

    /// get count of columns
    pub fn col_count(&self) -> usize {
        self.col_count
    }

    /// get count of rows
    pub fn row_count(&self) -> usize {
        self.col_count()
    }

    /// add matrix
    pub fn add(&mut self, matrix: &MatrixI) -> Result<&MatrixI, MatrixError> {
        if self.count() != matrix.count() {
            Err(MatrixError)
        } else {
            let mut comp0 = self.component.borrow_mut();
            let comp1 = matrix.component.borrow();
            for i in 0..comp0.len() {
                comp0[i] += comp1[i];
            }
            Ok(self)
        }
    }

    /// multiply
    pub fn multiply(
        &mut self,
        other: &MatrixI,
    ) -> Result<&MatrixI, MatrixError> {
        if self.count() == other.count() {
            let mut tmp_comp = Vec::with_capacity(self.count());
            for ridx in 0..self.row_count() {
                for cidx in 0..self.col_count() {
                    let mut cmp = 0.0;
                    for rcidx in 0..other.row_count() {
                        cmp += self.get_component(ridx, rcidx).unwrap()
                            * other.get_component(rcidx, cidx).unwrap();
                    }
                    tmp_comp.push(cmp);
                }
            }
            self.component.borrow_mut().clone_from_slice(&tmp_comp[0..]);
            Ok(self)
        } else {
            Err(MatrixError)
        }
    }

    /// copy field from other matrix
    pub fn copy(&mut self, other: &Self) -> Result<&Self, MatrixError> {
        if self.count() <= other.count() {
            let count = self.count();
            let mut comp0 = self.component.borrow_mut();
            let comp1 = other.component.borrow();
            comp0.clone_from_slice(&comp1[0..count]);
            Ok(self)
        } else {
            Err(MatrixError)
        }
    }

    /// get component
    pub fn get_component(
        &self,
        row: usize,
        col: usize,
    ) -> Result<f64, MatrixError> {
        let idx = self.col_count * row + col;
        if idx < self.count() {
            Ok(self.component.borrow()[idx])
        } else {
            Err(MatrixError)
        }
    }

    /// set component
    pub fn set_component(
        &mut self,
        row: usize,
        col: usize,
        val: f64,
    ) -> Option<MatrixError> {
        let idx = self.col_count * row + col;
        if idx < self.count() {
            let mut comp = self.component.borrow_mut();
            comp[idx] = val;
            None
        } else {
            Some(MatrixError)
        }
    }
}

impl std::cmp::PartialEq for MatrixI {
    /// operator equals overloading
    fn eq(&self, other: &Self) -> bool {
        let mut result = self.count() == other.count();
        if result {
            for i in 0..self.count() {
                result =
                    self.component.borrow()[i] == other.component.borrow()[i];
                if !result {
                    break;
                }
            }
        }
        result
    }
}

// vi: se ts=4 sw=4 et:
