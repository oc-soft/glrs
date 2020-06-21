use std::cmp::min;
use super::geom::GeomError;
use super::geom;
use super::Matrix;

#[derive(Debug)]
pub struct Segment {
    
    /// direction vector for segment
    d: Vec<f64>,

    /// the point on line
    c: Vec<f64>,

    /// parameter range
    t: [f64; 2]
}


impl Segment {
    /// constructor
    pub fn create_0(d: &Vec<f64>, c: &Vec<f64>, t: &[f64;2])
        -> Result<Segment, GeomError> {
        if d.len() > 1 &&  c.len() > 1 && t[0] != t[1] {
            let dim = min(d.len(), c.len());
            let mut d_0 = Vec::with_capacity(dim);
            let mut c_0 = Vec::with_capacity(dim);
            for i in 0 .. dim {
                d_0.push(d[i]);
                c_0.push(c[i]);
            }
            let mut d_len = geom::length(&d_0).unwrap();
            if d_len > 0.0 {
                let mut t_0 = t.clone();
                for elm in &mut t_0 {
                    *elm *= d_len; 
                }
                if t_0[0] > t_0[1] {
                    d_len *= -1.0;
                    t_0[0] = t[1];
                    t_0[1] = t[0];
                }
                let t_diff = t_0[1] - t_0[0];
                geom::scale_0(1.0 / d_len, &mut d_0);

                if d_0[dim - 1] != 0.0 || d_0[dim - 1] != -0.0 {
                    // the scale for normalize c into plane which is 
                    // last coordinate is zero
                    let c_scale = - c_0[dim - 1] / d_0[dim - 1]; 
                    c_0 = geom::plus(
                        &geom::scale(c_scale, &d_0), &c_0).unwrap();
                    t_0[0] = - c_scale;
                    t_0[1] = t_0[0] + t_diff;
                    Ok(Segment {
                        d: d_0,
                        c: c_0,
                        t: t_0 
                    })
                } else {

                    if dim > 1 {
                        if d_0[dim - 2]  != 0.0 || d_0[dim - 2] != -0.0 {
                            // the scale for normalize c into plane which is 
                            // last coordinate is zero
                            let c_scale = - c_0[dim - 2] / d_0[dim - 2]; 
                            let c_0 = geom::plus(
                                &geom::scale(c_scale, &d_0), &c_0).unwrap();
                            t_0[0] = - c_scale;
                            t_0[1] = t_0[0] + t_diff;
                            Ok(Segment {
                                d: d_0,
                                c: c_0,
                                t: t_0 
                            })
                        } else {
                            Err(GeomError)
                        }
                    } else {
                        Err(GeomError)
                    }
                }
            } else {
                Err(GeomError)
            }
        } else {
            Err(GeomError)
        }
    }


    /// constructor
    pub fn create_1(p1: &Vec<f64>, p2: &Vec<f64>)
        -> Result<Segment, GeomError> {
        if p1.len() > 1 &&  p2.len() > 1 {
            let d = geom::minus(p2, p1).unwrap();   
            let d_len = geom::length(&d).unwrap();
            if d_len > 0.0 {
                Self::create_0(&d, &p1, &[0.0, 1.0])
            } else {
                Err(GeomError)
            }
        } else {
            Err(GeomError)
        }
    }
}

impl Segment {

    /// p1
    pub fn p1(&self) -> Vec<f64> {
        geom::plus(&geom::scale(self.t[0], &self.d), &self.c).unwrap()
    }

    /// p1
    pub fn p2(&self) -> Vec<f64> {
        geom::plus(&geom::scale(self.t[1], &self.d), &self.c).unwrap()
    }


    /// dimension
    pub fn dim(&self) -> usize {
        self.d.len()
    }
    
    /// direction
    pub fn direction(&self) -> &Vec<f64> {
        &self.d
    }

    /// point on line 
    pub fn point_on_line(&self) -> &Vec<f64> {
        &self.c
    }


    /// paramter range
    pub fn t(&self) -> &[f64;2] {
        &self.t
    }

    /// get point at t
    pub fn point_on_t(&self, t:f64) -> Vec<f64> {
        geom::plus(&geom::scale(t, &self.d), &self.c).unwrap()
    }

}

impl Segment {

    /// calculate cross point parameter as line 2d
    pub fn cross_point_parameter_2d_0(&self, other: &Self, tolerance:f64)
        -> Option<[f64;2]> {

        let d_1 = vec!(self.direction()[0], self.direction()[1]);
        let d_2 = vec!(other.direction()[0], other.direction()[1]);

        let d_prod = geom::dot_product(&d_1, &d_2).unwrap(); 
        if (1.0 - d_prod).abs() > tolerance {
            let d_mat = Matrix::new_with_source_component_row_order(
                &vec!(- d_1[1], d_1[0],
                    - d_2[1], d_2[0]));
            let dc_vec = vec!(d_1[0] * self.c[1] - d_1[1] * self.c[0],
                d_2[0] * other.c[1] - d_2[1] * other.c[0]);
            match d_mat.inverse() {
                Some(res_mat) => {
                    let res_vec = res_mat.apply_r(&dc_vec).unwrap();

                    let mut acc_idx;
                    if d_1[0].abs() > tolerance {
                        acc_idx = 0;
                    } else {
                        acc_idx = 1;
                    }
                    let t_1 = (res_vec[acc_idx] - self.c[acc_idx])
                        / d_1[acc_idx];

                    if d_2[0].abs() > tolerance {
                        acc_idx = 0;
                    } else {
                        acc_idx = 1;
                    }
                    let t_2 = (res_vec[acc_idx] - other.c[acc_idx])
                        / d_2[acc_idx];
                    Some([t_1, t_2]) 
                },
                _ => None
            }
            
        } else {
            None
        }
    }
    /// calculate cross point parameter as line 2d
    pub fn cross_point_parameter_2d(&self, other: &Self)
        -> Option<[f64;2]> {
        self.cross_point_parameter_2d_0(other, 0.0)
    }

    /// calculate cross point parameter as line 2d
    /// you will get none if cross point paremeter is out of segment range.
    pub fn cross_point_2d_parameter_exact_0(
        &self, other: &Self, tolerance: f64)
        -> Option<[f64;2]> {
        match self.cross_point_parameter_2d_0(other, tolerance) {
            Some(param) => {
                if self.t[0] <= param[0] && param[0] <= self.t[1] {
                    if other.t[0] <= param[1] && param[1] <= other.t[1] {
                        Some(param)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => None
        }
    }

    /// calculate cross point parameter as line 2d
    /// you will get none if cross point paremeter is out of segment range.
    pub fn cross_point_2d_parameter_exact(
        &self, other: &Self)
        -> Option<[f64;2]> {
        self.cross_point_2d_parameter_exact_0(other, 0.0)
    }

}

impl Clone for Segment {

    /// implement clone
    fn clone(&self) -> Self {

        let d = Vec::with_capacity(self.d.len());
        let c = Vec::with_capacity(self.c.len());
        Self {
            d: d,
            c: c,
            t: self.t
        }
    }
}


// vi: se ts=4 sw=4 et:
