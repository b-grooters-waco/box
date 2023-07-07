
## Running in the Browser

You can run this project using the legacy WebGL bindings or the new WebGPU 
bindings (with browsers that support them). We use the run-wasm model to 
simplify running in the browser with WASM.

You must set the ```web_sys_unstable_apis``` flag to run with native WebGPU in the browser

```ps1
> $Env:RUSTFLAGS="--cfg=web_sys_unstable_apis"
```

You can then run the WASM project with one of the following commands depending
 on whether you are running WebGPU or WebGL backends.

 WebGPU
```ps1
> cargo run-wasm --package box-render
```

WebGL
```ps1
> cargo run-wasm --package box-render --features webgl
```

