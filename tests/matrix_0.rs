use glrs::MatrixI;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
#[test]
fn create_instance() {
    let comp = Rc::new(RefCell::new(vec![2.0]));

    let mt = MatrixI::bind(comp).unwrap();
    assert!(mt.col_count == 1)
}

#[test]
fn add_test_1() {
    let comp1 = Rc::new(RefCell::new(vec![1.0, 1.0, 0.0, 0.0]));
    let comp2 = Rc::new(RefCell::new(vec![1.0, 0.0, 1.0, 0.0]));
    let comp3 = Rc::new(RefCell::new(vec![2.0, 1.0, 1.0, 0.0]));

    let mut mt1 = MatrixI::bind(comp1).unwrap();
    let mt2 = MatrixI::bind(comp2).unwrap();
    let mt3 = MatrixI::bind(comp3).unwrap();

    assert!(mt1.add(&mt2).unwrap() == &mt3)
}
#[test]
fn component_0() {
    let comp1 = Rc::new(RefCell::new(vec![1.0, 1.0, 3.0, 0.0]));
    let mat = MatrixI::bind(comp1).unwrap();
    let cmp = mat.get_component(1, 0).unwrap();
    assert!(cmp == 3.0)
}

#[test]
fn multiply_0() {
    let comp1 = Rc::new(RefCell::new(vec![2.0, 1.0, 3.0, 4.0]));
    let comp2 = Rc::new(RefCell::new(vec![2.0, 0.0, 0.0, 2.0]));
    let comp3 = Rc::new(RefCell::new(vec![4.0, 2.0, 6.0, 8.0]));
    let mut mat1 = MatrixI::bind(comp1).unwrap();
    let mat2 = MatrixI::bind(comp2).unwrap();
    let mat3 = MatrixI::bind(comp3).unwrap();

    assert!(mat1.multiply(&mat2).unwrap() == &mat3);
}

#[test]
fn comp_access_0() {
    let comp1 = Rc::new(RefCell::new(vec![2.0, 1.0, 3.0, 4.0]));
    let mut mat1 = MatrixI::bind(comp1).unwrap();
    mat1.set_component(0, 0, 5.0).unwrap();

    assert!(mat1.get_component(0, 0).unwrap() == 5.0);
}

// vi: se ts=4 sw=4 et:
