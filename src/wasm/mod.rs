use wasm_bindgen::prelude::*;
pub use vector::*;
pub use vector_array::*;
pub use matrix::*;
pub use plane::*;
pub use distances::*;
pub use distance_indices::*;
pub use distance::*;

mod vector;
mod vector_array;
mod matrix;
mod plane;
mod distances;
mod distance_indices;
mod distance;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// vi: se ts=4 sw=4 et:
