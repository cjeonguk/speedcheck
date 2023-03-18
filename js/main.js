function forloopRandom(num, arr) {
  for (let i = 0; i < num; i++) {
    Math.random();
  }
  return true;
}

function bubbleSort(arr) {
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
  return arr;
}

function factorial(num) {
  if (num === 1) return 1;
  return num * factorial(num - 1);
}
// const fs = require('fs');

// const arr = fs.readFileSync(`${__dirname}/../random_arr.txt`, 'utf8').split(', ').map(Number);

const start = performance.now();

// console.log(factorial(8));

forloopRandom(1_000_000_000);

// bubbleSort(arr);

const end = performance.now();

console.log(`${(end - start) / 1000} s`);