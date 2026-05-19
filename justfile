# build for web
web:
    mkdir -p public
    cp src-web/*.html public
    cp -r assets public
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --target web --out-dir ./public --out-name "larrey" ./target/wasm32-unknown-unknown/release/larrey.wasm
    npx serve public
