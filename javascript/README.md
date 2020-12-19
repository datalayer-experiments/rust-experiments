[![Datalayer](https://raw.githubusercontent.com/datalayer/datalayer/main/res/logo/datalayer-25.svg?sanitize=true)](https://datalayer.io)

# Rust Javascript

- https://blog.logrocket.com/getting-started-with-webassembly-and-rust
- https://dev.to/jor/rust-wasm-browser-nodejs-2bo6
- https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
- https://github.com/rustwasm/wasm-pack

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
wasm-pack new hello-wasm
cd hello-wasm
wasm-pack
# Terminal 1
wasm-pack build --target web
# Terminal 2
python -m http.server 8000
open http://localhost:8000
```
