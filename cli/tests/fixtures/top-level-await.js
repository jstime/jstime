console.log(0);

await Promise.resolve();

queueMicrotask(() => {
  console.log(1);
});

await Promise.resolve();

console.log(2);
