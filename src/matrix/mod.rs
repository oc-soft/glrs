use crate::matrixi::MatrixI;
pub(crate) use error::MatrixError;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::FnMut;
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
    #[allow(clippy::needless_range_loop)]
    fn new_with_source_component_row_order_0(
        dim: usize,
        component: &[f64],
    ) -> Matrix {

        let vec_count = dim.pow(2);
        let mut comp = Vec::with_capacity(vec_count);
        for i in 0..vec_count {
            comp.push(component[i]);
        }
        Matrix {
            component: Rc::new(RefCell::new(comp)),
            dim,
        }
    }
    /// constructor
    fn new_with_source_component_col_order_0(
        dim: usize,
        component: &[f64],
    ) -> Matrix {
        let vec_count = dim.pow(2);
        let mut comp = Vec::with_capacity(vec_count);
        for i in 0..vec_count {
            let src_ridx = i % dim;
            let src_cidx = i / dim;
            comp.push(component[src_ridx * dim + src_cidx]);
        }
        Matrix {
            component: Rc::new(RefCell::new(comp)),
            dim,
        }
    }

    /// constructor
    pub fn new_with_source_component_row_order(component: &[f64]) -> Matrix {
        Self::new_with_source_component_row_order_0(
            (component.len() as f64).sqrt().floor() as usize,
            component,
        )
    }
    /// constructor
    pub fn new_with_source_component_col_order(component: &[f64]) -> Matrix {

        Self::new_with_source_component_col_order_0(
            (component.len() as f64).sqrt().floor() as usize,
            component,
        )
    }


    /// constructor
    pub fn new(dim: usize) -> Result<Self, MatrixError> {
        if dim != 0 {
            let vec_count = dim.pow(2);
            let comp = Rc::new(RefCell::new(Vec::with_capacity(vec_count)));

            for ridx in 0..dim {
                for cidx in 0..dim {
                    let mut val = 0.0;
                    if ridx == cidx {
                        val = 1.0;
                    }
                    comp.borrow_mut().push(val);  
                }
            }
            let matrix = Matrix {
                component: comp,
                dim,
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
    pub fn get_row_count(&self) -> usize {
        self.dim
    }

    /// get column count
    pub fn get_col_count(&self) -> usize {
        self.dim
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

    /// scale 
    pub fn scale(&self, scale: f64) -> Matrix {
        let mut result = self.clone();
        result.scale_mut(scale);
        result
    }
    /// mutate scale 
    pub fn scale_mut(&mut self, scale: f64) ->&Self {
        for ridx in 0..self.get_row_count() {
            for cidx in 0..self.get_col_count() {
                self.set_component(
                    ridx, cidx,
                    self.get_component(ridx, cidx).unwrap()
                        * scale).unwrap();
            }
        }
        self
    }
}

impl Matrix {
    /// apply this matrix to v from left side
    #[allow(clippy::ptr_arg)]
    pub fn apply_l(&self, v: &Vec<f64>) -> Result<Vec<f64>, MatrixError> {
        let matop =
            MatrixI::bind_with_col_count(self.component.clone(), self.dim)
                .unwrap();
        matop.apply_l(v)
    }

    /// apply this matrix to v from right side
    #[allow(clippy::ptr_arg)]
    pub fn apply_r(&self, v: &Vec<f64>) -> Result<Vec<f64>, MatrixError> {
        let matop =
            MatrixI::bind_with_col_count(self.component.clone(), self.dim)
                .unwrap();
        matop.apply_r(v)
    }
}

impl Matrix {
    /// iterate column order
    pub fn iterate_col_order(&self,
        call_back: &mut dyn FnMut(usize, f64) -> bool) { 
        let size = self.get_row_count() * self.get_col_count();
        for i in 0..size {
            let ridx = i % self.get_col_count();
            let cidx = i / self.get_col_count();
            let state = call_back(i, self.get_component(ridx, cidx).unwrap());
            if !state {
                break;
            }
        }
    }
    /// iterate row order
    pub fn iterate_row_order(&self,
        call_back: &mut dyn FnMut(usize, f64) -> bool)  { 
        let size = self.get_row_count() * self.get_col_count();
        for i in 0..size {
            let ridx = i / self.get_col_count();
            let cidx = i % self.get_col_count();
            let state = call_back(i, self.get_component(ridx, cidx).unwrap());
            if !state {
                break;
            }
        }
    }


    /// multiply
    pub fn multiply_mut(&mut self, mat : &Self) -> bool {
        if self.dim == mat.dim {
            let mut mat_0 = MatrixI::bind_with_col_count_0(
                self.component.clone(), self.dim).unwrap();
            let mat_1 = MatrixI::bind_with_col_count_0(
                mat.component.clone(), mat.dim).unwrap();
            mat_0.multiply(&mat_1).unwrap();
            true
        } else {
            false
        }
    }

    /// multiply
    pub fn multiply(&self, mat : &Self) -> Option<Self> {
        if self.dim == mat.dim {
            let mut res = self.clone();
            res.multiply_mut(mat);
            Some(res)
        } else {
            None
        }
    }
}

/// 3d operation
impl Matrix {
    /// outer product
    #[allow(clippy::needless_range_loop)]
    pub fn outer_product(u: &[f64], v: &[f64])
        -> Result<Matrix, MatrixError>  {
        if !u.is_empty() && !v.is_empty() {
            let dim = std::cmp::min(u.len(), v.len());
            let mut mat = Matrix::new(dim).unwrap();
            for ridx in 0..dim {
                for cidx in 0..dim {
                    mat.set_component(ridx, cidx, u[ridx] * v[cidx]).unwrap();
                }
            }
            Ok(mat)
        } else {
            Err(MatrixError)        
        }
    }
    /// create cross product matrix
    pub fn new_cross_product(x: f64, y: f64, z:f64) -> Matrix {
        let mut result = Matrix::new(3).unwrap();
        let v = vec![z, y, x];
        for ridx in 0..3 {
            for cidx in 0..3 {
                let mut cmp = 0.0;
                if ridx != cidx {
                    cmp = v[(ridx + cidx + 2) % 3];
                }
                if cidx == (ridx + 1) % 3 {
                    cmp *= -1.0;
                }
                result.set_component(ridx, cidx, cmp).unwrap();
            }
        }
        result
    }
}

/// gl related operation
impl Matrix {

    /// create rotation matrix
    pub fn new_axis_rotation(theta: f64, x: f64, y: f64, z:f64)
        -> Result<Matrix, MatrixError> {
        let mut vec = vec![x, y, z];
        let vec_len = crate::geom::length(&vec).unwrap();
        if vec_len != 0.0 || vec_len != -0.0 {
            crate::geom::scale_0(1.0 / vec_len, &mut vec); 
            let vec_outer = Matrix::outer_product(&vec, &vec).unwrap();
            let identity = Matrix::new(3).unwrap();
            let rad = theta.to_radians(); 
            let smat = Matrix::new_cross_product(vec[0], vec[1], vec[2]);
            let result = vec_outer.clone()
                + (identity - vec_outer).scale(rad.cos())
                + smat.scale(rad.sin());

            Ok(result)
        } else {
            Err(MatrixError)
        }
    }

    /// frustum 
    pub fn new_frustum(
        left: f64, right: f64,
        bottom: f64, top: f64, near: f64, far: f64)
        -> Result<Self, MatrixError> {
        let rl = right - left;
        let tb = top - bottom;
        let fn_v = far - near;
        
        if far > 0.0 && near > 0.0
            && rl != 0.0 || rl != -0.0
            && tb != 0.0 || tb != -0.0
            && fn_v != 0.0 || fn_v != -0.0 {
            let mut result = Matrix::new(4).unwrap();
            result.set_component(0, 0, 2.0 * near / rl).unwrap(); 
            result.set_component(1, 1, 2.0 * near / tb).unwrap(); 
            result.set_component(2, 2, -(far + near) / fn_v).unwrap(); 
            result.set_component(3, 3, 0.0).unwrap();
            
            result.set_component(0, 2, (right + left) / rl).unwrap(); 
            result.set_component(1, 2, (top + bottom) / tb).unwrap(); 
            result.set_component(2, 3, (-2.0 * far * near) / fn_v).unwrap(); 
            result.set_component(3, 2, -1.0).unwrap();

            Ok(result) 
        } else {
            Err(MatrixError)
        }
    }
    /// ortho 
    pub fn new_ortho(
        left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64)
        -> Result<Self, MatrixError> {
        let rl = right - left;
        let tb = top - bottom;
        let fn_v = far - near;
        
        if rl != 0.0 || rl != -0.0
            && tb != 0.0 || tb != -0.0
            && fn_v != 0.0 || fn_v != -0.0 {
            let mut result = Matrix::new(4).unwrap();
            result.set_component(0, 0, 2.0 / rl).unwrap(); 
            result.set_component(1, 1, 2.0 / tb).unwrap(); 
            result.set_component(2, 2, -2.0 / fn_v).unwrap(); 
            result.set_component(3, 3, 0.0).unwrap();
            
            result.set_component(0, 3, -(right + left) / rl).unwrap(); 
            result.set_component(1, 3, -(top + bottom) / tb).unwrap(); 
            result.set_component(2, 3, -(far + near) / fn_v).unwrap(); 
            result.set_component(3, 3, 1.0).unwrap();

            Ok(result) 
        } else {
            Err(MatrixError)
        }
    }

    /// perspective 
    pub fn new_perspective(fovy: f64, aspect: f64, z_near: f64, z_far: f64)
        -> Result<Self, MatrixError> {
        let nf = z_near - z_far;
        
        if aspect != 0.0 && aspect != -0.0
            && nf != 0.0 || nf != -0.0 {
            let f = 1.0 / (fovy / 2.0).tan();  
            let mut result = Matrix::new(4).unwrap();
            result.set_component(0, 0, f / aspect).unwrap(); 
            result.set_component(1, 1, f).unwrap(); 
            result.set_component(2, 2, (z_far + z_near) / nf).unwrap(); 
            result.set_component(3, 3, 0.0).unwrap();
            
            result.set_component(2, 3, (2.0 * z_far * z_near) / nf).unwrap(); 
            result.set_component(3, 2, -1.0).unwrap();

            Ok(result) 
        } else {
            Err(MatrixError)
        }
    }
 
    /// the matrix to look at the object  
    #[allow(clippy::too_many_arguments)]
    pub fn new_look_at(
        eye_x: f64, eye_y: f64, eye_z: f64,
        center_x: f64, center_y: f64, center_z: f64,
        up_x: f64, up_y: f64, up_z: f64)
        -> Result<Self, MatrixError> {
        let eye_vec = vec![eye_x, eye_y, eye_z];
        let center_vec = vec![center_x, center_y, center_z];
        let mut up_vec = vec![up_x, up_y, up_z];
        let mut f = crate::geom::minus(&center_vec, &eye_vec).unwrap();
        let f_len = crate::geom::length(&f).unwrap();
        let up_vec_len = crate::geom::length(&up_vec).unwrap();
        
        if f_len != 0.0 && f_len != -0.0
            && up_vec_len != 0.0 || up_vec_len != -0.0 {
            crate::geom::scale_0(1.0 / f_len, &mut f);
            crate::geom::scale_0(1.0 / up_vec_len, &mut up_vec);
            let cross_vec_mat_f = Self::new_cross_product(f[0], f[1], f[2]);
            let s = cross_vec_mat_f.apply_r(&up_vec).unwrap();           
            let cross_vec_mat_s = Self::new_cross_product(s[0], s[1], s[2]);
            let u = cross_vec_mat_s.apply_r(&f).unwrap();
            
            let mut result = Matrix::new(4).unwrap();
            for i in 0..3 {
                result.set_component(0, i, s[i]).unwrap(); 
                result.set_component(1, i, u[i]).unwrap();
                result.set_component(2, i, -f[i]).unwrap();
            }
            result.translate_mut(- eye_x, - eye_y, - eye_z).unwrap();

            Ok(result) 
        } else {
            Err(MatrixError)
        }
    }
 
    /// apply rotation matrix from right side
    pub fn rotate_mut(&mut self, theta: f64, x: f64, y: f64, z:f64)
        -> Result<&Self, MatrixError> {
        if self.dim > 2 {
            if self.dim < 5 {
                match Self::new_axis_rotation(theta, x, y, z) {
                    Ok(mut rot_mat) => {
                        if self.dim == 4 {
                            let mut temp_mat = Self::new(4).unwrap();
                            for ridx in 0..rot_mat.get_row_count() {
                                for cidx in 0..rot_mat.get_col_count() {
                                    temp_mat.set_component(
                                        ridx, cidx,
                                        rot_mat.get_component(
                                            ridx, cidx).unwrap()).unwrap();
                                }
                            }
                            rot_mat = temp_mat;
                        }
                        let mut mat_0 = MatrixI::bind_with_col_count_0(
                            self.component.clone(), self.dim).unwrap();
                        let mat_1 = MatrixI::bind_with_col_count_0(
                            rot_mat.component.clone(), rot_mat.dim).unwrap();
                        mat_0.multiply(&mat_1).unwrap();
                        Ok(self)
                    },
                    Err(e) => Err(e) 
                }

            } else {
                Err(MatrixError)
            }


        } else {
            Err(MatrixError)
        }
    }
    /// apply rotation matrix from right side
    pub fn rotate(&self, theta: f64, x: f64, y: f64, z:f64)
        -> Result<Self, MatrixError> {
        if self.dim > 2 {
            if self.dim < 5 {
                let mut result = self.clone();
                match result.rotate_mut(theta, x, y, z) {
                    Ok(_res) => {
                        Ok(result)
                    },
                    Err(e) => Err(e)
                }
            } else {
                Err(MatrixError)
            }
        } else {
            Err(MatrixError)
        }
    }

    /// translate matrix
    #[allow(clippy::needless_range_loop)]
    pub fn translate_mut(&mut self, x: f64, y: f64, z:f64)
        -> Result<&Self, MatrixError> {
        if self.dim == 4 {
            let trans_vec = vec![x, y, z];
            for idx in 0..3 {
                self.set_component(
                    idx, 3, 
                    self.get_component(idx, 3).unwrap()
                        + trans_vec[idx]).unwrap(); 
            }
            Ok(self)        
        } else {
            Err(MatrixError)
        }
    }

    /// translate matrix
    pub fn translate(&self, x: f64, y: f64, z: f64)
        -> Result<Self, MatrixError> {
        if self.dim == 4 {
            let mut result = self.clone();
            result.translate_mut(x, y, z).unwrap();
            Ok(result)
        } else {
            Err(MatrixError)
        }
    }

    /// scale {x, y, z} component
    #[allow(clippy::needless_range_loop)]
    pub fn scale3_mut(&mut self, x: f64, y: f64, z: f64)
        -> Result<&Self, MatrixError> {
        if self.dim > 2 && self.dim < 5 {
            let scale_vec = vec![x, y, z];
            for idx in 0..3 {
                self.set_component(
                    idx, idx, 
                    self.get_component(idx, idx).unwrap()
                        * scale_vec[idx]).unwrap();
            }
            Ok(self) 
        } else {
            Err(MatrixError)
        }
    }
    /// scale {x, y, z} component
    pub fn scale3(&self, x: f64, y: f64, z: f64)
        -> Result<Self, MatrixError> {
        if self.dim > 2 && self.dim < 5 {
            let mut result = self.clone();
            result.scale3_mut(x, y, z).unwrap();
            Ok(result) 
        } else {
            Err(MatrixError)
        }
    }

    /// frustum 
    pub fn frustum_mut(&mut self,
        left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64)
        -> Result<&Self, MatrixError> {
        if self.dim == 4 {
            match Self::new_frustum(left, right, bottom, top, near, far) {
                Ok(frustum) => {
                    let mut mat_0 = MatrixI::bind_with_col_count_0(
                        self.component.clone(), self.dim).unwrap();
                    let mat_1 = MatrixI::bind_with_col_count_0(
                        frustum.component.clone(), frustum.dim).unwrap();
                    mat_0.multiply(&mat_1).unwrap();
                    Ok(self)
                },
                Err(e) => Err(e)
            }
        } else {
            Err(MatrixError)
        }
    }
    /// frustum 
    pub fn frustum(& self,
        left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64)
        -> Result<Self, MatrixError> {
        if self.dim == 4 { 
            let mut result = self.clone();
            match result.frustum_mut(left, right, bottom, top, near, far) {
                Ok(_frustum) => Ok(result),
                Err(e) => Err(e)
            }
        } else {
            Err(MatrixError)
        }
    }

    /// ortho 
    pub fn ortho_mut(&mut self,
        left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64)
        -> Result<&Self, MatrixError> {
        if self.dim == 4 {
            match Self::new_ortho(left, right, bottom, top, near, far) {
                Ok(ortho) => {
                    let mut mat_0 = MatrixI::bind_with_col_count_0(
                        self.component.clone(), self.dim).unwrap();
                    let mat_1 = MatrixI::bind_with_col_count_0(
                        ortho.component.clone(), ortho.dim).unwrap();
                    mat_0.multiply(&mat_1).unwrap();
                    Ok(self)
                },
                Err(e) => Err(e)
            }
        } else {
            Err(MatrixError)
        }
    }
    /// ortho 
    pub fn ortho(& self,
        left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64)
        -> Result<Self, MatrixError> {
        if self.dim == 4 { 
            let mut result = self.clone();
            match result.ortho_mut(left, right, bottom, top, near, far) {
                Ok(_ortho) => Ok(result),
                Err(e) => Err(e)
            }
        } else {
            Err(MatrixError)
        }
    }

    /// perspective
    pub fn perspective_mut(&mut self,
        fovy: f64, aspect: f64, z_near: f64, z_far: f64)
        -> Result<&Self, MatrixError> {
        if self.dim == 4 {
            match Self::new_perspective(fovy, aspect, z_near, z_far) {
                Ok(perspective) => {
                    let mut mat_0 = MatrixI::bind_with_col_count_0(
                        self.component.clone(), self.dim).unwrap();
                    let mat_1 = MatrixI::bind_with_col_count_0(
                        perspective.component.clone(),
                        perspective.dim).unwrap();
                    mat_0.multiply(&mat_1).unwrap();
                    Ok(self)
                },
                Err(e) => Err(e)
            }
        } else {
            Err(MatrixError)
        }
    }
    /// perspective 
    pub fn perspective(& self,
        fovy: f64, aspect: f64, z_near: f64, z_far: f64)
        -> Result<Self, MatrixError> {
        if self.dim == 4 { 
            let mut result = self.clone();
            match result.perspective_mut(fovy, aspect, z_near, z_far) {
                Ok(_perspective) => Ok(result),
                Err(e) => Err(e)
            }
        } else {
            Err(MatrixError)
        }
    }

    /// apply matrix to look at the object  
    #[allow(clippy::too_many_arguments)]
    pub fn look_at_mut(
        &mut self,
        eye_x: f64, eye_y: f64, eye_z: f64,
        center_x: f64, center_y: f64, center_z: f64,
        up_x: f64, up_y: f64, up_z: f64)
        -> Result<&Self, MatrixError> {
        if self.dim == 4 {
            match Self::new_look_at(eye_x, eye_y, eye_z,
                center_x, center_y, center_z,
                up_x, up_y, up_z) {
                Ok(look_at) => {
                    let mut mat_0 = MatrixI::bind_with_col_count_0(
                        self.component.clone(), self.dim).unwrap();
                    let mat_1 = MatrixI::bind_with_col_count_0(
                        look_at.component.clone(),
                        look_at.dim).unwrap();
                    mat_0.multiply(&mat_1).unwrap();
                    Ok(self)
                },
                Err(e) => Err(e)
            }
            
        } else {
            Err(MatrixError)
        }
    } 
    /// apply matrix to look at the object  
    #[allow(clippy::too_many_arguments)]
    pub fn look_at(
        &self,
        eye_x: f64, eye_y: f64, eye_z: f64,
        center_x: f64, center_y: f64, center_z: f64,
        up_x: f64, up_y: f64, up_z: f64) -> Result<Self, MatrixError>  {
        if self.dim == 4 {
            let mut result = self.clone();
            match result.look_at_mut(
                eye_x, eye_y, eye_z,
                center_x, center_y, center_z,
                up_x, up_y, up_z) {
                Ok(_) => {
                    Ok(result)
                },
                Err(e) => Err(e)
            }
        } else {
            Err(MatrixError)
        }
    }
}

/// clone object
impl Clone for Matrix {
    fn clone(&self) -> Self {
        let comp = self.component.borrow();
        Matrix::new_with_source_component_row_order_0(self.dim, &comp)
    }
}

/// add operator
impl Add for Matrix {
    type Output = Self;

    /// it works as operator +
    fn add(self, other: Self) -> Self {
        let res = self;
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
/// subtract operator
impl Sub for Matrix {
    type Output = Self;

    /// it works as operator - 
    fn sub(self, other: Self) -> Self {
        let res = self;
        {
            let mut mat1 =
                MatrixI::bind_with_col_count_0(res.component.clone(), res.dim)
                    .unwrap();
            let mat2 =
                MatrixI::bind_with_col_count_0(other.component, other.dim)
                    .unwrap();
            mat1.sub(&mat2).unwrap();
        }
        res
    }
}
/// multiply operator
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let res = self;
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
