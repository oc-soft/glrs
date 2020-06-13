use wasm_bindgen::prelude::*;
pub use vector::*;
pub use vector_array::*;
pub use matrix::*;
pub use plane::*;
pub use distances::*;
pub use distance_indices::*;
pub use distance::*;
pub use float_vec::*;
pub use float_indices::*;
pub use float::*;

mod vector;
mod vector_array;
mod matrix;
mod plane;
mod distances;
mod distance_indices;
mod distance;
mod float_vec;
mod float;
mod float_indices;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// vi: se ts=4 sw=4 et:
