use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use js_sys::Float64Array;
use super::vector::*;

/// create vectors
#[wasm_bindgen]
pub fn vector_array_create() -> *const RefCell<Vec<Vec<f64>>> {
    let vec: Vec<Vec<f64>> = Vec::new();
    let vec_ref = Rc::new(RefCell::new(vec));
    Rc::into_raw(vec_ref)
}

/// increment reference count
#[wasm_bindgen]
pub fn vector_array_retain(v: *const RefCell<Vec<Vec<f64>>>) -> usize {
    if std::ptr::null() != v {
        unsafe {
            let vec = Rc::from_raw(v);
            let result = Rc::strong_count(&vec);
            Rc::into_raw(vec);
            result
        }
    } else {
        0 as usize
    }
}

/// increment reference count
#[wasm_bindgen]
pub fn vector_array_release(v: *const RefCell<Vec<Vec<f64>>>) -> usize {
    if std::ptr::null() != v {
        unsafe {
            let vec = Rc::from_raw(v);
            let mut result = Rc::strong_count(&vec);
            result -= 1;
            result
        }
    } else {
        0 as usize
    }
}

/// get dimension
#[wasm_bindgen]
pub fn vector_array_dimension(
    v: *const RefCell<Vec<Vec<f64>>>,
) -> Option<usize> {
    if std::ptr::null() != v {
        unsafe {
            match (*v).borrow().len() {
                0 => None,
                _ => Some((*v).borrow()[0].len()),
            }
        }
    } else {
        None
    }
}


/// get size 
#[wasm_bindgen]
pub fn vector_array_size(
    v: *const RefCell<Vec<Vec<f64>>>,
) -> Option<usize> {
    if std::ptr::null() != v {
        unsafe {
            Some((*v).borrow().len())
        }
    } else {
        None
    }
}



/// add vector
#[wasm_bindgen]
pub fn vector_array_add_0(
    va: *const RefCell<Vec<Vec<f64>>>,
    v: *const Vec<f64>,
) -> bool {
    if std::ptr::null() != va && std::ptr::null() != v {
        match vector_array_dimension(va) {
            Some(dim) => unsafe {
                if (*v).len() >= dim {
                    let mut vec = Vec::with_capacity(dim);
                    for i in 0..dim {
                        vec.push((*v)[i]);
                    }
                    (*va).borrow_mut().push(vec);
                    true
                } else {
                    false
                }
            },
            None => unsafe {
                let mut vec = Vec::with_capacity((*v).len());
                for i in 0..(*v).len() {
                    vec.push((*v)[i]);
                }
                (*va).borrow_mut().push(vec);
                true
            },
        }
    } else {
        false
    }
}

/// add vector
#[wasm_bindgen]
pub fn vector_array_add_1(
    va: *const RefCell<Vec<Vec<f64>>>,
    v: Float64Array,
) -> bool {
    if std::ptr::null() != va {
        let vec_raw = vector_create(v);
        let result = vector_array_add_0(va, vec_raw);
        vector_release(vec_raw);
        result
    } else {
        false
    }
}


// vi: se ts=4 sw=4 et:
