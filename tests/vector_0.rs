use glrs::Matrix;

#[cfg(test)]
#[test]
fn vec_0() {
    let comp = vec![2.0, 0.0, 0.0, 4.0];
    let mat0 = Matrix::new_with_source_component(&comp);
    let vec = vec![5.0, 3.0];

    let res = mat0.apply_r(vec).unwrap();

    assert!(res == vec![10.0, 12.0])
}
