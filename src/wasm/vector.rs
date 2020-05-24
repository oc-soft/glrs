use std::rc::Rc;
use js_sys::Float64Array;
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

/// create vector
pub fn vector_create_from_vec(
    v: Vec<f64>) -> *const Vec<f64> {
    let result = Rc::new(v);
    Rc::into_raw(result)
}

/// convert Vec<f64> from Float32Array
pub fn vector_convert_to_vec64_from_32(
    components: Float32Array) -> Vec<f64> {
    let mut result  = Vec::with_capacity(components.length() as usize);
    for i in 0..components.length() {
        result.push(components.get_index(i) as f64);
    }
    result
}

/// convert Float64Array from Vec<f64>
pub fn vector_convert_to_vec64_from_64(
    components: &Vec<f64>) -> Float64Array {
    let mut result  = Float64Array::new_with_length(components.len() as u32);
    for i in 0..components.len() {
        result.set_index(i as u32, components[i]);
    }
    result
}

/// convert Float64Array from Vec<f64>
pub fn vector_convert_to_vec32_from_64(
    components: &Vec<f64>) -> Float32Array {
    let mut result  = Float32Array::new_with_length(components.len() as u32);
    for i in 0..components.len() {
        result.set_index(i as u32, components[i] as f32);
    }
    result
}


/// create vector
#[wasm_bindgen]
pub fn vector_create(components: Float64Array) -> *const Vec<f64> {
    vector_create_from_vec(components.to_vec())
}

/// get vector components
#[wasm_bindgen]
pub fn vector_get_components(v: *const Vec<f64>) -> Option<Float64Array> {
    if std::ptr::null() != v {
        unsafe {
            let array = Float64Array::new_with_length((*v).len() as u32);
            for i in 0..(*v).len() {
                array.set_index(i as u32, (*v)[i]);
            }
            Some(array)
        }
    } else {
        None
    }
}

/// get vector components
#[wasm_bindgen]
pub fn vector_get_components_32(v: *const Vec<f64>) -> Option<Float32Array> {
    if std::ptr::null() != v {
        unsafe {
            let array = Float32Array::new_with_length((*v).len() as u32);
            for i in 0..(*v).len() {
                array.set_index(i as u32, (*v)[i] as f32);
            }
            Some(array)
        }
    } else {
        None
    }
}


/// get a component in vector
#[wasm_bindgen]
pub fn vector_get_component(v: *const Vec<f64>, idx: usize) -> Option<f64> {
    if std::ptr::null() != v {
        unsafe {
            if idx < (*v).len() {
                Some((*v)[idx])
            } else {
                None
            }
        }
    } else {
        None
    }
}

/// increment reference count
#[wasm_bindgen]
pub fn vector_retain(v: *const Vec<f64>) -> usize {
    if std::ptr::null() != v {
        unsafe {
            let v1 = Rc::from_raw(v);
            let v2 = v1.clone();
            let result = Rc::strong_count(&v2);
            Rc::into_raw(v1);
            Rc::into_raw(v2);
            result
        }
    } else {
        0 as usize
    }
}

/// decrement refenrece count
#[wasm_bindgen]
pub fn vector_release(v: *const Vec<f64>) -> usize {
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

/// vector dimension
#[wasm_bindgen]
pub fn vector_dimension(v: *const Vec<f64>) -> usize {
    if std::ptr::null() != v {
        unsafe { (*v).len() }
    } else {
        0 as usize
    }
}
// vi: se ts=4 sw=4 et:
