use std::rc::Rc;
use ordered_float::OrderedFloat;
use wasm_bindgen::prelude::*;

/// construct distance
pub fn float_create_0(
    dis: &OrderedFloat<f64>) -> *const OrderedFloat<f64> {
    let distance_ref = Rc::new(*dis);
    Rc::into_raw(distance_ref)
}

/// construct distance
#[wasm_bindgen]
pub fn float_create(
    value: f64) -> *const OrderedFloat<f64> {
    float_create_0(&OrderedFloat(value))
}

/// increment reference count
#[wasm_bindgen]
pub fn float_retain(
    float_obj: *const OrderedFloat<f64>) -> usize {
    if !float_obj.is_null() {
        unsafe {
            let float_ref_0 = Rc::from_raw(float_obj);
            let float_ref_1 = float_ref_0.clone();
            let result = Rc::strong_count(&float_ref_0);
            Rc::into_raw(float_ref_0);
            Rc::into_raw(float_ref_1);
            result
        }
    } else {
        0 as usize
    }
}

/// decrement reference count
#[wasm_bindgen]
pub fn float_release(
    float_obj: *const OrderedFloat<f64>) -> usize {
    if !float_obj.is_null() {
        unsafe {
            let float_ref = Rc::from_raw(float_obj);
            let mut result = Rc::strong_count(&float_ref);
            result -= 1;
            result
        }
    } else {
        0 as usize
    }
}


/// get float value
#[wasm_bindgen]
pub fn float_get_value(
    float_obj: *const OrderedFloat<f64>) -> Option<f64> {
    if !float_obj.is_null() {
        unsafe {
            Some((*float_obj).into_inner())
        }
    } else {
        None
    }
}

// vi: se ts=4 sw=4 et:
