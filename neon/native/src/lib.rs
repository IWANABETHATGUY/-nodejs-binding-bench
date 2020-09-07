use neon::{context::{Context, FunctionContext}, result::JsResult, types::JsNumber, register_module};

fn fibonacci(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let arg0 = cx.argument::<JsNumber>(0)?.value();
    Ok(cx.number(_fibonacci(arg0 as i32)))
}
#[inline]
fn _fibonacci(n: i32 ) -> i32 {
    match n {
        1 | 2 => 1,
        _ => _fibonacci(n - 1) + _fibonacci(n - 2),
    }
}
register_module!(mut cx, { cx.export_function("fibonacci", fibonacci) });
