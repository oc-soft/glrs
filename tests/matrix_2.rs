use glrs::Matrix;

#[cfg(test)]
#[test]
fn load_col_order_0() {
    let comp = vec![
        1.2, 2.3, 3.4,
        4.5, 5.6, 6.7,
        7.8, 8.9, 9.10
    ];
    let mat0 = Matrix::new_with_source_component_row_order(&comp);
    let mat1 = Matrix::new_with_source_component_col_order(&comp);
    let mat1t = mat1.transpose();
    assert!(mat0 == mat1t);
}

#[test]
fn cross_product_0() {
    let vec1 = vec![1.0, 2.0, 3.0];
    let vec2 = vec![4.0, 5.0, 6.0];

    let cross_mat = Matrix::new_cross_product(vec1[0], vec1[1], vec1[2]);

    let vec3 = cross_mat.apply_r(&vec2).unwrap(); 
    
    let expect_vec = vec![-3_f64, 6_f64, -3_f64];
    assert!(vec3 == expect_vec);
}


#[test]
fn rotation_0() {

    let rot_mat = Matrix::new_with_rotate(120_f64, 1f64, 1f64, 1f64).unwrap();

    let x_axis = vec![1f64, 0f64, 0f64];
    let y_axis = vec![0f64, 1f64, 0f64];    
    let z_axis = vec![0f64, 0f64, 1f64];

    let rot_x = rot_mat.apply_r(&x_axis).unwrap();
    let diff_vec_1 = glrs::geom::minus(&rot_x, &y_axis).unwrap(); 
    let diff_vec_1_len = glrs::geom::length(&diff_vec_1).unwrap();

    let rot_y = rot_mat.apply_r(&y_axis).unwrap();
    let diff_vec_2 = glrs::geom::minus(&rot_y, &z_axis).unwrap(); 
    let diff_vec_2_len = glrs::geom::length(&diff_vec_2).unwrap();

    let rot_z = rot_mat.apply_r(&z_axis).unwrap();
    let diff_vec_3 = glrs::geom::minus(&rot_z, &x_axis).unwrap(); 
    let diff_vec_3_len = glrs::geom::length(&diff_vec_3).unwrap();


    assert!(diff_vec_1_len < 0.000001);
    assert!(diff_vec_2_len < 0.000001);
    assert!(diff_vec_3_len < 0.000001);
}

#[test]
fn rotation_1() {
    let mut rot_mat = Matrix::new_with_rotate(
        120_f64, 1f64, 1f64, 1f64).unwrap();
    rot_mat.rotate_mut(120_f64, 1f64, 1f64, 1f64).unwrap();

    let x_axis = vec![1f64, 0f64, 0f64];
    let y_axis = vec![0f64, 1f64, 0f64];    
    let z_axis = vec![0f64, 0f64, 1f64];

    let rot_x = rot_mat.apply_r(&x_axis).unwrap();
    let diff_vec_1 = glrs::geom::minus(&rot_x, &z_axis).unwrap(); 
    let diff_vec_1_len = glrs::geom::length(&diff_vec_1).unwrap();

    let rot_y = rot_mat.apply_r(&y_axis).unwrap();
    let diff_vec_2 = glrs::geom::minus(&rot_y, &x_axis).unwrap(); 
    let diff_vec_2_len = glrs::geom::length(&diff_vec_2).unwrap();

    let rot_z = rot_mat.apply_r(&z_axis).unwrap();
    let diff_vec_3 = glrs::geom::minus(&rot_z, &y_axis).unwrap(); 
    let diff_vec_3_len = glrs::geom::length(&diff_vec_3).unwrap();


    assert!(diff_vec_1_len < 0.000001);
    assert!(diff_vec_2_len < 0.000001);
    assert!(diff_vec_3_len < 0.000001);
}


#[test]
fn translate_0() {
    let mut trans_mat = Matrix::new(4).unwrap();
    trans_mat.translate_mut(2f64, 3f64, 4f64).unwrap();
    trans_mat.translate_mut(1f64, 0f64, 0f64).unwrap();

    let res = trans_mat.apply_r(&vec![0f64, 0f64, 0f64, 1f64]).unwrap();

    let expected = vec![3f64, 3f64, 4f64, 1f64];

    assert!(res == expected);
}

#[test]
fn scale_0() {
    let mut scale_mat = Matrix::new(4).unwrap();
    scale_mat.scale3_mut(2f64, 3f64, 4f64).unwrap();
    scale_mat.scale3_mut(1f64, 1f64, 2f64).unwrap();

    let res = scale_mat.apply_r(&vec![1f64, 1f64, 1f64, 1f64]).unwrap();

    let expected = vec![2f64, 3f64, 8f64, 1f64];

    assert!(res == expected);
}


// vi: se ts=4 sw=4 et: 
