use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use js_sys::Float64Array;
use js_sys::Float32Array;
use super::vector::*;


/// create vectors
#[wasm_bindgen]
pub fn vector_array_create() -> *const RefCell<Vec<Vec<f64>>> {
    let vec: Vec<Vec<f64>> = Vec::new();
    let vec_ref = Rc::new(RefCell::new(vec));
    Rc::into_raw(vec_ref)
}

/// create vectors
pub fn vector_array_create_1(vec_array: &Vec<Vec<f64>>)
    -> *const RefCell<Vec<Vec<f64>>> {
    let mut result = vector_array_create();

    let mut state = true;
    for i in 0..vec_array.len() { 
        state = vector_array_add_00(result, &vec_array[i]);
        if !state {
            break;
        }
    }
    if !state {
        vector_array_release(result);
        result = std::ptr::null();
    }
    result
}


/// convet vector array from java script array
#[wasm_bindgen]
pub fn vector_array_from_js_array(array: js_sys::Array) 
    -> *const RefCell<Vec<Vec<f64>>> {
    let mut vec_array = Vec::new();
    let mut state;
    state = true;
    for i in 0..array.length() {
        let item = array.get(i);
        state = js_sys::Array::is_array(&item);
        if state {
            let array_1 = js_sys::Array::from(&item); 
            let mut vec = Vec::new();
            for j in 0..array_1.length() {
                let elem = array_1.get(j);
                 if let Some(val) = elem.as_f64() {
                    vec.push(val);
                } else {
                    state = false; 
                } 
                if !state {
                    break;
                }
            }
            if state {
                vec_array.push(vec);
            }
        }
        if !state {
            break;
        }
    }
    if state {
        vector_array_create_1(&vec_array)
    } else {
        std::ptr::null()
    }
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

/// get a element as Float64Array
pub fn vector_array_get_element_as_array64(
    v: *const RefCell<Vec<Vec<f64>>>,
    i: usize) -> Option<Float64Array> {
    if std::ptr::null() != v {
        if i < vector_array_size(v).unwrap() {
            unsafe {
                Some(vector_convert_to_array64_from_64(&(*v).borrow()[i]))
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// get a element as Float64Array
pub fn vector_array_get_element_as_array32(
    v: *const RefCell<Vec<Vec<f64>>>,
    i: usize) -> Option<Float32Array> {
    if std::ptr::null() != v {
        if i < vector_array_size(v).unwrap() {
            unsafe {
                Some(vector_convert_to_array32_from_64(&(*v).borrow()[i]))
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub fn vector_array_add_00(
    va: *const RefCell<Vec<Vec<f64>>>,
    v: &Vec<f64>,
) -> bool {
    if std::ptr::null() != va {
        let vec_size_ref: Option<usize>;
        if let Some(dim) = vector_array_dimension(va) {
            if dim <= v.len() {
                vec_size_ref = Some(dim);
            } else {
                vec_size_ref = None;
            }
        } else {
            vec_size_ref = Some(v.len()) 
        }
        if let Some(vec_size) = vec_size_ref {
            let mut vec = Vec::with_capacity(vec_size);
            for i in 0..vec_size {
                vec.push(v[i]);
            }
            unsafe {
                (*va).borrow_mut().push(vec);
            }
            true
        } else {
            false
        }
    } else {
        false
    }
}

/// add vector
#[wasm_bindgen]
pub fn vector_array_add_0(
    va: *const RefCell<Vec<Vec<f64>>>,
    v: *const Vec<f64>,
) -> bool {
    if std::ptr::null() != va && std::ptr::null() != v {
        unsafe {
            vector_array_add_00(va, &*v)
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

/// get every components as javasript array 
#[wasm_bindgen]
pub fn vector_array_get_components_as_array_array64(
    va: *const RefCell<Vec<Vec<f64>>>) -> Option<js_sys::Array> {
    if va != std::ptr::null() {
        let size = vector_array_size(va).unwrap();
        let array = js_sys::Array::new_with_length(size as u32);
        for i in 0..size {
            let array64 = vector_array_get_element_as_array64(va, i).unwrap();
            array.set(i as u32, JsValue::from(array64));
        }
        Some(array)
    } else {
        None
    }
}
 

/// get every components as javasript array 
#[wasm_bindgen]
pub fn vector_array_get_components_as_array_array32(
    va: *const RefCell<Vec<Vec<f64>>>) -> Option<js_sys::Array> {
    if va != std::ptr::null() {
        let size = vector_array_size(va).unwrap();
        let array = js_sys::Array::new_with_length(size as u32);
        for i in 0..size {
            let array64 = vector_array_get_element_as_array32(va, i).unwrap();
            array.set(
                i as u32, 
                JsValue::from(array64));
        }
        Some(array)
    } else {
        None
    }
}




// vi: se ts=4 sw=4 et:
