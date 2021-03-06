use crate::geom;
use crate::Distance;
use geom::GeomError;
use ordered_float::OrderedFloat;
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
    pub fn create(n: &[f64], c: &[f64]) -> Result<Plane, GeomError> {
        if !n.is_empty() && !c.is_empty() {
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
    /// create line
    pub fn create_with_2d(p1: &[f64], p2: &[f64]) -> Result<Plane, GeomError> {
        if p1.len() > 1 && p2.len() > 1 {
            match geom::minus(p2, p1) {
                Ok(v) => {
                    let v2d = vec![v[0], v[1]];
                    Self::create(&[-v2d[1], v2d[0]], p1)
                }
                _ => Err(GeomError),
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
    pub fn distance(&self, p: &[f64]) -> Result<f64, GeomError> {
        if p.len() >= self.get_dimension() {
            let result =
                geom::dot_product(&self.n, &geom::minus(&p, &self.c).unwrap())
                    .unwrap();
            Ok(result)
        } else {
            Err(GeomError)
        }
    }

    /// project a point to this plane
    pub fn project(&self, p: &[f64]) -> Result<Vec<f64>, GeomError> {
        match self.distance(p) {
            Ok(dis) => {
                let translate = geom::scale(-dis, &self.n);
                let result = geom::plus(&translate, &p).unwrap();
                Ok(result)
            }
            Err(err) => Err(err),
        }
    }

    /// sort point from this plane
    pub fn sort_points_1(
        &self,
        points: &[Vec<f64>],
    ) -> BTreeMap<Distance, Rc<RefCell<Vec<usize>>>> {
        let mut result: BTreeMap<Distance, Rc<RefCell<Vec<usize>>>> =
            BTreeMap::new();

        for (i, pt) in points.iter().enumerate() {
            if let Ok(distance) = self.distance(&pt) {
                match result.get(&Distance::new(&distance)) {
                    Some(vec) => vec.borrow_mut().push(i),
                    None => {
                        let mut vec = Vec::new();
                        vec.push(i);
                        result.insert(
                            Distance::new(&distance),
                            Rc::new(RefCell::new(vec)),
                        );
                    }
                }
            }
        }
        result
    }

    /// sort point from this plane
    pub fn sort_points_0(
        &self,
        points: &[Vec<f64>],
    ) -> BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>> {
        let mut result: BTreeMap<OrderedFloat<f64>, Rc<RefCell<Vec<usize>>>> =
            BTreeMap::new();

        for (i, pt) in points.iter().enumerate() {
            if let Ok(distance) = self.distance(&pt) {
                match result.get(&OrderedFloat(distance)) {
                    Some(vec) => vec.borrow_mut().push(i),
                    None => {
                        let mut vec = Vec::new();
                        vec.push(i);
                        result.insert(
                            OrderedFloat(distance),
                            Rc::new(RefCell::new(vec)),
                        );
                    }
                }
            }
        }
        result
    }
}

// vi: se ts=4 sw=4 et:
