use glrs::Segment;
use glrs::geom;


#[cfg(test)]
#[test]
fn cross_point_0() {
    let p1_1 = vec!(1.0, 1.0); 
    let p2_1 = vec!(2.0, 2.0);

    let p1_2 = vec!(1.0, 2.0); 
    let p2_2 = vec!(2.0, 1.0);

    let expected_p = vec!(1.5, 1.5);

    let seg_1 = Segment::create_1(&p1_1, &p2_1).unwrap();
    let seg_2 = Segment::create_1(&p1_2, &p2_2).unwrap();
    println!("{:?}", seg_1);
    println!("{:?}", seg_2);


    let cross_params = seg_1.cross_point_parameter_2d(&seg_2).unwrap();


    println!("{:?}", cross_params);
    let cross_pt_1 = seg_1.point_on_t(cross_params[0]);
    let cross_pt_2 = seg_2.point_on_t(cross_params[1]);

    let diff_1 = geom::minus(&expected_p, &cross_pt_1).unwrap();
    let diff_2 = geom::minus(&expected_p, &cross_pt_2).unwrap();

    assert!(geom::length(&diff_1).unwrap() < 0.000001);
    assert!(geom::length(&diff_2).unwrap() < 0.000001)
}


// vi: se ts=4 sw=4 et:
