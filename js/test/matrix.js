
const assert = require('assert');


/**
 * test matrix library
 */
class Matrix {
  /**
   * constructor
   */
  constrcutor() {
  }


  /**
   * test for instantiating
   */
  instantiate0(glrs) {
    const mt = glrs.matrix_create_with_components_row_order(
      [2.0, 0.0, 0.0, 1.0]) 
    assert.notEqual(mt, 0)
    assert.equal(glrs.matrix_release(mt), 0)
  }

  
  /**
   * test multiply 
   */
  mutltiply0(glrs) {
    matrix = [ 0, 0, 0]
    matrix[0] = glrs.matrix_create_with_components_row_order(
      [2.0, 0.0, 0.0, 5.0]) 

    matrix[1] = glrs.matrix_create_with_components_row_order(
      [2.0, 0.0, 0.0, 2.0]) 

    matrix[2] = glrs.matrix_multiply(mt0, mt1)


    const expectedComponents = [ 4.0, 0.0, 0.0, 10.0 ]

    const components = glrs.matrix_get_components_row_order(matrix[2])

    assert.equal(expectedComponents.length, components.length)

    components.forEach( (compo, idx) => {
      assert.ok(0.0001 > Math.abs((expectedComponents[idx] - compo)))
    })

    // release matrix instance
    matrix.forEach( mt => glrs.matrix_release(mt) )
  }


  /**
   * test inverse matrix
   */
  inverse0(glrs) {
    const matrix = [0, 0, 0]

    matrix[0] =
      glrs.matrix_create_with_components_row_order(
        [ 7, 2, 1,
          0, 3, -1,
          -3, 4, -2])
    matrix[1] = glrs.matrix_inverse(matrix[0])

    assert.notEqual(matrix[1], 0)

    const expectedComponents0 = [
      -2, 8, -5,
      3, -11, 7,
      9, -34, 21
    ]

    const components0 = glrs.matrix_get_components_row_order(matrix[1])

    components0.forEach( (compo, idx) => {
      assert.ok(0.0001 > Math.abs((expectedComponents0[idx] - compo)))
    })

    matrix[2] = glrs.matrix_multiply(matrix[0], matrix[1])

    const expectedComponents1 = [
      1, 0, 0,
      0, 1, 0,
      0, 0, 1
    ]

    const components1 = glrs.matrix_get_components_row_order(matrix[2])

    components1.forEach( (compo, idx) => {
      assert.ok(0.0001 > Math.abs((expectedComponents1[idx] - compo)))
    })

    // release matrix instance
    matrix.forEach( mt => glrs.matrix_release(mt) )
  }

  /**
   * test opration matrix vector
   */
  operateToVector0(glrs) {
    const matrix = [0, 0]

    matrix[0] =
      glrs.matrix_create_with_components_row_order(
        [ 7, 2, 1,
          0, 3, -1,
          -3, 4, -2])
    matrix[1] = glrs.matrix_inverse(matrix[0])

    assert.notEqual(matrix[1], 0)

    const vec0 = glrs.matrix_apply_r_64(matrix[0], [2, 3, 4])
    const vec1 = glrs.matrix_apply_r_64(matrix[1], vec0)

    const expectedVec0 = [ 24, 5, -2 ]

    vec0.forEach((compo, idx) => {
      assert.ok(0.0001 > Math.abs((expectedVec0[idx] - compo)))
    })

    const expectedVec1 = [ 2, 3, 4 ]

    vec1.forEach((compo, idx) => {
      assert.ok(0.0001 > Math.abs((expectedVec1[idx] - compo)))
    })

    // release matrix instance
    matrix.forEach( mt => glrs.matrix_release(mt) )
  }


  /**
   * do test glrs
   */
  run(glrs) {
    this.instantiate0(glrs)
    this.inverse0(glrs)
    this.operateToVector0(glrs)
  }
}


module.exports = Matrix
// vi: se ts=2 sw=2 et:
