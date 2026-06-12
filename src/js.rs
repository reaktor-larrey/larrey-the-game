use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "$lib/pkg/external")]
extern "C" {
    pub fn submit_score(score: usize);
}
