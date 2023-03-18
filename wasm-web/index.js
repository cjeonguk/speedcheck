// const js = import("hello-wasm");
// js.then(js => {
//   // js.greet('WebAssembly');
//   const start = performance.now();

//   js.bf(100_000);
//   const end = performance.now();

//   console.log(`${(end - start) / 1000} s`);
// });

const start = performance.now();

for (let i = 0; i < 100000; i++) {
  console.log(i);
}

const end = performance.now();

console.log(`${(end - start) / 1000} s`);