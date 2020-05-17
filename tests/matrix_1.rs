use glrs::Matrix;

#[cfg(test)]
#[test]
fn minor_0() {
    let comp = vec![3.0, 2.0, 5.0, 4.0, 3.0, 3.6, 101.0, 12.0, 4.5];

    let mat0 = Matrix::new_with_source_component(&comp);
    let comp = vec![3.0, 5.0, 101.0, 4.5];
    let mat1 = Matrix::new_with_source_component(&comp);

    assert!(mat0.minor(1, 1).unwrap() == mat1)
}
#[test]
fn determinant_0() {
    let comp = vec![4.0, 7.0, 2.0, 6.0];
    let mat0 = Matrix::new_with_source_component(&comp);

    assert!(mat0.determinant() == 4.0 * 6.0 - 7.0 * 2.0)
}

#[test]
fn cofactor_0() {
    let comp = vec![1.0, 4.0, 7.0, 3.0, 0.0, 5.0, -1.0, 9.0, 11.0];
    let mat0 = Matrix::new_with_source_component(&comp);
    assert!(mat0.get_cofactor(1, 2).unwrap() == -13.0)
}

#[test]
fn inverse_0() {
    let comp = vec![4.0, 7.0, 2.0, 6.0];
    let mat0 = Matrix::new_with_source_component(&comp);
    let comp = vec![0.6, -0.7, -0.2, 0.4];
    let mat1 = Matrix::new_with_source_component(&comp);

    assert!(mat0.inverse().unwrap() == mat1)
}

#[test]
fn inverse_1() {
    let comp = vec![7.0, 2.0, 1.0, 0.0, 3.0, -1.0, -3.0, 4.0, -2.0];
    let mat0 = Matrix::new_with_source_component(&comp);
    let comp = vec![-2.0, 8.0, -5.0, 3.0, -11.0, 7.0, 9.0, -34.0, 21.0];
    let mat1 = Matrix::new_with_source_component(&comp);

    assert!(mat0.inverse().unwrap() == mat1)
}

#[test]
fn inverse_2() {
    let comp = vec![7.0, 2.0, 1.0, 0.0, 0.0, -0.0, -3.0, 4.0, -2.0];
    let mat0 = Matrix::new_with_source_component(&comp);

    assert!(mat0.inverse().is_none())
}
