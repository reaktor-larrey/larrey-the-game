default: web-dev

rs-dev:
    cargo build

rs-release:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --target web --out-dir ./larrey-game-web/src/lib/pkg --out-name "larrey" ./target/wasm32-unknown-unknown/release/larrey.wasm
    cp -r assets ./larrey-game-web/static

[working-directory('larrey-game-web')]
web-dev: rs-release
    npm install && npm run dev
