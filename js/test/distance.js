const assert = require('assert');

/**
 * distance test
 */
class Distance {

  /**
   * constructor
   */
  constructor() {
  }

  /**
   * instanciation test
   */
  instanciate(glrs) {
    const dis = glrs.distance_create(3.1); 

    assert.equal(0, glrs.distance_release(dis));
  }

  /**
   * value test
   */
  getValue0(glrs) {
    const dis = glrs.distance_create(3.1); 
    assert.equal(3.1, glrs.distance_get_value(dis));
    assert.equal(0, glrs.distance_release(dis));
  }

  /**
   * value test
   */
  getValue1(glrs) {
    const dis = glrs.distance_create(-3.1); 
    assert.equal(3.1, glrs.distance_get_abs_value(dis));
    assert.equal(0, glrs.distance_release(dis));
  }


  /**
   * run test
   */
  run(glrs) {
    this.instanciate(glrs);
    this.getValue0(glrs);
    this.getValue1(glrs);
  }
}


module.exports = Distance;
// vi: se ts=2 sw=2 et:
