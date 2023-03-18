// use std::fs;
use std::time::Instant;

fn main() {
    // let mut arr: Vec<i32> = fs::read_to_string("/home/woora/project/speedcheck/random_arr.txt")
    //     .expect("ERROR")
    //     .split(", ")
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .collect();

    let now = Instant::now();

    // bubble_sort(&mut arr);
    forloop_random(1_000_000_000);

    println!("{} s", now.elapsed().as_secs_f64());

    // println!("{:?}", arr);
}

fn forloop_random(n: i64) {
    for _i in 0..n {
        rand::random::<i32>();
    }
}

// fn bubble_sort(v: &mut Vec<i32>) {
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

// fn factorial(n: i32) -> i32 {
//     if n == 1 {
//         1
//     } else {
//         n * factorial(n - 1)
//     }
// }
