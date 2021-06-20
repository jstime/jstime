console.log(0);
queueMicrotask(() => {
  queueMicrotask(() => {
    console.log(4);
  });
  console.log(2);
});
queueMicrotask(() => {
  queueMicrotask(() => {
    console.log(5);
  });
  console.log(3);
});
console.log(1);
