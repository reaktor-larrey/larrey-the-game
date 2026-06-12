default: web-dev

rs-dev:
    cargo build

rs-release:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --target web --out-dir ./web/src/lib/pkg --out-name "larrey" ./target/wasm32-unknown-unknown/release/larrey.wasm
    cp -r ./game/assets ./web/static

[working-directory('web')]
web-dev: rs-release
    npm install && npm run dev
