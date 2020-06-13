use std::cmp::Ord;
use std::cmp::Ordering;

/// distance from plan
#[derive(Copy, Clone)]
pub struct Distance {
    /// distance
    distance: f64,
}

/// distance
impl Distance {
    /// instanciate
    pub fn new(dis: &f64) -> Self {
        Distance { distance: *dis }
    }

    /// get distance
    pub fn get_distance(&self) -> f64 {
        self.distance
    }

    /// get absolute distance
    pub fn get_abs_distance(&self) -> f64 {
        self.distance.abs()
    }
}

/// implement Ord
impl Ord for Distance {
    /// compare
    fn cmp(&self, other: &Self) -> Ordering {
        let diff = self.get_abs_distance() - other.get_abs_distance();
        if diff < 0.0 {
            Ordering::Less
        } else if diff > 0.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

/// implement PartialOrd
impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// implement PartialEq
impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Distance {}

// vi: se ts=4 sw=4 et:
