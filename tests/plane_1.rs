use glrs::Distance;
use glrs::Plane;
use std::collections::BTreeMap;

#[cfg(test)]
#[test]
fn distance_comp_0() {
    let dist_0 = Distance::new(&-1.0);
    let dist_1 = Distance::new(&1.0);

    assert!(dist_0 == dist_1)
}

#[test]
fn distance_key_0() {
    let mut map = BTreeMap::new();

    map.insert(Distance::new(&-1.0), "Hello");
    map.insert(Distance::new(&2.0), "World");

    assert!(map.get(&Distance::new(&1.0)).unwrap() == &"Hello")
}

#[test]
fn sort_0() {
    let p = Plane::create(&vec![0.0, 0.0, 1.0], &vec![0.0, 0.0, 0.0]).unwrap();
    let mut points: Vec<Vec<f64>> = Vec::new();

    points.push(vec![2.0, 3.0, 2.0]);
    points.push(vec![0.0, 0.0, -0.3]);
    points.push(vec![1.0, 3.0, 5.0]);
    points.push(vec![-3.0, 4.0, -2.0]);
    points.push(vec![1.0, 1.0, 2.0]);

    let sorted_indices = p.sort_points_1(&points);

    {
        let values = sorted_indices.values().next().unwrap();
        let indices = values.borrow_mut();
        println!("{:?}", indices);
        assert!(indices[0] == 1)
    }
}

// vi: se ts=4 sw=4 et:
