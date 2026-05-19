# Larrey the Game
Medibalance, but a game. Built in Rust using [Bevy](https://bevy.org/) game engine.

Try it online: https://reaktor-larrey.github.io/larrey-the-game/

## Roadmap
- [x] Basic single player mode
- [x] Add AI/Agent upgrades
- [x] Sound effects


## Development
Build and run in debug mode, for desktop:
`cargo run`

Build and test in the browser (you must install [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner) first):
```
cargo run --target wasm32-unknown-unknown
```

Build for release, browser:
```
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web --out-dir ./public --out-name "larrey" ./target/wasm32-unknown-unknown/release/larrey.wasm

npx serve public
```
