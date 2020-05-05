

#[cfg(test)]
#[test]
fn create_instance() {
    let mut comp = vec![ 2.0 ];
    let mt = glrs_matrix::MatrixI::bind(&mut comp).unwrap();
    assert!(mt.col_count == 1)
}

#[test]
fn add_test_1() {
    let mut comp1 = vec![ 1.0, 1.0, 0.0, 0.0];
    let mut comp2 = vec![ 1.0, 0.0, 1.0, 0.0];
    let mut comp3 = vec![ 2.0, 1.0, 1.0, 0.0];

    let mut mt1 = glrs_matrix::MatrixI::bind(&mut comp1).unwrap();
    let mt2 = glrs_matrix::MatrixI::bind(&mut comp2).unwrap();
    let mt3 = glrs_matrix::MatrixI::bind(&mut comp3).unwrap();

    assert!(mt1.add(&mt2).unwrap() == &mt3)

}
#[test]
fn component_0() {
    let mut comp1 = vec![ 1.0, 1.0, 3.0, 0.0];
    let mat = glrs_matrix::MatrixI::bind(&mut comp1).unwrap();
    let cmp = mat.get_component(1, 0).unwrap();
    assert!(cmp == 3.0)
}

#[test]
fn multiply_0() {
    let mut comp1 = vec![ 2.0, 1.0, 3.0, 4.0];
    let mut comp2 = vec![ 2.0, 0.0, 0.0, 2.0];
    let mut comp3 = vec![ 4.0, 2.0, 6.0, 8.0];
    let mut mat1 = glrs_matrix::MatrixI::bind(&mut comp1).unwrap();
    let mat2 = glrs_matrix::MatrixI::bind(&mut comp2).unwrap();
    let mat3 = glrs_matrix::MatrixI::bind(&mut comp3).unwrap();
 
    assert!(mat1.multiply(&mat2).unwrap() == &mat3);
}


// vi: se ts=4 sw=4 et:
