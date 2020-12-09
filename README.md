
# Setup
Just invoke cargo and it runs the example
```
cargo run
```

# Example output

```
$> cargo run -q
Hello from deno!
Got request body: Hello from Rust, headers: [object Object]
Dispatching request: Array([Object({"body": String("this is the patched body"), "headers": Object({"Content-Type": String("application/json")})})])
```

# Explanation
The first and second line come from the `flow_heater.js` deno runtime (via `Deno.core.print()`).

Function invocations (so-called `Ops`) from Deno to Rust are called with `Deno.core.jsonOpSync(function_name, arguments)`;

The Ops need to be registered in Rust on the `JsRuntime`. We use `json_ops` in conjunction with SERDE to use JSON as the serialization format for Ops arguments and return value.
