/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly main: (a: number, b: number) => number;
    readonly wasm_bindgen__convert__closures_____invoke__hf0794128ad5926e9: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h14af77eef5f80588: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h10412c038c2a9f7e: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h10412c038c2a9f7e_3: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h10412c038c2a9f7e_4: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h10412c038c2a9f7e_5: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h10412c038c2a9f7e_6: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h10412c038c2a9f7e_7: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h10412c038c2a9f7e_8: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h10412c038c2a9f7e_9: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0a448e3853c2adcc: (a: number, b: number, c: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hd5ae6fb4260f5c2e: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hd15b78d8ba46e319: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
