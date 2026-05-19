default: web-dev

rs-dev:
    cargo build

rs-release:
    cargo build --release --target wasm32-unknown-unknown

web-dev: rs-release
    wasm-bindgen --no-typescript --target web --out-dir ./lib --out-name "larrey" ./target/wasm32-unknown-unknown/release/larrey.wasm
    cd larrey-game-web
    npm install && npm run dev

# build for web
web-release:
    mkdir -p public
    cp src-web/*.html public
    cp -r assets public
    wasm-bindgen --no-typescript --target web --out-dir ./public --out-name "larrey" ./target/wasm32-unknown-unknown/release/larrey.wasm
    npx serve public
