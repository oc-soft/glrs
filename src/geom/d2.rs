use crate::geom::GeomError;
use crate::Segment;
use crate::geom;

pub struct D2 {
}

impl D2 {

    /// move each points offset along to line(p1 p2) normal vector
    pub fn offset_points_0(
        offset: f64, p1: &[f64], p2: &[f64], tolerance:f64)
        -> Result<[Vec<f64>; 2], GeomError> { 
        if p1.len() > 1 && p2.len() > 1 {
            let p1_d2 = vec!(p1[0], p1[1]);
            let p2_d2 = vec!(p2[0], p2[1]); 
            let d = geom::minus(&p2_d2, &p1_d2).unwrap();
            let d_len = geom::length(&d).unwrap();
            if d_len > tolerance {
                let n_vec = vec!(- d[1] / d_len, d[0] / d_len);
                let disp = geom::scale(offset, &n_vec);
                let result = [geom::plus(&p1_d2, &disp).unwrap(),
                    geom::plus(&p2_d2, &disp).unwrap()];
                Ok(result)
            } else {
                Err(GeomError)
            }
        } else {
            Err(GeomError)
        }
    }

    /// move each points offset along to line(p1 p2) normal vector
    pub fn offset_points(
        offset: f64, p1: &[f64], p2: &[f64])
        -> Result<[Vec<f64>; 2], GeomError> { 
        Self::offset_points_0(offset, p1, p2, 0.0)
    }

    /// move all of points with offset displacement
    pub fn offset_points_vec_0(
        offset: f64, points: &[Vec<f64>], 
        close: bool, tolerance:f64)
        -> Result<Vec<Vec<f64>>, GeomError> {
        
        if points.len() > 1 {
            let end_idx;
            let mut state;
            if close {
                end_idx = points.len();
            } else {
                end_idx = points.len() - 1;
            }
            let res_0;
            res_0 = Self::offset_points_0(
                offset,
                &points[0], &points[1], tolerance);
            state = true;
            let mut end_pt_ref : Option<Vec<f64>> = None;
            let mut offset_points: Vec<Vec<f64>>
                = Vec::with_capacity(points.len());
            if let Ok(pts) = res_0 {
                offset_points.push(Self::dup_vec(&pts[0]));
                end_pt_ref = Some(Self::dup_vec(&pts[1]));
            } else {
                state = false;
            }
            
            if state {
                let mut first_seg_ref : Option<Segment> = None;

                let mut last_pts : Option<[Vec<f64>;2]>;
                {
                    let end_pt = end_pt_ref.unwrap();
                    last_pts = Some(
                        [Self::dup_vec(&offset_points[0]),
                         Self::dup_vec(&end_pt)]);
                    end_pt_ref = Some(end_pt);
                }
                for idx in 1..end_idx {
                    let res_1 = Self::offset_points_0(
                        offset,
                        &points[(idx) % points.len()], 
                        &points[(idx + 1) % points.len()], tolerance);
                    
                    if let Ok(pts_1) = res_1 {
                        let pts_0 = last_pts.unwrap();


                        let seg_res_0 = Segment::create_1(&pts_0[0], &pts_0[1]);
                        let seg_res_1 = Segment::create_1(&pts_1[0], &pts_1[1]);

                        last_pts = Some(pts_1);
                        match (seg_res_0, seg_res_1) {
                            (Ok(seg_0), Ok(seg_1)) => {
                                let cross_res = 
                                    seg_0.cross_point_parameter_2d_0(
                                        &seg_1, tolerance);
                            
                                if let Some(pt_params) = cross_res {
                                    offset_points.push(
                                        seg_0.point_on_t(pt_params[0]))
                                } else {
                                    offset_points.push(
                                        Self::dup_vec(&pts_0[1]));
                                }
                                if idx == 1 {
                                    first_seg_ref = Some(seg_0);
                                }
                                if idx == end_idx - 1 {
                                    if close {
                                        let first_seg = first_seg_ref.unwrap(); 
                                        let cross_res =
                                            seg_1.cross_point_parameter_2d_0(
                                                &first_seg,
                                                tolerance); 
                                        first_seg_ref = Some(first_seg);
                                        if let Some(pt_params) = cross_res {
                                            end_pt_ref = 
                                                Some(seg_1.point_on_t(
                                                    pt_params[0]));
                                        } else {
                                            state = false;
                                            break;
                                        }
                                    } else {
                                        let pts_1 = last_pts.unwrap();
                                        end_pt_ref = Some(
                                            Self::dup_vec(&pts_1[1]));
                                        last_pts = Some(pts_1);
                                    }
                                }
                            },
                            _ => {
                                state = false;
                                break;
                            }
                        }
                    }
                }
            }
            if state {
                if let Some(pt) = end_pt_ref {
                    if close {
                        offset_points[0] = pt;
                    } else {
                        offset_points.push(pt);
                    }
                }
            }
            if state {
                Ok(offset_points)
            } else {
                Err(GeomError)
            }
        } else {
            Err(GeomError)
        }
    }
    /// move all of points with offset displacement
    pub fn offset_points_vec(
        offset: f64, points: &[Vec<f64>], 
        close: bool)
        -> Result<Vec<Vec<f64>>, GeomError> {
        Self::offset_points_vec_0(offset, points, close, 0.0)
    }
 
    fn dup_vec(source: &[f64]) -> Vec<f64> {
        let mut result = Vec::with_capacity(source.len());
        result.extend_from_slice(source);
        result
    }
}

// vi: se ts=4 sw=4 et: 
