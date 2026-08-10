**fix(ws): `wss.close()` now reaches `js_ws_server_close`, and client handles handed to user TS unbox to the real id instead of 0.**

`WebSocketServer.close()` was dispatched to the generic client-only `js_ws_close` entry — a server handle is never in `WS_CONNECTIONS`, so the call silently no-op'd and `WS_ACTIVE_SERVERS` kept the event loop alive forever. `perry-codegen`'s `NET_EVENTS_ROWS` now adds a `class_filter: Some("WebSocketServer")` row routing `close` to `js_ws_server_close`.

`js_ws_process_pending` (both `perry-ext-ws` and `perry-stdlib`) now NaN-boxes the numeric ws ids it passes to user `connection`/`message`/`close`/`client_error` handlers with POINTER_TAG, matching the `new WebSocket` ctor path — previously the raw f64 unboxed to `0` at the first method call site via the `unbox_to_i64` receiver contract, so `sock.send/.close` hit the wrong peer (or the client-only dispatcher).
