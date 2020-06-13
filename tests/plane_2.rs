
use glrs::Plane;
#[cfg(test)]
#[test]
fn project_0() {
    let plane = glrs::Plane::create(
        &vec!(1f64, 1f64, 1f64),
        &vec!(0f64, 0f64, 0f64)).unwrap();
    let pt = plane.project(&vec!(2.5, 2.5, 2.5)).unwrap();

    assert!(pt[0].abs() < 0.00001);
    assert!(pt[1].abs() < 0.00001);
    assert!(pt[2].abs() < 0.00001);
} 

// vi: se ts=4 sw=4 et: 
