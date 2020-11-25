# Matrix library for web-assembly

This is matrix library for web-assembly. I designed this libray for web gl, but
you may use general purpose program.

## How to use this in javascript

Matrix procedures are in web-assembly. You have to load the assembly first.

``` javascript
// load glrs.js 
const glrs = require('glrs')

let wasmMod = undefined;

if (typeof window === 'undefined') {
  // You have to compile if you use this library in nodejs.
  const path = require('path')
  const fs = require('fs')
  const wasmPath = path.join(
      path.dirname(require.resolve('glrs')), 'glrs_bg.wasm') 
  wasmMod = WebAssembly.compile(fs.readFileSync(wasmPath))
}

(async () => {
  const glrsInterface = await glrs.init(wasmMod)
  // You can use glrs matrix library
  
})
```

## Matrix operation in javascript

To use matrix, you have to get handle to matrix object. When you don't need it
anymore, You should release the handle from memory heap.

``` javascript
// instantiate matrix 
// create matrix 
// | 2.0 0.0 |
// | 0.0 1.0 |
const mt = glrs.matrix_create_with_components_row_order([2.0, 0.0, 0.0, 1.0]) 
// mt is handle

// You use mt 

// release matrix
glrs.matrix_release(mt)
``` 

You can do some matrix operations with matrix handle.

``` javascript
// the handle to matrix object
const matrix = [0, 0, 0]

matrix[0] =
glrs.matrix_create_with_components_row_order(
[ 7, 2, 1,
  0, 3, -1,
  -3, 4, -2])

// get inverse of matrix.
matrix[1] = glrs.matrix_inverse(matrix[0])

// If you pass the matrix having determinant 0, 
// you would have invalid handle which equals 0

// inverse of matrix is here.
const expectedComponents0 = [
  -2, 8, -5,
  3, -11, 7,
  9, -34, 21
]

const components0 = glrs.matrix_get_components_row_order(matrix[1])

components0.forEach( (compo, idx) => {
  // internal matrix use floating value.
  // some components may not equal exact value you expect for floating 
  // operation error.
  assert.ok(0.0001 > Math.abs((expectedComponents0[idx] - compo)))
})

// multiply operation
matrix[2] = glrs.matrix_multiply(matrix[0], matrix[1])
// result equals identity matrix
const expectedComponents1 = [
  1, 0, 0,
  0, 1, 0,
  0, 0, 1
]
const components1 = glrs.matrix_get_components_row_order(matrix[2])
components1.forEach( (compo, idx) => {
  // some value might not equal exactly for floating operation error.
  assert.ok(0.0001 > Math.abs((expectedComponents1[idx] - compo)))
})

// release matrix instance
matrix.forEach( mt => glrs.matrix_release(mt) )
```

## Matrix vector opeation

You can operate vector with matrix.

``` javascript
// the handle to matrix object
const matrix = [0, 0]

// create 3x3 matrix
matrix[0] = glrs.matrix_create_with_components_row_order(
  [ 7, 2, 1,
    0, 3, -1,
    -3, 4, -2 ])
// get inverse of matrix
matrix[1] = glrs.matrix_inverse(matrix[0])

// apply matrix to vector
const vec0 = glrs.matrix_apply_r_64(matrix[0], [2, 3, 4])
const vec1 = glrs.matrix_apply_r_64(matrix[1], vec0)

// you expect these components. 
const expectedVec0 = [ 24, 5, -2 ]


vec0.forEach((compo, idx) => {
  // some components do not equal exactly for floating operation error.
  assert.ok(0.0001 > Math.abs((expectedVec0[idx] - compo)))
})

const expectedVec1 = [ 2, 3, 4 ]

vec1.forEach((compo, idx) => {
  assert.ok(0.0001 > Math.abs((expectedVec1[idx] - compo)))
})

// release matrix instance from heap
matrix.forEach( mt => glrs.matrix_release(mt) )
```

## Using web-assembly by rust

This library is made from rust language. You might this library as rust's 
crate too. I customized some crates to compile this web-assembly.

1. I did not use wasmpack.
2. I used xtask to build web-assembly.

### Build without wasm-pack.

I had some requirements to use web-assembly.

1. load web-assembly separately from javascript at http page. 
2. have well transpiled type-script declaration.

I tried to run wasm-pack with web option first, I could not get either
requirements. If I load wasmpack separately, I had to write type-script
declaration by hand. If I use transpiled type-script declations, I had no
choice but to load one javascript embedding web-assebmly.

I customized wasm-bindgen tool to generate type-script declaration for my
requirement. The wasm-bindgen-cli has new option web-bundler. The option 
generates type-script declaration to use compiled web-assembly which is fetched
lately.

I would like to use the customized wasm-bindgen tool  with wasm-pack, but
wasm-pack did not give me the way to load the customized wasm-bindgen tool.
I gave up to use wasm-pack for my purpose.

### Using xtask

I learned that wasm-bindgen has to generate web-assembly after build the 
wasm32-unknown-unknown target library. I would like to run some commands after
I ran `cargo build --target wasm32-unknown-unknown`. I found [this discussion](https://github.com/rust-lang/cargo/issues/545). I realized that I can not have
post build script officially. In this discussion, [xtask suggention](https://github.com/matklad/cargo-xtask)
is nice for me following reasons. 

1. You don't have to learn any other syntax but rust to build your project.
2. You can use all of resources of rust natural way.


