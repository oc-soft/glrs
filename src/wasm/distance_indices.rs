use crate::Distance;
use js_sys::Uint32Array;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// create instance
pub fn distance_indices_create_0(
    di: BTreeMap<Distance, Rc<RefCell<Vec<usize>>>>,
) -> *const BTreeMap<Distance, Rc<RefCell<Vec<usize>>>> {
    Rc::into_raw(Rc::new(di))
}

/// increment reference count
#[wasm_bindgen]
pub fn distance_indices_retain(
    di: *const BTreeMap<Distance, Rc<RefCell<Vec<usize>>>>,
) -> usize {
    if !di.is_null() {
        unsafe {
            let di_ref_0 = Rc::from_raw(di);
            let di_ref_1 = di_ref_0.clone();
            let result = Rc::strong_count(&di_ref_0);
            Rc::into_raw(di_ref_0);
            Rc::into_raw(di_ref_1);
            result
        }
    } else {
        0 as usize
    }
}

/// decrement reference count
#[wasm_bindgen]
pub fn distance_indices_release(
    di: *const BTreeMap<Distance, Rc<RefCell<Vec<usize>>>>,
) -> usize {
    if !di.is_null() {
        unsafe {
            let di_ref_0 = Rc::from_raw(di);
            let mut result = Rc::strong_count(&di_ref_0);
            result -= 1;
            result
        }
    } else {
        0 as usize
    }
}

/// get all registered distances
#[wasm_bindgen]
pub fn distance_indices_get_distances(
    di: *const BTreeMap<Distance, Rc<RefCell<Vec<usize>>>>,
) -> *const Vec<Distance> {
    if !di.is_null() {
        unsafe {
            let keys_itr = (*di).keys();

            let mut keys = Vec::new();
            for key in keys_itr {
                keys.push(*key);
            }
            Rc::into_raw(Rc::new(keys))
        }
    } else {
        std::ptr::null()
    }
}

/// get indices
#[wasm_bindgen]
pub fn distance_indices_get_indices(
    di: *const BTreeMap<Distance, Rc<RefCell<Vec<usize>>>>,
    distance: *const Distance,
) -> Option<Uint32Array> {
    if !di.is_null() && !distance.is_null() {
        unsafe {
            match (*di).get(&*distance) {
                Some(vec) => {
                    let indices = Uint32Array::new_with_length(
                        vec.borrow().len() as u32,
                    );
                    for i in 0..vec.borrow().len() {
                        indices.set_index(i as u32, vec.borrow()[i] as u32);
                    }
                    Some(indices)
                }
                _ => None,
            }
        }
    } else {
        None
    }
}

// vi: se ts=4 sw=4 et:
