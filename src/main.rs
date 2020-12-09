use deno_core::error::AnyError;
use deno_core::JsRuntime;
use deno_core::OpState;
use deno_core::ZeroCopyBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio;

#[derive(Serialize, Deserialize)]
struct Request {
    body: String,
    headers: HashMap<String, String>,
}

fn op_get_request(
    _state: &mut OpState,
    _args: Value,
    _bufs: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
    let body = "Hello from Rust".to_string();
    Ok(serde_json::json!(Request {
        body,
        headers: HashMap::new()
    }))
}

fn op_dispatch_request(
    _state: &mut OpState,
    args: Value,
    _bufs: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
    println!("Dispatching request: {:?}", args);
    Ok(serde_json::json!(()))
}

fn main() {
    let mut js_runtime = JsRuntime::new(Default::default());

    // let js = "console.log('Hello, World');";

    // let x = js_runtime.execute("index.js", js).unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    js_runtime.register_op(
        "dispatch_request",
        deno_core::json_op_sync(op_dispatch_request),
    );
    js_runtime.register_op("get_request", deno_core::json_op_sync(op_get_request));

    let future = async move {
        js_runtime
            .execute("flow_heater.js", include_str!("flow_heater.js"))
            .unwrap();
        js_runtime.run_event_loop().await
    };

    runtime.block_on(future).unwrap();
}
