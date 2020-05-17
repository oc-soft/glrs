
use crate::MatrixError;

/// create vector for gl
fn create_vec(v: Vec<f64>) -> Result<Vec<f64>, MatrixError> {
    if 0 < v.len() && v.len() < 5 {
        let res = Vec<f64>::with_capacity(dim);
        for i in 0..v.len() {
            res.push(v[i]);
        }
        Ok(res)
    } else {
        Err(MatrixError)
    }
}




// vi: se ts=4 sw=4 et:
