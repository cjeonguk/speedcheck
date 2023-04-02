// const Benchmark = require('benchmark');
// const wasm = require('wasm');
// import init, {bubble_sort} from "../wasm-browser/pkg/wasm_browser.js";
import init, {bubble_sort, push_loop} from "./pkg/wasm_browser.js";


async function run() {
  await init();

  const suite = new Benchmark.Suite('Bubble sort');
  const suite2 = new Benchmark.Suite('Push loop');
  
  // const fs = require('fs');
  
  // const arr = fs.readFileSync(`${__dirname}/../random_arr.txt`, 'utf8').split(', ').map(Number);

  const arr = [];
  const arr2 = [];
  
  for (let i = 10_000; i > 0; i--) {
    arr.push(i);
    arr2.push(i);
    // console.log(i);
  }
  
  console.log('Suite 1');


  suite.add('JS', () => {
    let tmp;
    for (let i = 0; i < arr.length - 1; i++) {
      for (let j = 0; j < arr.length - i - 1; j++) {
        if (arr[j] > arr[j + 1]) {
          tmp = arr[j];
          arr[j] = arr[j + 1];
          arr[j + 1] = tmp;
        }
      }
    }
  }).add('WASM', () => {
    // wasm.bubble_sort(arr);
    bubble_sort(arr);
  }).on('cycle', (event) => {
    console.log(String(event.target));
  }).on('complete', (event) => {
    const suite = event.currentTarget;
    const fastestOption = suite.filter('fastest').map('name');
  
    console.log(`Fastest is ${fastestOption}`);
    // console.log('Fastest is ' + this.filter('fastest').map('name'));
  }).run()

  console.log('Suite 2');


  suite2.add('JS', () => {
    const arr3 = [];
    for (let i = 0; i < 1_000_000; i++) {
      arr3.push(i);
      // console.log(i);
    }
  }).add('WASM', () => {
    push_loop(1_000_000);
    // bubble_sort(arr);
  }).on('cycle', (event) => {
    console.log(String(event.target));
  }).on('complete', (event) => {
    const suite = event.currentTarget;
    const fastestOption = suite.filter('fastest').map('name');
  
    console.log(`Fastest is ${fastestOption}`);
    // console.log('Fastest is ' + this.filter('fastest').map('name'));
  }).run()

}

run();