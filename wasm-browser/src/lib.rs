use js_sys::Int32Array;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::Response;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// #[wasm_bindgen(start)]
// pub fn run() {
//     let mut arr = vec![];
//     let start = now();
//     // forloop_random(1_000_000_000);
//     bubble_sort_r(&mut arr);
//     let end = now();
//     log(&format!("{} s", ((end - start) as f64) / 1000.0));
// }

// #[wasm_bindgen]
// pub fn forloop_random(n: i32) {
//     for i in 0..n {
//         log(&i.to_string());
//     }
// }

// #[wasm_bindgen]
// pub fn dom_load() {
//     let window = web_sys::window().unwrap();
//     let document = window.document().expect("should have a document on window");
// }

#[wasm_bindgen]
pub fn bubble_sort(arr: &Int32Array) -> Int32Array {
    let mut arr_vec: Vec<i32> = Vec::new();
    for i in 0..arr.length() {
        arr_vec.push(arr.get_index(i));
    }
    let mut tmp;
    for i in 0..(arr_vec.len() - 1) {
        for j in 0..(arr_vec.len() - i - 1) {
            if arr_vec[j] > arr_vec[j + 1] {
                tmp = arr_vec[j];
                arr_vec[j] = arr_vec[j + 1];
                arr_vec[j + 1] = tmp;
            }
        }
    }
    Int32Array::from(&arr_vec[..])
}
