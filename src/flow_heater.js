Deno.core.print(`Hello from deno!\n`);

// invoke ops
Deno.core.ops();

// run the get_request function (provided by the surrounding rust ecosystem)
let request = Deno.core.jsonOpSync("get_request", []);

Deno.core.print(`Got request body: ${request.body}, headers: ${request.headers}\n`);

request.body = "this is the patched body";
request.headers['Content-Type'] = 'application/json';

Deno.core.jsonOpSync("dispatch_request", [request]);