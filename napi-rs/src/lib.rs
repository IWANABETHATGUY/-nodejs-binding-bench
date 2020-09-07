#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;

use napi::{CallContext, Env, JsNumber, JsObject, Module, Result, Task};

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

register_module!(example, init);


fn init(module: &mut Module) -> Result<()> {
  module.create_named_method("fibonacci", fibonacci)?;
  Ok(())
}


#[js_function(1)] // ------> arguments length, omit for zero
fn fibonacci(ctx: CallContext) -> Result<JsNumber> {
  let n = ctx.get::<JsNumber>(0)?.try_into()?;
  ctx.env.create_int32(fibonacci_native(n))
}

#[inline]
fn fibonacci_native(n: i32) -> i32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci_native(n - 1) + fibonacci_native(n - 2),
  }
}