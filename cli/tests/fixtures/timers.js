console.log('Start');
let count = 0;
setTimeout(() => {
  console.log('Timeout 1');
}, 50);
setTimeout(() => {
  console.log('Timeout 2');
}, 20);
const intervalId = setInterval(() => {
  count++;
  console.log(`Interval ${count}`);
  if (count >= 2) {
    clearInterval(intervalId);
  }
}, 30);
console.log('End');
