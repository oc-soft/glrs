
const glrs = require('glrs');
const Vector = require('./vector');
const Distance = require('./distance');
const VectorArray = require('./vector-array');
const Plane = require('./plane');
(()=> {
  (new Vector()).run();
  (new Distance()).run();
  (new VectorArray()).run();
  (new Plane()).run();
})();

// vi: se ts=2 sw=2 et:
