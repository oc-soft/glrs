const assert = require('assert');

/**
 * vector test
 */
class Vector {
  
  /**
   * constructor
   */
  constructor() {

  }


  /**
   * item storage
   */
  itemStorage(glrs) {
    let vec = glrs.vector_create(new Float32Array([1.0, 2.0, 3.0]));
    assert.equal(2.0, glrs.vector_get_component(vec, 1));
    glrs.vector_release(vec);
  }

  /**
   * reference count test
   */
  refCount0(glrs) {
    let vec = glrs.vector_create(new Float32Array([1.0, 2.0, 3.0]));

    let refCount = glrs.vector_retain(vec);
    assert.equal(2, refCount);
    refCount = glrs.vector_release(vec);
    assert.equal(1, refCount);
    refCount = glrs.vector_release(vec);
    assert.equal(0, refCount);
  }


  /**
   * run test
   */
  run(glrs) {
    this.itemStorage(glrs);
    this.refCount0(glrs);
  }
}

module.exports = Vector;

// vi: se ts=2 sw=2 et:
