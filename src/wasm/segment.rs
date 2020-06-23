use std::rc::Rc;

use wasm_bindgen::prelude::*;
use js_sys::Float64Array;
use super::vector::*;


use crate::Segment;

/// construct segment
fn segment_create_i_0(
    seg: Segment) -> *const Segment {
    let seg_ref = Rc::new(seg);
    Rc::into_raw(seg_ref)
}

/// increment reference count
#[wasm_bindgen]
pub fn segment_retain(seg: *const Segment) -> usize {
    if std::ptr::null() != seg {
        unsafe {
            let seg_ref_0 = Rc::from_raw(seg);
            let seg_ref_1 = seg_ref_0.clone();
            let result = Rc::strong_count(&seg_ref_0);
            Rc::into_raw(seg_ref_0);
            Rc::into_raw(seg_ref_1);
            result
        }
    } else {
        0 as usize
    }
}


/// decrement reference count
#[wasm_bindgen]
pub fn segment_release(seg: *const Segment) -> usize {
    if std::ptr::null() != seg {
        unsafe {
            let seg_ref_0 = Rc::from_raw(seg);
            let mut result = Rc::strong_count(&seg_ref_0);
            result -= 1;
            result
        }
    } else {
        0 as usize
    }
}


/// create segment
#[wasm_bindgen]
pub fn segment_create_00(
    d: *const Vec<f64>, 
    c: *const Vec<f64>, 
    t0: f64,
    t1: f64) -> *const Segment {

    if d != std::ptr::null() && c != std::ptr::null()  {
        unsafe {
            let seg_ref = Segment::create_0(
                &*d, &*c, &[t0, t1]);
            if let Ok(seg) = seg_ref {
                segment_create_i_0(seg)
            } else {
                std::ptr::null()
            }
        }
    } else {
        std::ptr::null()
    }
}


/// create segment instance
#[wasm_bindgen]
pub fn segment_create_01(
    d: Float64Array, 
    c: Float64Array, 
    t0: f64,
    t1: f64) -> *const Segment {

    let d_vec = vector_create(d);
    let c_vec = vector_create(c);

    let result = segment_create_00(d_vec, c_vec, t0, t1);

    vector_release(d_vec);
    vector_release(c_vec);
    result
}

/// create segment
#[wasm_bindgen]
pub fn segment_create_10(
    p1: *const Vec<f64>, 
    p2: *const Vec<f64>) -> *const Segment {

    if p1 != std::ptr::null() &&  p2 != std::ptr::null() {
        unsafe {
            let seg_ref = Segment::create_1(&*p1, &*p2);
            if let Ok(seg) = seg_ref {
                segment_create_i_0(seg)
            } else {
                std::ptr::null()
            }
        }
    } else {
        std::ptr::null()
    }
}


/// create segment instance
#[wasm_bindgen]
pub fn segment_create_11(
    p1: Float64Array, 
    p2: Float64Array) -> *const Segment {

    let p1_vec = vector_create(p1);
    let p2_vec = vector_create(p2);

    let result = segment_create_10(p1_vec, p2_vec);

    vector_release(p1_vec);
    vector_release(p2_vec);
    result
}

/// p1 vector
#[wasm_bindgen]
pub fn segment_p1_0(seg: *const Segment) -> *const Vec<f64> {
    if seg != std::ptr::null() {
        unsafe {
            vector_create_from_vec((*seg).p1())
        }
    } else {
        std::ptr::null() 
    }
}


/// p1 vector
#[wasm_bindgen]
pub fn segment_p1(seg: *const Segment) -> Option<Float64Array> {
    if seg != std::ptr::null() {
        let vec = segment_p1_0(seg);
        let result = vector_get_components(vec); 
        vector_release(vec);
        result
    } else {
        None
    }
}

/// p2 vector
#[wasm_bindgen]
pub fn segment_p2_0(seg: *const Segment) -> *const Vec<f64> {
    if seg != std::ptr::null() {
        unsafe {
            vector_create_from_vec((*seg).p2())
        }
    } else {
        std::ptr::null() 
    }
}


/// p2 vector
#[wasm_bindgen]
pub fn segment_p2(seg: *const Segment) -> Option<Float64Array> {
    if seg != std::ptr::null() {
        let vec = segment_p2_0(seg);
        let result = vector_get_components(vec); 
        vector_release(vec);
        result
    } else {
        None
    }
}

/// direction
#[wasm_bindgen]
pub fn segment_direction_0(seg: *const Segment) -> *const Vec<f64> {
    if seg != std::ptr::null() {
        unsafe {
            vector_create_from_vec_ref((*seg).direction())
        }
    } else {
        std::ptr::null() 
    }
}


/// direction
#[wasm_bindgen]
pub fn segment_direction(seg: *const Segment) -> Option<Float64Array> {
    if seg != std::ptr::null() {
        let vec = segment_direction_0(seg);
        let result = vector_get_components(vec); 
        vector_release(vec);
        result
    } else {
        None
    }
}

/// point on line 
#[wasm_bindgen]
pub fn segment_point_on_t_0(seg: *const Segment, t: f64) -> *const Vec<f64> {
    if seg != std::ptr::null() {
        unsafe {
            vector_create_from_vec((*seg).point_on_t(t))
        }
    } else {
        std::ptr::null()
    }
}

/// point on line 
#[wasm_bindgen]
pub fn segment_point_on_t(seg: *const Segment, t: f64) -> Option<Float64Array> {
    if seg != std::ptr::null() {
        let vec = segment_point_on_t_0(seg, t);
        let result = vector_get_components(vec); 
        vector_release(vec);
        result
    } else {
        None
    }
}

/// parameter range
#[wasm_bindgen]
pub fn segment_get_parameter_range_0(seg: *const Segment) -> *const Vec<f64> {
    if seg != std::ptr::null() {
        unsafe {
            let range = (*seg).t();
            vector_create_from_vec(vec!(range[0], range[1])) 
        }
    } else {
        std::ptr::null()
    }
}

/// parameter range
#[wasm_bindgen]
pub fn segment_get_parameter_range(seg: *const Segment)
    -> Option<Float64Array> {
    if seg != std::ptr::null() {
        let vec = segment_get_parameter_range_0(seg);
        let result = vector_get_components(vec);
        vector_release(vec);
        result
    } else {
        None
    }
}

/// get point at t
#[wasm_bindgen]
pub fn segment_point_on_line_0(seg: *const Segment) -> *const Vec<f64> {
    if seg != std::ptr::null() {
        unsafe {
            vector_create_from_vec_ref((*seg).point_on_line())
        }
    } else {
        std::ptr::null() 
    }
}


/// get point at t
#[wasm_bindgen]
pub fn segment_point_on_line(seg: *const Segment) -> Option<Float64Array> {
    if seg != std::ptr::null() {
        let vec = segment_point_on_line_0(seg);
        let result = vector_get_components(vec);
        vector_release(vec);
        result
    } else {
        None 
    }
}

/// calculate cross point parameter as line 2d
#[wasm_bindgen]
pub fn segment_cross_point_parameter_2d_00(
    seg_1 : *const Segment, 
    seg_2 : *const Segment, 
    tolerance:f64)
    -> *const Vec<f64> {
    if seg_1 != std::ptr::null() && seg_2 != std::ptr::null() {
        let param_ref;
        unsafe {
            param_ref = (*seg_1).cross_point_parameter_2d_0(
                &*seg_2, tolerance);
        }
        if let Some(param) = param_ref {
            vector_create_from_vec(vec!(param[0], param[1]))
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
} 

/// calculate cross point parameter as line 2d
#[wasm_bindgen]
pub fn segment_cross_point_parameter_2d_01(
    seg_1 : *const Segment, 
    seg_2 : *const Segment, 
    tolerance:f64)
    -> Option<Float64Array> {
    let vec= segment_cross_point_parameter_2d_00(seg_1, seg_2, tolerance);

    if vec != std::ptr::null() {
        let result = vector_get_components(vec);
        vector_release(vec);
        result
    } else {
        None
    }
}

/// calculate cross point parameter as line 2d
#[wasm_bindgen]
pub fn segment_cross_point_parameter_2d_10(
    seg_1 : *const Segment, 
    seg_2 : *const Segment)
    -> *const Vec<f64> {
    if seg_1 != std::ptr::null() && seg_2 != std::ptr::null() {
        let param_ref;
        unsafe {
            param_ref = (*seg_1).cross_point_parameter_2d(&*seg_2);
        }
        if let Some(param) = param_ref {
            vector_create_from_vec(vec!(param[0], param[1]))
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
} 

/// calculate cross point parameter as line 2d
#[wasm_bindgen]
pub fn segment_cross_point_parameter_2d_11(
    seg_1 : *const Segment, 
    seg_2 : *const Segment)
    -> Option<Float64Array> {
    let vec= segment_cross_point_parameter_2d_10(seg_1, seg_2);

    if vec != std::ptr::null() {
        let result = vector_get_components(vec);
        vector_release(vec);
        result
    } else {
        None
    }
}

/// calculate cross point parameter as line 2d
/// you will get none if cross point paremeter is out of segment range.
#[wasm_bindgen]
pub fn segment_cross_point_parameter_2d_exact_00(
    seg_1: *const Segment, 
    seg_2: *const Segment,
    tolerance: f64)
    -> *const Vec<f64> {
    if seg_1 != std::ptr::null() && seg_2 != std::ptr::null() {
        let param_ref;
        unsafe {
            param_ref =
                (*seg_1).cross_point_parameter_2d_exact_0(&*seg_2, tolerance);
        }
        if let Some(param) = param_ref {
            vector_create_from_vec(vec!(param[0], param[1]))
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}

/// calculate cross point parameter as line 2d
/// you will get none if cross point paremeter is out of segment range.
#[wasm_bindgen]
pub fn segment_cross_point_parameter_2d_exact_01(
    seg_1: *const Segment, 
    seg_2: *const Segment,
    tolerance: f64)
    -> Option<Float64Array> {
    let vec = segment_cross_point_parameter_2d_exact_00(
        seg_1, seg_2, tolerance);
    if vec != std::ptr::null() {
        let result = vector_get_components(vec);
        vector_release(vec);
        result
    } else {
        None
    }
}
 
/// calculate cross point parameter as line 2d
/// you will get none if cross point paremeter is out of segment range.
#[wasm_bindgen]
pub fn segment_cross_point_parameter_2d_exact_10(
    seg_1: *const Segment,
    seg_2: *const Segment) -> *const Vec<f64> {

    if seg_1 != std::ptr::null() && seg_2 != std::ptr::null() {
        let param_ref;
        unsafe {
            param_ref = (*seg_1).cross_point_parameter_2d_exact(&*seg_2);
        }
        if let Some(param) = param_ref {
            vector_create_from_vec(vec!(param[0], param[1]))
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}
 
/// calculate cross point parameter as line 2d
/// you will get none if cross point paremeter is out of segment range.
#[wasm_bindgen]
pub fn segment_cross_point_parameter_2d_exact_11(
    seg_1: *const Segment,
    seg_2: *const Segment) -> Option<Float64Array> {

    let vec = segment_cross_point_parameter_2d_exact_10(seg_1, seg_2);
    if vec != std::ptr::null() {
        let result = vector_get_components(vec);
        vector_release(vec);
        result
    } else {
        None
    }
}
// vi: se ts=4 sw=4 et:

