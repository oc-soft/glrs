use std::rc::Rc;
use ordered_float::OrderedFloat;
use wasm_bindgen::prelude::*;
use super::float::*;


/// increment reference count
#[wasm_bindgen]
pub fn float_vec_retain(
    vec: *const Vec<OrderedFloat<f64>>) -> usize {
    unsafe {
        let vec_ref_0 = Rc::from_raw(vec);
        let vec_ref_1 = vec_ref_0.clone();
        let result = Rc::strong_count(&vec_ref_0);
        Rc::into_raw(vec_ref_0);
        Rc::into_raw(vec_ref_1);
        result
    }
}

/// decrement reference count
#[wasm_bindgen]
pub fn float_vec_release(
    vec: *const Vec<OrderedFloat<f64>>) -> usize {
    unsafe {
        let vec_ref = Rc::from_raw(vec);
        let mut result = Rc::strong_count(&vec_ref);
        result -= 1;
        result
    }
}


/// get count of distanceses
#[wasm_bindgen]
pub fn float_vec_size(
    vec: *const Vec<OrderedFloat<f64>>) -> usize {
    if !vec.is_null() {
        unsafe {
            (*vec).len()
        }
    } else {
        0 as usize
    }
}

/// get float object at index
#[wasm_bindgen]
pub fn float_vec_get(
    vec: *const Vec<OrderedFloat<f64>>,
    idx: usize) -> *const OrderedFloat<f64> {
    if !vec.is_null() {
        unsafe {
            if idx < float_vec_size(vec) {
                float_create_0(&(*vec)[idx])
            } else {
                std::ptr::null()
            }
        }
    } else {
        std::ptr::null()
    }
}
// vi: se ts=4 sw=4 et:
