/// represent matrix
#[readonly::make]
pub struct MatrixI<'a> {
    component: &'a mut Vec<f64>,
    pub col_count: usize,
}

#[derive(Debug)]
pub struct MatrixError;

// Matrix implementatin
impl<'a> MatrixI<'a>  {
    /// create matrix instance to bind vector
    pub fn bind<'b: 'a>(comp:&'b mut Vec<f64>) ->Result<Self, MatrixError> {
        if comp.len() == 0 {
            return Err(MatrixError);
        }
        let flen = comp.len() as f64;
        Ok(MatrixI {
            component: comp,
            col_count: (flen.sqrt().floor()) as usize,
        })
    }

    pub fn bind_with_count<'b: 'a>(comp:&'b mut Vec<f64>, col_count: usize) -> Result<Self, MatrixError> {
        let comp_size = col_count.pow(2);
        while comp.len() < comp_size {
            comp.push(0.0);
        }
        Self::bind(comp)
    }

    /// get count of component
    pub fn count(&self) -> usize {
        self.component.len()
    }


    /// get count of columns
    pub fn col_count(&self) -> usize {
        self.col_count
    }

    /// get count of rows
    pub fn row_count(&self) -> usize {
        self.col_count()
    }

    /// add matrix
    pub fn add(&'a mut self, matrix: &MatrixI) -> Result<&MatrixI, MatrixError> {
        if self.count() != matrix.count() {
            Err(MatrixError)
        } else {
            for i in 0..self.count() {
                self[i] += matrix[i];
            }
            Ok(self)
        }
    }

    /// multiply
    pub fn multiply(&'a mut self, other: &MatrixI) -> Result<&MatrixI, MatrixError> {
        if self.count() == other.count() {
            let mut tmp_comp = Vec::with_capacity(self.count());
            for ridx in 0..self.row_count() {
                for cidx in 0..self.col_count() {
                    let mut cmp = 0.0;
                    for rcidx in 0..other.row_count() {
                        cmp +=
                            self.get_component(ridx, rcidx).unwrap()
                            * other.get_component(rcidx, cidx).unwrap();
                    }
                    tmp_comp.push(cmp);
                }
            }
            self.component.clone_from_slice(&tmp_comp[0..]);
            Ok(self) 
        } else {
            Err(MatrixError)
        }
    }


    /// copy field from other matrix
    pub fn copy(&'a mut self, other: &Self) -> Result<&Self, MatrixError> {
        if self.count() <= other.count() {
            let count = self.count();
            self.component.clone_from_slice(&other.component[0..count]);
            Ok(self)
        } else {
            Err(MatrixError)
        }
    }


    /// get component
    pub fn get_component(&self, row : usize, col: usize) -> Result<f64, MatrixError> {
        let idx = self.col_count * row + col;
        if idx < self.count() {
            Ok(self[idx])
        } else {
            Err(MatrixError)
        }
    }

    /// set component
    pub fn set_component(&mut self, row: usize, col: usize, val: f64) ->Option<MatrixError> {
        let idx = self.col_count * row + col;
        if idx < self.count() {
            self[idx] = val;
            None
        } else {
            Some(MatrixError)
        }
    }
}

impl<'a> std::ops::Index<usize> for MatrixI<'a> {
    type Output = f64;
    fn index(&self, index : usize)-> &Self::Output {
        &self.component[index]
    }
}


impl<'a> std::ops::IndexMut<usize> for MatrixI<'a> {
    fn index_mut(&mut self, index : usize)-> &mut Self::Output {
        &mut self.component[index]
    }
}

impl<'a> std::cmp::PartialEq for MatrixI<'a> {
    /// operator equals overloading
    fn eq(&self, other: &Self) -> bool {
        let mut result = self.count() == other.count();
        if result {
            for i in 0..self.count() {
                result = self[i] == other[i];
                if !result {
                    break;
                }
            }
        }
        result
    }
}

// vi: se ts=4 sw=4 et:
