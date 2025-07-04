# Broken Deno Extension

This project reproduces the problem of failure to build a deno extension by following the example extension
https://github.com/denoland/deno/tree/v2.4.0/ext/broadcast_channel.

## Enviroment
- rust 1.87.0

## Project layout
```bash
`-- crates
    |-- greeter
    |   |-- Cargo.toml
    |   |-- hello_world.js
    |   `-- src
    |       `-- lib.rs
    `-- hello-world
        |-- Cargo.toml
        |-- build.rs
        |-- src
        |   `-- main.rs
        `-- static
            `-- app.js
```

where
- `greeter` is the extension
- `hello-world` is the main program embeding the extension to run static/app.js

## Rereproduction
```bash
cd crates/hello-world

cargo run
```

Errors goes as
```bash
error: failed to run custom build command for `hello-world v0.1.0 (/github.com/sammyne/broken-deno-extension/crates/hello-world)`

Caused by:
  process didn't exit successfully: `/github.com/sammyne/broken-deno-extension/target/debug/build/hello-world-c978182b1db19571/build-script-build` (exit status: 101)
  --- stdout
  Creating a snapshot...

  --- stderr
  WARNING: v8::OwnedIsolate for snapshot was leaked

  thread 'main' panicked at /root/.cargo/registry/src/rsproxy.cn-e3de039b2554c837/deno_core-0.352.0/runtime/jsruntime.rs:2185:9:
  Failed to initialize JsRuntime for snapshotting: NonEvaluatedModules(["ext:hello_world/hello_world.js"])
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Really appreciate if someone could helps me out~
