use js_sys::Int32Array;
// use std::fs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> i32;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[wasm_bindgen(start)]
// pub fn run() {
//     let mut arr = vec![];
//     let start = now();
//     // forloop_random(1_000_000_000);
//     bubble_sort_r(&mut arr);
//     let end = now();
//     log(&format!("{} s", ((end - start) as f64) / 1000.0));
// }

#[wasm_bindgen]
pub fn forloop_random(n: i64) {
    for _i in 0..n {
        rand::random::<i32>();
    }
}

#[wasm_bindgen]
pub fn bubble_sort(arr_input: &Int32Array) -> Int32Array {
    // let mut arr_vec: Vec<i32> = Vec::new();
    let mut arr_vec = vec![0; arr_input.length() as usize];
    arr_input.copy_to(&mut arr_vec[..]);
    // for i in 0..arr.length() {
    //     arr_vec.push(arr.get_index(i));
    // }
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

#[wasm_bindgen]
pub fn factorial(n: i32) -> i32 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// fn bubble_sort_r(v: &mut Vec<i32>) {
//     let mut tmp;
//     for i in 0..(v.len() - 1) {
//         for j in 0..(v.len() - i - 1) {
//             if v[j] > v[j + 1] {
//                 tmp = v[j];
//                 v[j] = v[j + 1];
//                 v[j + 1] = tmp;
//             }
//         }
//     }
// }

#[wasm_bindgen]
pub fn push_loop(n: i32) -> Int32Array {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..n {
        v.push(i);
    }

    Int32Array::from(&v[..])
}