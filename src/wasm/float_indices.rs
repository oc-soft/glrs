use js_sys::Uint32Array;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// create instance
pub fn float_indices_create_0(
    fi: BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>>,
) -> *const BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>> {
    Rc::into_raw(Rc::new(fi))
}

/// increment reference count
#[wasm_bindgen]
pub fn float_indices_retain(
    fi: *const BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>>,
) -> usize {
    if !fi.is_null() {
        unsafe {
            let fi_ref_0 = Rc::from_raw(fi);
            let fi_ref_1 = fi_ref_0.clone();
            let result = Rc::strong_count(&fi_ref_0);
            Rc::into_raw(fi_ref_0);
            Rc::into_raw(fi_ref_1);
            result
        }
    } else {
        0 as usize
    }
}

/// decrement reference count
#[wasm_bindgen]
pub fn float_indices_release(
    fi: *const BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>>,
) -> usize {
    if !fi.is_null() {
        unsafe {
            let fi_ref_0 = Rc::from_raw(fi);
            let mut result = Rc::strong_count(&fi_ref_0);
            result -= 1;
            result
        }
    } else {
        0 as usize
    }
}

/// get all registered float values
#[wasm_bindgen]
pub fn float_indices_get_float_keys(
    fi: *const BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>>,
) -> *const Vec<OrderedFloat<f64>> {
    if !fi.is_null() {
        unsafe {
            let keys_itr = (*fi).keys();

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
pub fn float_indices_get_indices(
    fi: *const BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>>,
    float_obj: *const OrderedFloat<f64>,
) -> Option<Uint32Array> {
    if !fi.is_null() && !float_obj.is_null() {
        unsafe {
            match (*fi).get(&*float_obj) {
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
