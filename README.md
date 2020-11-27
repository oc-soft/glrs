# Matrix library for web-assembly

This is matrix library for web-assembly. I designed this libray for web-gl, but
you may use general purpose program.

# Build without wasm-pack

I had some requirements to use web-assembly.

1. load web-assembly separately from javascript at http page. 
2. have well transpiled type-script declaration.

I tried to run wasm-pack with web option first, I could not get either
requirements. If I load wasmpack separately, I had to write type-script
declaration by hand. If I use transpiled type-script declations, I had no
choice but to load one javascript embedding web-assebmly.

I customized [wasm-bindgen tool](https://github.com/toshiyuki-rs/wasm-bindgen)
to generate type-script declaration for my requirement.
The wasm-bindgen-cli has new option web-bundler. The option 
generates type-script declaration to use compiled web-assembly which is fetched
lately.

I would like to use the customized wasm-bindgen tool  with wasm-pack, but
wasm-pack did not give me the way to load the customized wasm-bindgen tool.
I gave up to use wasm-pack for my purpose.

# Using xtask

I learned that wasm-bindgen has to generate web-assembly after build the 
wasm32-unknown-unknown target library. I would like to run some commands after
I ran `cargo build --target wasm32-unknown-unknown`. I found [this discussion](https://github.com/rust-lang/cargo/issues/545). I realized that I can not have
post build script officially. In this discussion, [xtask suggention](https://github.com/matklad/cargo-xtask)
is nice for me in following reasons. 

1. You don't have to learn any other syntax but rust to build your project.
2. You can use all of resources of rust natural way.
