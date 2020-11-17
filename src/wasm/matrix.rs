use std::cell::RefCell;
use std::rc::Rc;

use crate::Matrix;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

use js_sys::Float32Array;
use js_sys::Float64Array;

/// create matrix
#[wasm_bindgen]
pub fn matrix_create_with_components_row_order(
    components: Clamped<Vec<f64>>,
) -> *const RefCell<Matrix> {
    let mut comps = Vec::with_capacity(components.len());
    for comp in components.iter() {
        comps.push(*comp);
    }
    let mat = Matrix::new_with_source_component_row_order(&comps);
    let result = Rc::new(RefCell::new(mat));
    Rc::into_raw(result)
}
#[wasm_bindgen]
pub fn matrix_create_with_components_col_order(
    components: Clamped<Vec<f64>>,
) -> *const RefCell<Matrix> {
    let mut comps = Vec::with_capacity(components.len());
    for comp in components.iter() {
        comps.push(*comp);
    }
    let mat = Matrix::new_with_source_component_col_order(&comps);
    let result = Rc::new(RefCell::new(mat));
    Rc::into_raw(result)
}

/// create matrix
#[wasm_bindgen]
pub fn matrix_create_with_dimension(dim: usize) -> *const RefCell<Matrix> {
    match Matrix::new(dim) {
        Ok(mat) => {
            let result = Rc::new(RefCell::new(mat));
            Rc::into_raw(result)
        }
        Err(_) => std::ptr::null(),
    }
}

/// increment reference count
#[wasm_bindgen]
pub fn matrix_retain(mat: *const RefCell<Matrix>) -> usize {
    if !mat.is_null() {
        unsafe {
            if !mat.is_null() {
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
pub fn matrix_release(mat: *const RefCell<Matrix>) -> usize {
    if !mat.is_null() {
        unsafe {
            if !mat.is_null() {
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

/// components as array
#[wasm_bindgen]
pub fn matrix_get_components_col_order(
    mat: *const RefCell<Matrix>,
) -> Option<Float64Array> {
    if !mat.is_null() {
        unsafe {
            let array = Float64Array::new_with_length(
                ((*mat).borrow().get_row_count()
                    * (*mat).borrow().get_col_count()) as u32,
            );
            (*mat).borrow().iterate_col_order(&mut |idx, val| {
                array.set_index(idx as u32, val);
                true
            });
            Some(array)
        }
    } else {
        None
    }
}
/// components as array
#[wasm_bindgen]
pub fn matrix_get_components_col_order_32(
    mat: *const RefCell<Matrix>,
) -> Option<Float32Array> {
    if !mat.is_null() {
        unsafe {
            let array = Float32Array::new_with_length(
                ((*mat).borrow().get_row_count()
                    * (*mat).borrow().get_col_count()) as u32,
            );
            (*mat).borrow().iterate_col_order(&mut |idx, val| {
                array.set_index(idx as u32, val as f32);
                true
            });
            Some(array)
        }
    } else {
        None
    }
}

/// components as array
#[wasm_bindgen]
pub fn matrix_get_components_row_order(
    mat: *const RefCell<Matrix>,
) -> Option<Float64Array> {
    if !mat.is_null() {
        unsafe {
            let array = Float64Array::new_with_length(
                ((*mat).borrow().get_row_count()
                    * (*mat).borrow().get_col_count()) as u32,
            );
            (*mat).borrow().iterate_row_order(&mut |idx, val| {
                array.set_index(idx as u32, val);
                true
            });
            Some(array)
        }
    } else {
        None
    }
}
/// components as array
#[wasm_bindgen]
pub fn matrix_get_components_row_order_32(
    mat: *const RefCell<Matrix>,
) -> Option<Float32Array> {
    if !mat.is_null() {
        unsafe {
            let array = Float32Array::new_with_length(
                ((*mat).borrow().get_row_count()
                    * (*mat).borrow().get_col_count()) as u32,
            );
            (*mat).borrow().iterate_row_order(&mut |idx, val| {
                array.set_index(idx as u32, val as f32);
                true
            });
            Some(array)
        }
    } else {
        None
    }
}

/// mutiply two matrices
#[wasm_bindgen]
pub fn matrix_multiply_mut(
    mat1: *const RefCell<Matrix>,
    mat2: *const RefCell<Matrix>,
) -> Option<bool> {
    if !mat1.is_null() && !mat2.is_null() {
        unsafe { Some((*mat1).borrow_mut().multiply_mut(&(*mat2).borrow())) }
    } else {
        None
    }
}

/// mutiply two matrices
#[wasm_bindgen]
pub fn matrix_multiply(
    mat1: *const RefCell<Matrix>,
    mat2: *const RefCell<Matrix>,
) -> *const RefCell<Matrix> {
    if !mat1.is_null() && !mat2.is_null() {
        unsafe {
            match (*mat1).borrow().multiply(&(*mat2).borrow()) {
                Some(mat) => {
                    let result = Rc::new(RefCell::new(mat));
                    Rc::into_raw(result)
                }
                None => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// calculate inverse matrix
#[wasm_bindgen]
pub fn matrix_inverse(mat: *const RefCell<Matrix>) -> *const RefCell<Matrix> {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow().inverse() {
                Some(mat_inv) => {
                    let result = Rc::new(RefCell::new(mat_inv));
                    Rc::into_raw(result)
                }
                None => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// calculate inverse matrix
#[wasm_bindgen]
pub fn matrix_scale_mut(mat: *const RefCell<Matrix>, scale: f64) -> bool {
    if !mat.is_null() {
        unsafe {
            (*mat).borrow_mut().scale_mut(scale);
            true
        }
    } else {
        false
    }
}

/// calculate inverse matrix
#[wasm_bindgen]
pub fn matrix_scale(
    mat: *const RefCell<Matrix>,
    scale: f64,
) -> *const RefCell<Matrix> {
    if !mat.is_null() {
        unsafe {
            let mat_res = (*mat).borrow().scale(scale);
            let result = Rc::new(RefCell::new(mat_res));
            Rc::into_raw(result)
        }
    } else {
        std::ptr::null()
    }
}

/// apply vector to left side matrix
#[wasm_bindgen]
pub fn matrix_apply_l_with_vec(
    mat: *const RefCell<Matrix>,
    v: *const Vec<f64>,
) -> *const Vec<f64> {
    if !mat.is_null() && !v.is_null() {
        unsafe {
            match (*mat).borrow().apply_l(&*v) {
                Ok(vec_res) => super::vector::vector_create_from_vec(vec_res),
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// apply vector to right side matrix
#[wasm_bindgen]
pub fn matrix_apply_r_with_vec(
    mat: *const RefCell<Matrix>,
    v: *const Vec<f64>,
) -> *const Vec<f64> {
    if !mat.is_null() && !v.is_null() {
        unsafe {
            match (*mat).borrow().apply_r(&*v) {
                Ok(vec_res) => super::vector::vector_create_from_vec(vec_res),
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// apply vector to left side matrix
#[wasm_bindgen]
pub fn matrix_apply_l_64(
    mat: *const RefCell<Matrix>,
    v: Float64Array,
) -> Option<Float64Array> {
    if !mat.is_null() {
        let vec = v.to_vec();
        unsafe {
            match (*mat).borrow().apply_l(&vec) {
                Ok(vec_res) => Some(
                    super::vector::vector_convert_to_array64_from_64(&vec_res),
                ),
                Err(_) => None,
            }
        }
    } else {
        None
    }
}

/// apply vector to left side matrix
#[wasm_bindgen]
pub fn matrix_apply_l_32(
    mat: *const RefCell<Matrix>,
    v: Float32Array,
) -> Option<Float32Array> {
    if !mat.is_null() {
        let vec = super::vector::vector_convert_to_vec64_from_32(v);
        unsafe {
            match (*mat).borrow().apply_l(&vec) {
                Ok(vec_res) => Some(
                    super::vector::vector_convert_to_array32_from_64(&vec_res),
                ),
                Err(_) => None,
            }
        }
    } else {
        None
    }
}

/// apply vector to right side matrix
#[wasm_bindgen]
pub fn matrix_apply_r_64(
    mat: *const RefCell<Matrix>,
    v: Float64Array,
) -> Option<Float64Array> {
    if !mat.is_null() {
        let vec = v.to_vec();
        unsafe {
            match (*mat).borrow().apply_r(&vec) {
                Ok(vec_res) => Some(
                    super::vector::vector_convert_to_array64_from_64(&vec_res),
                ),
                Err(_) => None,
            }
        }
    } else {
        None
    }
}

/// apply vector to right side matrix
#[wasm_bindgen]
pub fn matrix_apply_r_32(
    mat: *const RefCell<Matrix>,
    v: Float32Array,
) -> Option<Float32Array> {
    if !mat.is_null() {
        let vec = super::vector::vector_convert_to_vec64_from_32(v);
        unsafe {
            match (*mat).borrow().apply_r(&vec) {
                Ok(vec_res) => Some(
                    super::vector::vector_convert_to_array32_from_64(&vec_res),
                ),
                Err(_) => None,
            }
        }
    } else {
        None
    }
}

/// create cross product matrix
#[wasm_bindgen]
pub fn matrix_new_cross_product(
    x: f64,
    y: f64,
    z: f64,
) -> *const RefCell<Matrix> {
    let mat = Matrix::new_cross_product(x, y, z);
    let result = Rc::new(RefCell::new(mat));
    Rc::into_raw(result)
}

/// create axis rotation matrix
#[wasm_bindgen]
pub fn matrix_new_axis_rotation(
    theta: f64,
    x: f64,
    y: f64,
    z: f64,
) -> *const RefCell<Matrix> {
    match Matrix::new_axis_rotation(theta, x, y, z) {
        Ok(mat) => {
            let result = Rc::new(RefCell::new(mat));
            Rc::into_raw(result)
        }
        Err(_) => std::ptr::null(),
    }
}

/// apply rotatation matrix
#[wasm_bindgen]
pub fn matrix_rotate_mut(
    mat: *const RefCell<Matrix>,
    theta: f64,
    x: f64,
    y: f64,
    z: f64,
) -> bool {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow_mut().rotate_mut(theta, x, y, z) {
                Ok(_mat_ref) => true,
                Err(_) => false,
            }
        }
    } else {
        false
    }
}

/// apply rotatation matrix
#[wasm_bindgen]
pub fn matrix_rotate(
    mat: *const RefCell<Matrix>,
    theta: f64,
    x: f64,
    y: f64,
    z: f64,
) -> *const RefCell<Matrix> {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow().rotate(theta, x, y, z) {
                Ok(mat_res) => {
                    let result = Rc::new(RefCell::new(mat_res));
                    Rc::into_raw(result)
                }
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// apply rotatation matrix
#[wasm_bindgen]
pub fn matrix_translate_mut(
    mat: *const RefCell<Matrix>,
    x: f64,
    y: f64,
    z: f64,
) -> bool {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow_mut().translate_mut(x, y, z) {
                Ok(_mat_ref) => true,
                Err(_) => false,
            }
        }
    } else {
        false
    }
}

/// apply translate matrix
#[wasm_bindgen]
pub fn matrix_translate(
    mat: *const RefCell<Matrix>,
    x: f64,
    y: f64,
    z: f64,
) -> *const RefCell<Matrix> {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow().translate(x, y, z) {
                Ok(mat_res) => {
                    let result = Rc::new(RefCell::new(mat_res));
                    Rc::into_raw(result)
                }
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// apply scale matrix
#[wasm_bindgen]
pub fn matrix_scale3_mut(
    mat: *const RefCell<Matrix>,
    x: f64,
    y: f64,
    z: f64,
) -> bool {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow_mut().scale3_mut(x, y, z) {
                Ok(_mat_ref) => true,
                Err(_) => false,
            }
        }
    } else {
        false
    }
}

/// apply scale matrix
#[wasm_bindgen]
pub fn matrix_scale3(
    mat: *const RefCell<Matrix>,
    x: f64,
    y: f64,
    z: f64,
) -> *const RefCell<Matrix> {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow().scale3(x, y, z) {
                Ok(mat_res) => {
                    let result = Rc::new(RefCell::new(mat_res));
                    Rc::into_raw(result)
                }
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// create frustum matrix
#[wasm_bindgen]
pub fn matrix_new_flustum(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> *const RefCell<Matrix> {
    match Matrix::new_frustum(left, right, bottom, top, near, far) {
        Ok(mat) => {
            let result = Rc::new(RefCell::new(mat));
            Rc::into_raw(result)
        }
        Err(_) => std::ptr::null(),
    }
}

/// apply frustum matrix
#[wasm_bindgen]
pub fn matrix_frustum_mut(
    mat: *const RefCell<Matrix>,
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> bool {
    if !mat.is_null() {
        unsafe {
            match (*mat)
                .borrow_mut()
                .frustum_mut(left, right, bottom, top, near, far)
            {
                Ok(_mat_ref) => true,
                Err(_) => false,
            }
        }
    } else {
        false
    }
}

/// apply frustum matrix
#[wasm_bindgen]
pub fn matrix_frustum(
    mat: *const RefCell<Matrix>,
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> *const RefCell<Matrix> {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow().frustum(left, right, bottom, top, near, far)
            {
                Ok(mat_res) => {
                    let result = Rc::new(RefCell::new(mat_res));
                    Rc::into_raw(result)
                }
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// create ortho matrix
#[wasm_bindgen]
pub fn matrix_new_ortho(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> *const RefCell<Matrix> {
    match Matrix::new_ortho(left, right, bottom, top, near, far) {
        Ok(mat) => {
            let result = Rc::new(RefCell::new(mat));
            Rc::into_raw(result)
        }
        Err(_) => std::ptr::null(),
    }
}

/// apply ortho  matrix
#[wasm_bindgen]
pub fn matrix_ortho_mut(
    mat: *const RefCell<Matrix>,
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> bool {
    if !mat.is_null() {
        unsafe {
            match (*mat)
                .borrow_mut()
                .ortho_mut(left, right, bottom, top, near, far)
            {
                Ok(_mat_ref) => true,
                Err(_) => false,
            }
        }
    } else {
        false
    }
}

/// apply ortho matrix
#[wasm_bindgen]
pub fn matrix_ortho(
    mat: *const RefCell<Matrix>,
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> *const RefCell<Matrix> {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow().ortho(left, right, bottom, top, near, far) {
                Ok(mat_res) => {
                    let result = Rc::new(RefCell::new(mat_res));
                    Rc::into_raw(result)
                }
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// create perspective matrix
#[wasm_bindgen]
pub fn matrix_new_perspective(
    fovy: f64,
    aspect: f64,
    z_near: f64,
    z_far: f64,
) -> *const RefCell<Matrix> {
    match Matrix::new_perspective(fovy, aspect, z_near, z_far) {
        Ok(mat) => {
            let result = Rc::new(RefCell::new(mat));
            Rc::into_raw(result)
        }
        Err(_) => std::ptr::null(),
    }
}

/// apply perspective  matrix
#[wasm_bindgen]
pub fn matrix_perspective_mut(
    mat: *const RefCell<Matrix>,
    fovy: f64,
    aspect: f64,
    z_near: f64,
    z_far: f64,
) -> bool {
    if !mat.is_null() {
        unsafe {
            match (*mat)
                .borrow_mut()
                .perspective_mut(fovy, aspect, z_near, z_far)
            {
                Ok(_mat_ref) => true,
                Err(_) => false,
            }
        }
    } else {
        false
    }
}

/// apply perspective matrix
#[wasm_bindgen]
pub fn matrix_perspective(
    mat: *const RefCell<Matrix>,
    fovy: f64,
    aspect: f64,
    z_near: f64,
    z_far: f64,
) -> *const RefCell<Matrix> {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow().perspective(fovy, aspect, z_near, z_far) {
                Ok(mat_res) => {
                    let result = Rc::new(RefCell::new(mat_res));
                    Rc::into_raw(result)
                }
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// create look at matrix
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn matrix_new_look_at(
    eye_x: f64,
    eye_y: f64,
    eye_z: f64,
    center_x: f64,
    center_y: f64,
    center_z: f64,
    up_x: f64,
    up_y: f64,
    up_z: f64,
) -> *const RefCell<Matrix> {
    match Matrix::new_look_at(
        eye_x, eye_y, eye_z, center_x, center_y, center_z, up_x, up_y, up_z,
    ) {
        Ok(mat) => {
            let result = Rc::new(RefCell::new(mat));
            Rc::into_raw(result)
        }
        Err(_) => std::ptr::null(),
    }
}

/// apply look at matrix
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn matrix_look_at_mut(
    mat: *const RefCell<Matrix>,
    eye_x: f64,
    eye_y: f64,
    eye_z: f64,
    center_x: f64,
    center_y: f64,
    center_z: f64,
    up_x: f64,
    up_y: f64,
    up_z: f64,
) -> bool {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow_mut().look_at_mut(
                eye_x, eye_y, eye_z, center_x, center_y, center_z, up_x, up_y,
                up_z,
            ) {
                Ok(_mat_ref) => true,
                Err(_) => false,
            }
        }
    } else {
        false
    }
}

/// apply perspective matrix
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn matrix_look_at(
    mat: *const RefCell<Matrix>,
    eye_x: f64,
    eye_y: f64,
    eye_z: f64,
    center_x: f64,
    center_y: f64,
    center_z: f64,
    up_x: f64,
    up_y: f64,
    up_z: f64,
) -> *const RefCell<Matrix> {
    if !mat.is_null() {
        unsafe {
            match (*mat).borrow().look_at(
                eye_x, eye_y, eye_z, center_x, center_y, center_z, up_x, up_y,
                up_z,
            ) {
                Ok(mat_res) => {
                    let result = Rc::new(RefCell::new(mat_res));
                    Rc::into_raw(result)
                }
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

// vi: se ts=4 sw=4 et:
