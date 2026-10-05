// turnloop — the DEFAULT-IMPORT member shape: `import WebSocket from 'ws'`
// followed by `new WebSocket.Server({ server })` /
// `new WebSocket.WebSocketServer({ server })`.
//
// This is what real-world `ws` code overwhelmingly uses (e.g. nostr-filter's
// `new WebSocket.Server({ server, maxPayload })`), but the parity corpus only
// covered the named-import shape (`new WebSocketServer({ server })`, see
// test_gap_turnloop_ws_attached.ts). The member shape arrives in HIR as
// `NewDynamic { callee: PropertyGet { NativeModuleRef("ws"), "Server" } }`,
// which used to fall through to the runtime construct helper, read `Server`
// off the `ws` namespace object (undefined there), and throw "undefined is
// not a constructor". It now reroutes to the builtin server arm.
//
// Two attached servers are exercised sequentially on ephemeral ports: one via
// the `Server` alias, one via the `WebSocketServer` alias. Each gets a client
// echo roundtrip and a clean close. Nothing host-specific is printed
// (ephemeral ports are reduced to a boolean), so the output is fully
// deterministic — but the Node leg cannot execute this exact import shape:
// under `--experimental-strip-types` the `ws` ESM wrapper (`wrapper.mjs`)
// default-exports the bare client class without `.Server`, so Node exits
// non-zero and the harness falls back to the stored oracle in
// test-parity/expected/ (the NOTE path in run_parity_tests.sh). The sibling
// ws tests all use the named-import shape and run under real Node; the same
// event sequence here was additionally verified byte-identical against Node
// via CJS `require` (where `WS.Server` exists).
import http from 'node:http';
import WebSocket from 'ws';

function makeQueue() {
  const items: any[] = [];
  const waiters: any[] = [];
  return {
    push(item: any) {
      if (waiters.length > 0) {
        const w = waiters.shift();
        w(item);
      } else {
        items.push(item);
      }
    },
    take(): Promise<any> {
      return new Promise((resolve) => {
        if (items.length > 0) {
          resolve(items.shift());
          return;
        }
        let done = false;
        let timer: any = null;
        const fire = (v: any) => {
          if (done) return;
          done = true;
          if (timer !== null) clearTimeout(timer);
          resolve(v);
        };
        waiters.push(fire);
        timer = setTimeout(() => {
          const i = waiters.indexOf(fire);
          if (i >= 0) waiters.splice(i, 1);
          fire(null);
        }, 4000);
      });
    },
  };
}

async function runOne(alias: string): Promise<void> {
  const serverEvents = makeQueue();
  const clientEvents = makeQueue();
  const server = http.createServer((req: any, res: any) => {
    res.setHeader('Content-Type', 'text/plain');
    res.end('http:' + req.url);
  });

  // Direct member-new syntax only: `const ctor = WS.Server; new ctor(...)`
  // reads the namespace object value (undefined for class exports Perry
  // materializes at construction time) and is a separate followup. Both
  // aliases below must arrive as `PropertyGet { NativeModuleRef("ws"), _ }`
  // and reroute to the builtin server arm.
  let wss: any;
  if (alias === 'Server') {
    wss = new (WebSocket as any).Server({ server });
  } else {
    wss = new (WebSocket as any).WebSocketServer({ server });
  }
  wss.on('connection', (sock: any) => {
    serverEvents.push({ tag: 'connection' });
    sock.on('message', (data: any) => {
      serverEvents.push({ tag: 'message', text: data.toString() });
      sock.send('echo:' + data.toString());
    });
    sock.on('close', (code: any) => serverEvents.push({ tag: 'close', code }));
  });

  await new Promise<void>((r) => server.listen(0, '127.0.0.1', () => r()));
  const ad: any = server.address();
  const bound = typeof ad === 'object' && ad !== null ? ad.port : 0;
  console.log(alias + ' listening:', bound > 0);

  const ws: any = new (WebSocket as any)('ws://127.0.0.1:' + bound + '/x');
  ws.on('open', () => clientEvents.push({ tag: 'open' }));
  ws.on('message', (data: any) => clientEvents.push({ tag: 'echo', text: data.toString() }));
  ws.on('close', (code: any) => clientEvents.push({ tag: 'closed', code }));
  ws.on('error', (err: any) => clientEvents.push({ tag: 'error', message: String(err && err.message) }));

  const opened = await clientEvents.take();
  console.log(alias + ' open:', opened === null ? '(timeout)' : opened.tag);
  const conn = await serverEvents.take();
  console.log(alias + ' server:', conn === null ? '(timeout)' : conn.tag);

  ws.send('ping');
  const got = await serverEvents.take();
  console.log(alias + ' server message:', got === null ? '(timeout)' : got.text);
  const echo = await clientEvents.take();
  console.log(alias + ' client echo:', echo === null ? '(timeout)' : echo.text);

  ws.close(1000, 'done');
  const sclose = await serverEvents.take();
  console.log(alias + ' server close:', sclose === null ? '(timeout)' : sclose.code);
  const cclose = await clientEvents.take();
  console.log(alias + ' client close:', cclose === null ? '(timeout)' : cclose.code);

  await new Promise<void>((r) => wss.close(() => r()));
  await new Promise<void>((r) => server.close(() => r()));
  console.log(alias + ' closed');
}

async function main() {
  await runOne('Server');
  await runOne('WebSocketServer');
}

main();
