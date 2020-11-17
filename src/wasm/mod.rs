pub use distance::*;
pub use distance_indices::*;
pub use distances::*;
pub use float::*;
pub use float_indices::*;
pub use float_vec::*;
pub use geom::*;
pub use matrix::*;
pub use plane::*;
pub use segment::*;
pub use vector::*;
pub use vector_array::*;
use wasm_bindgen::prelude::*;

mod distance;
mod distance_indices;
mod distances;
mod float;
mod float_indices;
mod float_vec;
mod geom;
mod matrix;
mod plane;
mod segment;
mod vector;
mod vector_array;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// vi: se ts=4 sw=4 et:
