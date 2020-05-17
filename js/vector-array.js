
const glrs = require('glrs');
const assert = require('assert');

/// vector array test
class VectorArray {

  /// constructor
  constructor() {
  }


  /// test to access item
  accessItems0() {
    const va = glrs.vector_array_create();

    assert.equal(undefined, glrs.vector_array_dimension(va));

    assert(glrs.vector_array_add_1(va, [3.4, 2.4, 5.5]));
    assert.equal(1, glrs.vector_array_size(va));
   
    assert.equal(3, glrs.vector_array_dimension(va));

    assert(!glrs.vector_array_add_1(va, [1.4, 3.4]));

    assert(glrs.vector_array_add_1(va, [1.4, 3.4, 0.0]));
    assert.equal(2, glrs.vector_array_size(va));
    assert.equal(0, glrs.vector_array_release(va));
  }

  run() {
    this.accessItems0(); 
  }
}


module.exports = VectorArray;

// vi: se ts=2 sw=2 et:
