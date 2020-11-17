use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use crate::*;
use js_sys::Float64Array;
use super::vector_array::*;
use super::vector::*;



/// move points toward cross line direction with offset
#[wasm_bindgen]
pub fn geom_d2_offset_points_00(
    offset: f64,
    p1: *const Vec<f64>,
    p2: *const Vec<f64>,
    tolerance: f64) -> *const RefCell<Vec<Vec<f64>>> {

    if !p1.is_null() && !p2.is_null() {
        unsafe {
            let pts_res = geom::D2::offset_points_0(offset,
                &*p1, &*p2, tolerance);
            if let Ok(pts) = pts_res {
                let result = vector_array_create(); 
                for pt in &pts {
                    vector_array_add_00(result, pt);
                }
                result
            } else {
                std::ptr::null()
            }
        }
    } else {
        std::ptr::null()
    }
}

/// move points toward cross line direction with offset
#[wasm_bindgen]
pub fn geom_d2_offset_points_01(
    offset: f64,
    p1: Float64Array,
    p2: Float64Array,
    tolerance: f64) -> *const RefCell<Vec<Vec<f64>>> {

    let vec_1 = vector_create(p1);
    let vec_2 = vector_create(p2);

    let result = geom_d2_offset_points_00(offset, vec_1, vec_2, tolerance);

    vector_release(vec_1);
    vector_release(vec_2);
    result
}

/// move points toward cross line direction with offset
#[wasm_bindgen]
pub fn geom_d2_offset_points_02(
    offset: f64,
    p1: Float64Array,
    p2: Float64Array,
    tolerance: f64) -> Option<js_sys::Array> {

    let offset_pts = geom_d2_offset_points_01(offset, p1, p2, tolerance);
    if !offset_pts.is_null() {
        let offset_pts_js = vector_array_get_components_as_array_array64(
            offset_pts);
        vector_array_release(offset_pts);
        offset_pts_js
    } else {
        None
    }
}


/// move points toward cross line direction with offset
#[wasm_bindgen]
pub fn geom_d2_offset_points_10(
    offset: f64,
    p1: *const Vec<f64>,
    p2: *const Vec<f64>) -> *const RefCell<Vec<Vec<f64>>> {

    if !p1.is_null() && !p2.is_null() {
        unsafe {
            let pts_res = geom::D2::offset_points(
                offset, &*p1, &*p2);
            if let Ok(pts) = pts_res {
                let result = vector_array_create(); 
                for pt in &pts {
                    let vec = vector_create_from_vec_ref(pt);
                    vector_array_add_0(result, vec);
                    vector_release(vec);
                }
                result
            } else {
                std::ptr::null()
            }
        }
    } else {
        std::ptr::null()
    }
}

/// move points toward cross line direction with offset
#[wasm_bindgen]
pub fn geom_d2_offset_points_11(
    offset: f64,
    p1: Float64Array,
    p2: Float64Array) -> *const RefCell<Vec<Vec<f64>>> {

    let vec_1 = vector_create(p1);
    let vec_2 = vector_create(p2);

    let result = geom_d2_offset_points_10(offset, vec_1, vec_2);

    vector_release(vec_1);
    vector_release(vec_2);
    result
}

/// move points toward cross line direction with offset
#[wasm_bindgen]
pub fn geom_d2_offset_points_12(
    offset: f64,
    p1: Float64Array,
    p2: Float64Array) -> Option<js_sys::Array> {

    let offset_pts = geom_d2_offset_points_11(offset, p1, p2);
    if !offset_pts.is_null() {
        let result = vector_array_get_components_as_array_array64(
            offset_pts);
        vector_array_release(offset_pts);
        result
    } else {
        None
    }
}

/// move all of points with offset displacement
#[wasm_bindgen]
pub fn geom_2d_offset_points_vec_00(
    offset: f64, 
    points: *const RefCell<Vec<Vec<f64>>>, 
    close: bool, tolerance:f64) -> *const RefCell<Vec<Vec<f64>>> {
    if !points.is_null() { 
        unsafe {
            let offset_pts_ref = geom::D2::offset_points_vec_0(
                offset, &(*points).borrow(), close, tolerance);
            if let Ok(offset_pts) = offset_pts_ref {
                vector_array_create_1(&offset_pts)
            } else {
                std::ptr::null()
            }
        }
    } else {
        std::ptr::null()
    }
}

/// move all of points with offset displacement
#[wasm_bindgen]
pub fn geom_2d_offset_points_vec_01(
    offset: f64, 
    points: js_sys::Array, 
    close: bool, tolerance:f64) -> *const RefCell<Vec<Vec<f64>>> {
    let points_vec_array  = vector_array_from_js_array(points);
    if !points_vec_array.is_null() {
        let result = geom_2d_offset_points_vec_00(
            offset, points_vec_array, close, tolerance);
        vector_array_release(points_vec_array);
        result
    } else {
        std::ptr::null()
    }
}

/// move all of points with offset displacement
#[wasm_bindgen]
pub fn geom_2d_offset_points_vec_02(
    offset: f64, 
    points: js_sys::Array, 
    close: bool, tolerance:f64) -> Option<js_sys::Array> {
    let offset_pts  = geom_2d_offset_points_vec_01(
        offset, points, close, tolerance);
    if !offset_pts.is_null() {
        let result = vector_array_get_components_as_array_array64(offset_pts);
        vector_array_release(offset_pts);
        result
    } else {
        None
    }
}


/// move all of points with offset displacement
#[wasm_bindgen]
pub fn geom_2d_offset_points_vec_10(
    offset: f64, 
    points: *const RefCell<Vec<Vec<f64>>>, 
    close: bool) -> *const RefCell<Vec<Vec<f64>>> {
    if !points.is_null() { 
        unsafe {
            let offset_pts_ref = geom::D2::offset_points_vec(
                offset, &(*points).borrow(), close);
            if let Ok(offset_pts) = offset_pts_ref {
                vector_array_create_1(&offset_pts)
            } else {
                std::ptr::null()
            }
        }
    } else {
        std::ptr::null()
    }
}

/// move all of points with offset displacement
#[wasm_bindgen]
pub fn geom_2d_offset_points_vec_11(
    offset: f64, 
    points: js_sys::Array, 
    close: bool) -> *const RefCell<Vec<Vec<f64>>> {
    let points_vec_array  = vector_array_from_js_array(points);
    if !points_vec_array.is_null() {
        let result = geom_2d_offset_points_vec_10(
            offset, points_vec_array, close);
        vector_array_release(points_vec_array);
        result
    } else {
        std::ptr::null()
    }
}

/// move all of points with offset displacement
#[wasm_bindgen]
pub fn geom_2d_offset_points_vec_12(
    offset: f64, 
    points: js_sys::Array, 
    close: bool) -> Option<js_sys::Array> {
    let offset_pts  = geom_2d_offset_points_vec_11(
        offset, points, close);
    if !offset_pts.is_null() {
        let result = vector_array_get_components_as_array_array64(offset_pts);
        vector_array_release(offset_pts);
        result
    } else {
        None
    }
}



// vi: se ts=4 sw=4 et:
