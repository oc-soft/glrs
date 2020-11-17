use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::Distance;

/// construct distance
pub fn distance_create_0(
    dis: &Distance) -> *const Distance {
    let distance_ref = Rc::new(*dis);
    Rc::into_raw(distance_ref)
}

/// construct distance
#[wasm_bindgen]
pub fn distance_create(
    dis: f64) -> *const Distance {
    distance_create_0(&Distance::new(&dis))
}
/// increment reference count
#[wasm_bindgen]
pub fn distance_retain(
    distance: *const Distance) -> usize {
    if !distance.is_null() {
        unsafe {
            let distance_ref_0 = Rc::from_raw(distance);
            let distance_ref_1 = distance_ref_0.clone();
            let result = Rc::strong_count(&distance_ref_0);
            Rc::into_raw(distance_ref_0);
            Rc::into_raw(distance_ref_1);
            result
        }
    } else {
        0 as usize
    }
}

/// decrement reference count
#[wasm_bindgen]
pub fn distance_release(
    distance: *const Distance) -> usize {
    if !distance.is_null() {
        unsafe {
            let distance_ref = Rc::from_raw(distance);
            let mut result = Rc::strong_count(&distance_ref);
            result -= 1;
            result
        }
    } else {
        0 as usize
    }
}


/// get distance value
#[wasm_bindgen]
pub fn distance_get_value(
    distance: *const Distance) -> Option<f64> {
    if !distance.is_null() {
        unsafe {
            Some((*distance).get_distance())
        }
    } else {
        None
    }
}

/// get absolute distance value
#[wasm_bindgen]
pub fn distance_get_abs_value(
    distance: *const Distance) -> Option<f64> {
    if !distance.is_null() {
        unsafe {
            Some((*distance).get_abs_distance())
        }
    } else {
        None
    }
}


// vi: se ts=4 sw=4 et:
