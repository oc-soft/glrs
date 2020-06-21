use glrs::Segment;
use glrs::geom;

#[cfg(test)]
#[test]
fn instantiate_0() {
    let p1 = vec!(0.0, 0.0);
    let p2 = vec!(2.0, 3.0);

    let seg = Segment::create_1(&p1, &p2).unwrap();

    assert!(seg.dim() == 2);
    assert!(
        geom::length(
            &geom::minus(
                &seg.p1(), &p1).unwrap()).unwrap() < 0.0001);
    assert!(
        geom::length(
            &geom::minus(
                &seg.p2(), &p2).unwrap()).unwrap() < 0.0001);
}

#[test]
fn instantiate_1() {
    let p1 = vec!(1.0, 0.0);
    let p2 = vec!(2.0, 6.0);

    let seg = Segment::create_1(&p1, &p2).unwrap();

    assert!(seg.dim() == 2);
    assert!(
        geom::length(
            &geom::minus(
                &seg.p1(), &p1).unwrap()).unwrap() < 0.0001);
    assert!(
        geom::length(
            &geom::minus(
                &seg.p2(), &p2).unwrap()).unwrap() < 0.0001);
}

#[test]
fn instantiate_2() {
    let p1 = vec!(0.0, 0.0, 0.0);
    let p2 = vec!(2.0, 6.0, 3.0);

    let seg = Segment::create_1(&p1, &p2).unwrap();

    assert!(seg.dim() == 3);
    assert!(
        geom::length(
            &geom::minus(
                &seg.p1(), &p1).unwrap()).unwrap() < 0.0001);
    assert!(
        geom::length(
            &geom::minus(
                &seg.p2(), &p2).unwrap()).unwrap() < 0.0001);
}

#[test]
fn instantiate_3() {
    let p1 = vec!(0.0, 0.0);
    let p2 = vec!(2.0, 0.0);

    let seg = Segment::create_1(&p1, &p2).unwrap();

    assert!(seg.dim() == 2);
    assert!(
        geom::length(
            &geom::minus(
                &seg.p1(), &p1).unwrap()).unwrap() < 0.0001);
    assert!(
        geom::length(
            &geom::minus(
                &seg.p2(), &p2).unwrap()).unwrap() < 0.0001);
}


#[test]
fn param_0() {
    let p1 = vec!(0.0, 0.0);
    let p2 = vec!(2.0, 3.0);

    let seg = Segment::create_1(&p1, &p2).unwrap();

    let len = geom::length(&p2).unwrap();

    assert!(seg.t()[0] < 0.00001);
    assert!((len - seg.t()[1]).abs() < 0.00001);

    assert!(seg.point_on_line()[1] < 0.000001);
}

#[test]
fn param_1() {
    let p1 = vec!(1.0, 1.0);
    let p2 = vec!(2.0, 3.0);

    let mut d = vec!(1.0, 2.0);
    let d_len = geom::length(&d).unwrap(); 
    geom::scale_0(1.0 / d_len, &mut d);
    let seg = Segment::create_1(&p1, &p2).unwrap();


    assert!((seg.direction()[0] - d[0]).abs() < 0.00001);
    assert!((seg.direction()[1] - d[1]).abs() < 0.00001);

    assert!((0.5 - seg.point_on_line()[0]).abs() < 0.00001);
    assert!(seg.point_on_line()[1] < 0.000001);

    assert!((seg.p1()[0] - p1[0]).abs() < 0.000001);
    assert!((seg.p1()[1] - p1[1]).abs() < 0.000001);

    assert!((seg.p2()[0] - p2[0]).abs() < 0.000001);
    assert!((seg.p2()[1] - p2[1]).abs() < 0.000001)
}

#[test]
fn param_2() {
    let p2 = vec!(1.0, 1.0);
    let p1 = vec!(2.0, 3.0);

    let mut d = vec!(-1.0, -2.0);
    let d_len = geom::length(&d).unwrap(); 
    geom::scale_0(1.0 / d_len, &mut d);
    let seg = Segment::create_1(&p1, &p2).unwrap();


    assert!((seg.direction()[0] - d[0]).abs() < 0.00001);
    assert!((seg.direction()[1] - d[1]).abs() < 0.00001);

    assert!((0.5 - seg.point_on_line()[0]).abs() < 0.00001);
    assert!(seg.point_on_line()[1] < 0.000001);

    assert!((seg.p1()[0] - p1[0]).abs() < 0.000001);
    assert!((seg.p1()[1] - p1[1]).abs() < 0.000001);

    assert!((seg.p2()[0] - p2[0]).abs() < 0.000001);
    assert!((seg.p2()[1] - p2[1]).abs() < 0.000001)
}




// vi: se ts=4 sw=4 et:
