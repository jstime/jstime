import './b.mjs'

queueMicrotask(() => {
  console.log('a')
})