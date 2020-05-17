const assert = require('assert');
const glrs = require('glrs');

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
  instanciate() {
    const dis = glrs.distance_create(3.1); 

    assert.equal(0, glrs.distance_release(dis));
  }

  /**
   * value test
   */
  getValue0() {
    const dis = glrs.distance_create(3.1); 
    assert.equal(3.1, glrs.distance_get_value(dis));
    assert.equal(0, glrs.distance_release(dis));
  }

  /**
   * value test
   */
  getValue1() {
    const dis = glrs.distance_create(-3.1); 
    assert.equal(3.1, glrs.distance_get_abs_value(dis));
    assert.equal(0, glrs.distance_release(dis));
  }


  /**
   * run test
   */
  run() {
    this.instanciate();
    this.getValue0();
    this.getValue1();
  }
}


module.exports = Distance;
// vi: se ts=2 sw=2 et:
