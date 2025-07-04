use deno_core::{op2,extension};

#[op2(fast)]
fn greet(#[string] value: String) {
    println!("Received this value from JS: {value}");
}

extension!(
  hello_world,
  ops = [greet],
  esm = ["hello_world.js"],
);
