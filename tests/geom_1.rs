use glrs::geom;

#[cfg(test)]
#[test]
fn offset_0() {
    let p1 = vec!(1.0, 0.0);
    let p2 = vec!(2.0, 0.0);
    let expected = vec!(vec!(1.0, 1.0), vec!(2.0, 1.0));

    let res = geom::D2::offset_points(1.0, &p1, &p2);

    let pts = res.unwrap();

    for i in 0..pts.len() {
        let v = geom::minus(&pts[i], &expected[i]).unwrap();
        assert!(geom::length(&v).unwrap() < 0.00001);
    }

}

#[test]

fn offset_1() {
    let p1 = vec!(1.0, 0.0);
    let p2 = vec!(1.00000001, 0.0);

    let res = geom::D2::offset_points_0(1.0, &p1, &p2, 0.01);

    assert!(res.is_err())
}

#[test]
fn offset_2() {

    let triangle_len = 1.0;
    let offset = 0.1;
    let small_triangle_len_0 = triangle_len - 2.0 * offset / 3f64.sqrt();
    let coord_offset = (triangle_len - small_triangle_len_0) / 2.0
        + 2.0 * offset / 3f64.sqrt();
    let small_triangle_len = triangle_len - 2.0 * coord_offset;
    let left_bottom = [0.0, 0.0];
    let pts = vec!(vec!(left_bottom[0], left_bottom[1]),
        vec!(left_bottom[0] + triangle_len, left_bottom[1]),
        vec!(left_bottom[0] + triangle_len / 2.0,
             left_bottom[1] + (triangle_len / 2.0) * 3f64.sqrt()));
    let expected = vec!(
        vec!(left_bottom[0] + coord_offset,
             left_bottom[1] + offset), 
        vec!(left_bottom[0] + coord_offset + small_triangle_len, 
             left_bottom[1] + offset),
        vec!(left_bottom[0] + coord_offset + small_triangle_len / 2.0,
             left_bottom[1] + offset
                + (small_triangle_len / 2.0) * 3f64.sqrt()));

    let res = geom::D2::offset_points_vec(offset, &pts, true);

    let pts = res.unwrap();
    for i in 0..pts.len() {
        let v = geom::minus(&pts[i], &expected[i]).unwrap();
        assert!(geom::length(&v).unwrap() < 0.00001);
    }
}

#[test]
fn offset_3() {

    let triangle_len = 1.0;
    let offset = 0.1;
    let small_triangle_len_0 = triangle_len - 2.0 * offset / 3f64.sqrt();
    let coord_offset = (triangle_len - small_triangle_len_0) / 2.0
        + 2.0 * offset / 3f64.sqrt();
    let small_triangle_len = triangle_len - 2.0 * coord_offset;
    let left_bottom = [0.0, 0.0];
    let pts = vec!(vec!(left_bottom[0], left_bottom[1]),
        vec!(left_bottom[0] + triangle_len, left_bottom[1]),
        vec!(left_bottom[0] + triangle_len / 2.0,
             left_bottom[1] + (triangle_len / 2.0) * 3f64.sqrt()));
    let expected = vec!(
        vec!(left_bottom[0],
             left_bottom[1] + offset), 
        vec!(left_bottom[0] + coord_offset + small_triangle_len, 
             left_bottom[1] + offset),
        vec!(left_bottom[0] + triangle_len / 2.0 
             - offset * 3f64.sqrt() / 2.0,
             left_bottom[1]
                + (triangle_len / 2.0) * 3f64.sqrt()
                - offset / 2.0));


    let res = geom::D2::offset_points_vec(offset, &pts, false);
    let pts = res.unwrap();
    for i in 0..pts.len() {
        let v = geom::minus(&pts[i], &expected[i]).unwrap();
        assert!(geom::length(&v).unwrap() < 0.00001);
    }
}


// vi: se ts=4 sw=4 et:



