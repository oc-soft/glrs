const glrs = require('../glrs')
const Vector = require('./vector')
const Distance = require('./distance')
const VectorArray = require('./vector-array')
const Plane = require('./plane')
const Matrix = require('./matrix')

let wasmMod = undefined;
if (typeof window === 'undefined') {
  const path = require('path')
  const fs = require('fs')
  const wasmPath = path.join(
      path.dirname(require.resolve('../glrs')), 'glrs_bg.wasm') 
  wasmMod = WebAssembly.compile(fs.readFileSync(wasmPath))
}

(async () => {
  const glrsModule = await glrs.init(wasmMod)

  const testObjects = [
    new Vector(),
    new Distance(),
    new VectorArray(),
    new Plane(),
    new Matrix()
  ] 
  testObjects.forEach(testObj => {
    testObj.run(glrsModule)
  })
  console.log('test finished') 
})()

// vi: se ts=2 sw=2 et:
