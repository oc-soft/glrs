use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use crate::Matrix;

/// create matrix
#[wasm_bindgen]
pub fn matrix_create_with_components(
    components: Clamped<Vec<f64>>,
) -> *const Matrix {
    let mut comps = Vec::with_capacity(components.len());
    for comp in components.iter() {
        comps.push(*comp);
    }
    let mat = Matrix::new_with_source_component(&comps);
    let result = Rc::new(mat);
    Rc::into_raw(result)
}

/// create matrix
#[wasm_bindgen]
pub fn matrix_create_with_dimension(dim: usize) -> *const Matrix {
    match Matrix::new(dim) {
        Ok(mat) => {
            let result = Rc::new(mat);
            Rc::into_raw(result)
        }
        Err(_) => std::ptr::null(),
    }
}

/// increment reference count
#[wasm_bindgen]
pub fn matrix_retain(mat: *const Matrix) -> usize {
    if std::ptr::null() != mat {
        unsafe {
            if std::ptr::null() != mat {
                let mat_rc_0 = Rc::from_raw(mat);
                let mat_rc_1 = mat_rc_0.clone();
                let result = Rc::strong_count(&mat_rc_0);
                Rc::into_raw(mat_rc_0);
                Rc::into_raw(mat_rc_1);
                result
            } else {
                0 as usize
            }
        }
    } else {
        0 as usize
    }
}

/// increment reference count
#[wasm_bindgen]
pub fn matrix_release(mat: *const Matrix) -> usize {
    if std::ptr::null() != mat {
        unsafe {
            if std::ptr::null() != mat {
                let mat_rc_0 = Rc::from_raw(mat);
                let mut result = Rc::strong_count(&mat_rc_0);
                result -= 1;
                result
            } else {
                0 as usize
            }
        }
    } else {
        0 as usize
    }
}
// vi: se ts=4 sw=4 et:
