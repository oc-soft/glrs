use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::Float64Array;
use crate::Plane;
use crate::Distance;
use super::distance_indices::*;
use super::vector::*;
use super::vector_array::*;

/// instanciate plane
#[wasm_bindgen]
pub fn plane_create_0(n: *const Vec<f64>, c: *const Vec<f64>)
    -> *const Plane {
    if std::ptr::null() != n && std::ptr::null() != c {
        unsafe {
            match Plane::create(&*n, &*c) {
                Ok(p) => Rc::into_raw(Rc::new(p)),
                Err(_) => std::ptr::null(),
            }
        }
    } else {
        std::ptr::null()
    }
}

/// create plane instance
#[wasm_bindgen]
pub fn plane_create(n: Float64Array, c: Float64Array)
    -> *const Plane {
    let n_vec = vector_create(n); 
    let c_vec = vector_create(c);
    let result = plane_create_0(n_vec, c_vec);
    vector_release(n_vec);
    vector_release(c_vec);
    result
}

/// increment reference count
#[wasm_bindgen]
pub fn plane_retain(p: *const Plane) -> usize {
    if std::ptr::null() != p {
        unsafe {
            let p_ref_0 = Rc::from_raw(p);
            let p_ref_1 = p_ref_0.clone();
            let result = Rc::strong_count(&p_ref_0);
            Rc::into_raw(p_ref_0);
            Rc::into_raw(p_ref_1);
            result
        }
    } else {
        0 as usize
    }
}

/// decrement reference count
#[wasm_bindgen]
pub fn plane_release(p: *const Plane) -> usize {
    if std::ptr::null() != p {
        unsafe {
            let p_ref_0 = Rc::from_raw(p);
            let mut result = Rc::strong_count(&p_ref_0);
            result -= 1;
            result
        }
    } else {
        0 as usize
    }
}

/// get dimension
#[wasm_bindgen]
pub fn plane_get_dimension(
    p: *const Plane) -> Option<usize> {
    if std::ptr::null() != p {
        unsafe {
            Some((*p).get_dimension())
        }
    } else {
        None
    }
}

/// calcurate distance
#[wasm_bindgen]
pub fn plane_distance_0(
    p: *const Plane,
    v: *const Vec<f64>) -> Option<f64> {
    if std::ptr::null() != p && std::ptr::null() != v {
        unsafe {
            match (*p).distance(&(*v)) {
                Ok(dis) => Some(dis),
                Err(_) => None
            }
        }
    } else {
        None
    }
}


/// calcurate distance
#[wasm_bindgen]
pub fn plane_distance(
    p: *const Plane,
    v: Float64Array) -> Option<f64> {
    if std::ptr::null() != p {
        let vec = vector_create(v);  
        let result = plane_distance_0(p, vec);
        vector_release(vec);
        result
    } else {
        None
    }
}



/// sort points
#[wasm_bindgen]
pub fn plane_sort_points_0(
    p: *const Plane,
    va: *const RefCell<Vec<Vec<f64>>>,
) -> *const BTreeMap<Distance, Rc<RefCell<Vec<usize>>>> {
    if std::ptr::null() != p && std::ptr::null() != va {
        unsafe {
            let sorted_indices = (*p).sort_points(&(*va).borrow());
            distance_indices_create_0(sorted_indices)
        }
    } else {
        std::ptr::null()
    }
}

/// sort points
#[wasm_bindgen]
pub fn plane_sort_points(
    p: *const Plane,
    point_container: JsValue)
    -> *const BTreeMap<Distance, Rc<RefCell<Vec<usize>>>> {
    if std::ptr::null() != p {
        if point_container.is_object() {
            let array_js = js_sys::Array::from(&point_container);
            let vec_array = vector_array_create();

            for i in 0..array_js.length() {
                let elem_js = array_js.get(i);
                if elem_js.is_object() {
                    let va_js = js_sys::Array::from(&elem_js);
                    let mut vec = Vec::new();
                    for j in 0..va_js.length() {
                        match va_js.get(j).as_f64() {
                            Some(val) => vec.push(val),
                            None => break
                        }
                    }
                    vector_array_add_0(vec_array, &vec);
                } else {
                    vector_array_add_0(vec_array, &Vec::new());  
                }
            }
            let result = plane_sort_points_0(p, vec_array);
            vector_array_release(vec_array);
            result
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}

// vi: se ts=4 sw=4 et:
