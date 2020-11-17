use glrs::Plane;

#[cfg(test)]
#[test]
fn instantiate_0() {
    assert!(Plane::create(&Vec::new(), &vec!(2.0, 3.0)).is_err())
}

#[test]
fn instantiate_1() {
    assert!(Plane::create(&vec!(1.0, 3.0), &vec!(2.0, 3.0)).is_ok())
}

#[test]
fn instantiage_2() {
    let instance =
        Plane::create_with_2d(&vec![0.0, 1.0, 2.0], &vec![1.0, 0.0, 3.0, 4.0])
            .unwrap();
    let diff =
        instance.distance(&vec![0.0, 0.0, 0.0]).unwrap().abs() - 2f64.sqrt();
    assert!(diff < 0.00001)
}

#[test]
fn distance_0() {
    let p = Plane::create(&vec![0.0, 3.0], &vec![0.0, 3.0]).unwrap();

    assert!(p.distance(&vec!(0.0, 2.0)).unwrap() == -1.0)
}

#[test]
fn distance_1() {
    let p = Plane::create(&vec![3.0, 3.0], &vec![0.0, 0.0]).unwrap();
    let dis = p.distance(&vec![1.0, 1.0]).unwrap();
    assert!(dis == 2.0_f64.sqrt())
}

#[test]
fn distance_2() {
    let p = Plane::create(&vec![0.0, 0.0, 1.0], &vec![0.0, 0.0, 0.0]).unwrap();
    let dis = p.distance(&vec![0.0, 0.0, 1.0]).unwrap();
    assert!(dis == 1.0_f64);
}

// vi: se ts=4 sw=4 et:
