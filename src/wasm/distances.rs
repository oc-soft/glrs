use std::rc::Rc;
use wasm_bindgen::prelude::*;
use super::distance::*;

use crate::Distance;


/// increment reference count
#[wasm_bindgen]
pub fn distances_retain(
    distances: *const Vec<Distance>) -> usize {
    unsafe {
        let distances_ref_0 = Rc::from_raw(distances);
        let distances_ref_1 = distances_ref_0.clone();
        let result = Rc::strong_count(&distances_ref_0);
        Rc::into_raw(distances_ref_0);
        Rc::into_raw(distances_ref_1);
        result
    }
}

/// decrement reference count
#[wasm_bindgen]
pub fn distances_release(
    distances: *const Vec<Distance>) -> usize {
    unsafe {
        let distances_ref = Rc::from_raw(distances);
        let mut result = Rc::strong_count(&distances_ref);
        result -= 1;
        result
    }
}


/// get count of distanceses
#[wasm_bindgen]
pub fn distances_size(
    distances: *const Vec<Distance>) -> usize {
    if !distances.is_null() {
        unsafe {
            (*distances).len()
        }
    } else {
        0 as usize
    }
}

/// get distance at index
#[wasm_bindgen]
pub fn distances_get(
    distances: *const Vec<Distance>,
    idx: usize) -> *const Distance {
    if !distances.is_null() {
        unsafe {
            if idx < distances_size(distances) {
                distance_create_0(&(*distances)[idx])
            } else {
                std::ptr::null()
            }
        }
    } else {
        std::ptr::null()
    }
}
// vi: se ts=4 sw=4 et:
