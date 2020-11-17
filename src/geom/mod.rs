pub(crate) use error::GeomError;

pub use d2::D2;

mod d2;
mod error;

/// calculat length of vector
pub fn length(v: &[f64]) -> Result<f64, GeomError> {
    if !v.is_empty() {
        let mut len = 0.0;
        for elem in v {
            len += elem.powi(2);
        }
        Ok(len.sqrt())
    } else {
        Err(GeomError)
    }
}

/// calculate dot product for vector
pub fn dot_product(v1: &[f64], v2: &[f64]) -> Result<f64, GeomError> {
    if !v1.is_empty() && !v2.is_empty() {
        let mut result = 0.0;
        let size = std::cmp::min(v1.len(), v2.len());
        for i in 0..size {
            result += v1[i] * v2[i];
        }
        Ok(result)
    } else {
        Err(GeomError)
    }
}

/// apply scale each component
#[allow(clippy::needless_range_loop)]
pub fn scale_0(s: f64, v: &mut [f64]) -> &[f64] {
    for i in 0..v.len() {
        v[i] *= s;
    }
    v
}

/// apply scale each component
pub fn scale(s: f64, v: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(v.len());
    for elem in v {
        result.push(s * elem);
    }
    result
}

/// add each component
pub fn plus(v1: &[f64], v2: &[f64]) -> Result<Vec<f64>, GeomError> {
    if !v1.is_empty() && !v2.is_empty() {
        let size = std::cmp::min(v1.len(), v2.len());
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(v1[i] + v2[i]);
        }
        Ok(result)
    } else {
        Err(GeomError)
    }
}
/// minus  each component
pub fn minus(v1: &[f64], v2: &[f64]) -> Result<Vec<f64>, GeomError> {
    if !v1.is_empty() && !v2.is_empty() {
        let size = std::cmp::min(v1.len(), v2.len());
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(v1[i] - v2[i]);
        }
        Ok(result)
    } else {
        Err(GeomError)
    }
}


// vi: se ts=4 sw=4 et:
