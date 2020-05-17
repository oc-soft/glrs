use glrs::geom;

#[cfg(test)]
#[test]
fn length_0() {
    assert!(5.0 == geom::length(&vec!(3.0, 4.0)).unwrap())
}

#[test]
fn dot_product_0() {
    assert!(
        geom::dot_product(&vec!(2.0, 3.0), &vec!(4.0, 5.0)).unwrap() == 23.0
    )
}

#[test]
fn scale_0() {
    let mut v0 = vec![3.0, 2.0];
    geom::scale_0(3.0, &mut v0);
    assert!(v0 == vec!(9.0, 6.0))
}

#[test]
fn scale_1() {
    assert!(geom::scale(3.0, &vec!(2.0, 4.0)) == vec!(6.0, 12.0))
}

#[test]
fn plus_0() {
    assert!(
        geom::plus(&vec!(2.0, 4.0), &vec!(5.0, 6.0)).unwrap()
            == vec!(7.0, 10.0)
    )
}

#[test]
fn minus_0() {
    assert!(
        geom::minus(&vec!(2.0, 4.0), &vec!(5.0, 6.0)).unwrap()
            == vec!(-3.0, -2.0)
    )
}
