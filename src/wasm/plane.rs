use super::distance_indices::*;
use super::float_indices::*;
use super::vector::*;
use super::vector_array::*;
use crate::Distance;
use crate::Plane;
use js_sys::Float64Array;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/// instanciate plane
#[wasm_bindgen]
pub fn plane_create_0(n: *const Vec<f64>, c: *const Vec<f64>) -> *const Plane {
    if !n.is_null() && !c.is_null() {
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
#[wasm_bindgen]
pub fn plane_create_with_2d_0(
    p1: *const Vec<f64>,
    p2: *const Vec<f64>,
) -> *const Plane {
    if !p1.is_null() && !p2.is_null() {
        unsafe {
            match Plane::create_with_2d(&*p1, &*p2) {
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
pub fn plane_create(n: Float64Array, c: Float64Array) -> *const Plane {
    let n_vec = vector_create(n);
    let c_vec = vector_create(c);
    let result = plane_create_0(n_vec, c_vec);
    vector_release(n_vec);
    vector_release(c_vec);
    result
}

/// create plane instance
#[wasm_bindgen]
pub fn plane_create_with_2d(
    p1: Float64Array,
    p2: Float64Array,
) -> *const Plane {
    let p1_vec = vector_create(p1);
    let p2_vec = vector_create(p2);
    let result = plane_create_with_2d_0(p1_vec, p2_vec);
    vector_release(p1_vec);
    vector_release(p2_vec);
    result
}
/// increment reference count
#[wasm_bindgen]
pub fn plane_retain(p: *const Plane) -> usize {
    if !p.is_null() {
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
    if !p.is_null() {
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
pub fn plane_get_dimension(p: *const Plane) -> Option<usize> {
    if !p.is_null() {
        unsafe { Some((*p).get_dimension()) }
    } else {
        None
    }
}

/// calcurate distance
#[wasm_bindgen]
pub fn plane_distance_0(p: *const Plane, v: *const Vec<f64>) -> Option<f64> {
    if !p.is_null() && !v.is_null() {
        unsafe {
            match (*p).distance(&(*v)) {
                Ok(dis) => Some(dis),
                Err(_) => None,
            }
        }
    } else {
        None
    }
}

/// calcurate distance
#[wasm_bindgen]
pub fn plane_distance(p: *const Plane, v: Float64Array) -> Option<f64> {
    if !p.is_null() {
        let vec = vector_create(v);
        let result = plane_distance_0(p, vec);
        vector_release(vec);
        result
    } else {
        None
    }
}

/// project point to plane
#[wasm_bindgen]
pub fn plane_project_0(
    p: *const Plane,
    v: *const Vec<f64>,
) -> Option<Vec<f64>> {
    if !p.is_null() && !v.is_null() {
        unsafe {
            match (*p).project(&(*v)) {
                Ok(proj_pt) => Some(proj_pt),
                Err(_) => None,
            }
        }
    } else {
        None
    }
}

/// project the point on plane
#[wasm_bindgen]
pub fn plane_project(
    p: *const Plane,
    v: Float64Array,
) -> Option<Float64Array> {
    if !p.is_null() {
        let vec = vector_create(v);
        let res = plane_project_0(p, vec);
        vector_release(vec);
        match res {
            Some(vec_res) => Some(vector_convert_to_array64_from_64(&vec_res)),
            _ => None,
        }
    } else {
        None
    }
}

/// sort points
#[wasm_bindgen]
pub fn plane_sort_points_0_i(
    p: *const Plane,
    va: *const RefCell<Vec<Vec<f64>>>,
) -> *const BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>> {
    if !p.is_null() && !va.is_null() {
        unsafe {
            let sorted_indices = (*p).sort_points_0(&(*va).borrow());
            float_indices_create_0(sorted_indices)
        }
    } else {
        std::ptr::null()
    }
}

/// sort points
#[wasm_bindgen]
pub fn plane_sort_points_0(
    p: *const Plane,
    point_container: JsValue,
) -> *const BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>> {
    if !p.is_null() {
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
                            None => break,
                        }
                    }
                    vector_array_add_0(vec_array, &vec);
                } else {
                    vector_array_add_0(vec_array, &Vec::new());
                }
            }
            let result = plane_sort_points_0_i(p, vec_array);
            vector_array_release(vec_array);
            result
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}

/// sort points
#[wasm_bindgen]
pub fn plane_sort_points_1_i(
    p: *const Plane,
    va: *const RefCell<Vec<Vec<f64>>>,
) -> *const BTreeMap<Distance, Rc<RefCell<Vec<usize>>>> {
    if !p.is_null() && !va.is_null() {
        unsafe {
            let sorted_indices = (*p).sort_points_1(&(*va).borrow());
            distance_indices_create_0(sorted_indices)
        }
    } else {
        std::ptr::null()
    }
}

/// sort points
#[wasm_bindgen]
pub fn plane_sort_points_1(
    p: *const Plane,
    point_container: JsValue,
) -> *const BTreeMap<Distance, Rc<RefCell<Vec<usize>>>> {
    if !p.is_null() {
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
                            None => break,
                        }
                    }
                    vector_array_add_0(vec_array, &vec);
                } else {
                    vector_array_add_0(vec_array, &Vec::new());
                }
            }
            let result = plane_sort_points_1_i(p, vec_array);
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
