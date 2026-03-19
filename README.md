Build and run in debug mode, for desktop:
`cargo run`

Build and run in browser (you must install [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner) first):
`cargo run --target wasm32-unknown-unknown`

Build for release, browser:
```
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web --out-dir ./public --out-name "larrey" ./target/wasm32-unknown-unknown/release/larrey.wasm

npx serve public
```
