use std::path::PathBuf;

use deno_runtime::ops::bootstrap::SnapshotOptions;

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("get env OUT_DIR"))
        .join("DENO_EXT_QUICKSTART.snapshot");

    let snapshot_options = SnapshotOptions {
        ts_version: "5.8.3".to_owned(),
        v8_version: deno_core::v8::VERSION_STRING,
        target: std::env::var("TARGET").unwrap(),
    };

    // add custom extensions
    let custom = vec![greeter::hello_world::init()];

    deno_runtime::snapshot::create_runtime_snapshot(out, snapshot_options, custom);
}
