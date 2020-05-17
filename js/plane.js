
const assert = require('assert');
const glrs = require('glrs');

/**
 * test plane object
 */
class Plane {

  /**
   * constructor
   */
  constructor() {
  }

  /**
   * test instanciate
   */
  instanciate0() {
    const p = glrs.plane_create([], []);
    assert.equal(p, 0);
  }

  /**
   * test instaciate
   */
  instanciate1() {
    const p = glrs.plane_create([0, 0, 1.0], [2, 3, 1]);
    assert(p != 0);

    assert.equal(0, glrs.plane_release(p));
  }

  /**
   * test distance
   */
  distance0() {
    const p = glrs.plane_create([0, 0, 1.0], [2, 3, 1]);
    assert(p != 0);

    const dis = glrs.plane_distance(p, [0, 0, 0])
    
    assert.equal(-1, dis);
    assert.equal(0, glrs.plane_release(p));
  }

  /**
   * test to sort points
   */
  sort0() {
    const p = glrs.plane_create([0, 0, 1.0], [2, 3, 1]);
    assert(p != 0);

    const vecs = [
      [ 1.0, 0.2, 0.3 ],
      [ 10.0, 3.0, 1.8],
      [ 20, 100.0, 5]
    ];

    const di = glrs.plane_sort_points(p, vecs);
    const distances = glrs.distance_indices_get_distances(di);
    assert.equal(3, glrs.distances_size(distances));
    const expectedIdxs = [
      new Uint32Array([ 0 ]), 
      new Uint32Array([ 1 ]), 
      new Uint32Array([ 2 ])
    ];
    for (let i = 0; i < glrs.distances_size(distances); i++) {
      const dis = glrs.distances_get(distances, i);
      assert(dis !== undefined ); 
    
      
      let idxs = glrs.distance_indices_get_indices(di, dis);

      assert(idxs !== undefined);
      
      assert.deepStrictEqual(idxs, expectedIdxs[i]);

      glrs.distance_release(dis);
    }


    assert.equal(0, glrs.distances_release(distances));
    assert.equal(0, glrs.distance_indices_release(di)); 
    assert.equal(0, glrs.plane_release(p));

  }
  /**
   * test to sort points
   */
  sort1() {
    const p = glrs.plane_create([0, 0, 1.0], [2, 3, 1]);
    assert(p != 0);

    const vecs = [
      [ 10.0, 3.0, 1.8],
      [ 1.0, 0.2, 0.3 ],
      [ 20, 100.0, 5]
    ];

    const di = glrs.plane_sort_points(p, vecs);
    const distances = glrs.distance_indices_get_distances(di);
    assert.equal(3, glrs.distances_size(distances));
    const expectedIdxs = [
      new Uint32Array([ 1 ]), 
      new Uint32Array([ 0 ]), 
      new Uint32Array([ 2 ])
    ];
    for (let i = 0; i < glrs.distances_size(distances); i++) {
      const dis = glrs.distances_get(distances, i);
      assert(dis !== undefined ); 
    
      
      let idxs = glrs.distance_indices_get_indices(di, dis);

      assert(idxs !== undefined);
      
      assert.deepStrictEqual(idxs, expectedIdxs[i]);

      glrs.distance_release(dis);
    }


    assert.equal(0, glrs.distances_release(distances));
    assert.equal(0, glrs.distance_indices_release(di)); 
    assert.equal(0, glrs.plane_release(p));

  }

  /**
   * test to sort points
   */
  sort2() {
    const p = glrs.plane_create([0, 0, 1.0], [2, 3, 1]);
    assert(p != 0);

    const vecs = [
      [ 10.0, 3.0, 1.7],
      [ 1.0, 0.2, 0.3 ],
      [ 20, 100.0, 5]
    ];

    const di = glrs.plane_sort_points(p, vecs);
    const distances = glrs.distance_indices_get_distances(di);
    assert.equal(2, glrs.distances_size(distances));
    const expectedIdxs = [
      new Uint32Array([ 0, 1 ]), 
      new Uint32Array([ 2 ])
    ];
    for (let i = 0; i < glrs.distances_size(distances); i++) {
      const dis = glrs.distances_get(distances, i);
      assert(dis !== undefined ); 
    
      
      let idxs = glrs.distance_indices_get_indices(di, dis);

      assert(idxs !== undefined);
      
      assert.deepStrictEqual(idxs, expectedIdxs[i]);

      glrs.distance_release(dis);
    }


    assert.equal(0, glrs.distances_release(distances));
    assert.equal(0, glrs.distance_indices_release(di)); 
    assert.equal(0, glrs.plane_release(p));

  }


  /**
   * run test
   */
  run() {
    this.instanciate0();
    this.instanciate1();
    this.distance0();
    this.sort0();
    this.sort1();
    this.sort2();
 }
}


module.exports = Plane;
// vi:se ts=2 sw=2 et:
