
use crate::matrixi::MatrixI;
use crate::MatrixError;
use std::rc::Rc;
use std::ops::Add;
use std::cell::RefCell;

/// matrix instance

pub struct Matrix {
    component: Rc<RefCell<Vec<f64>>>,
    dim: usize
}


impl Matrix {
    fn new_with_source_component(dim: usize, component: &Vec<f64>) -> Matrix {
        let vec_count = dim.pow(2);
        let mut comp = Vec::with_capacity(vec_count);
        comp.clone_from_slice(&component[0..vec_count]);
        Matrix {
            component: Rc::new(RefCell::new(comp)),
            dim: dim
        }
    }

    pub fn new(dim: usize) -> Result<Rc<Self>, MatrixError> {
        if dim != 0 {
            let vec_count = dim.pow(2);
            let comp = Rc::new(RefCell::new(Vec::with_capacity(vec_count)));

            {
                let mut matop = MatrixI::bind_with_col_count(
                    comp.clone(), dim).unwrap();
                for i in 0..matop.row_count() {
                    matop.set_component(i, i, 1.0);
                }
                
            }
            let matrix = Matrix {
                component: comp,
                dim: dim
            };
            Ok(Rc::new(matrix))
        } else {
            Err(MatrixError)
        }
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        let comp = self.component.borrow();
        Matrix::new_with_source_component(self.dim, &comp)
    }
}


impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
         
        let res = self.clone();
        {
            
            let mut mat1 = MatrixI::bind_with_col_count_0(
                res.component.clone(), res.dim).unwrap();
            let mat2 = MatrixI::bind_with_col_count_0(
                other.component, other.dim).unwrap();
            mat1.add(&mat2).unwrap();
        }
        res
    }
}


// vi: se ts=4 sw=4 et:
