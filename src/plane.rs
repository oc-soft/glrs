use crate::geom;
use crate::Distance;
use geom::GeomError;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::BTreeMap;
use std::rc::Rc;

/// I represent plane with normal vector and the point on the plane
pub struct Plane {
    /// nomal vector for plane
    n: Vec<f64>,
    /// the point on the plane
    c: Vec<f64>,
}

impl Plane {
    /// create plane
    pub fn create(n: &Vec<f64>, c: &Vec<f64>) -> Result<Plane, GeomError> {
        if n.len() > 0 && c.len() > 0 {
            let dim = min(n.len(), c.len());

            let mut n0 = Vec::with_capacity(dim);
            let mut c0 = Vec::with_capacity(dim);
            for i in 0..dim {
                n0.push(n[i]);
                c0.push(c[i]);
            }
            let n_len = geom::length(&n0).unwrap();
            if n_len > 0.0 {
                geom::scale_0(1.0 / n_len, &mut n0);
                Ok(Plane { n: n0, c: c0 })
            } else {
                Err(GeomError)
            }
        } else {
            Err(GeomError)
        }
    }

    /// get dimension
    pub fn get_dimension(&self) -> usize {
        self.n.len()
    }

    /// calculate distance
    pub fn distance(&self, p: &Vec<f64>) -> Result<f64, GeomError> {
        if p.len() >= self.get_dimension() {
            let result =
                geom::dot_product(&self.n, &geom::minus(&p, &self.c).unwrap())
                    .unwrap();
            Ok(result)
        } else {
            Err(GeomError)
        }
    }

    /// sort point from this plane
    pub fn sort_points(
        &self,
        points: &Vec<Vec<f64>>,
    ) -> BTreeMap<Distance, Rc<RefCell<Vec<usize>>>> {
        let mut result: BTreeMap<Distance, Rc<RefCell<Vec<usize>>>> =
            BTreeMap::new();

        for i in 0..points.len() {
            let pt = &points[i];
            match self.distance(&pt) {
                Ok(distance) => match result.get(&Distance::new(&distance)) {
                    Some(vec) => vec.borrow_mut().push(i),
                    None => {
                        let mut vec = Vec::new();
                        vec.push(i);
                        result.insert(
                            Distance::new(&distance),
                            Rc::new(RefCell::new(vec)),
                        );
                    }
                },
                Err(_) => (),
            }
        }
        result
    }
}

// vi: se ts=4 sw=4 et:
