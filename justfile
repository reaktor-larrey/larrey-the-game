default: web-dev

rs-dev:
    cargo build

rs-release:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --target web --out-dir ./larrey-game-web/src/assets/game --out-name "larrey" ./target/wasm32-unknown-unknown/release/larrey.wasm
    cp assets/* ./larrey-game-web/src/assets/game

[working-directory('larrey-game-web')]
web-dev: rs-release
    npm install && npm run dev
