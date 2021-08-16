console.log('start b')

let b = await new Promise(resolve => {
  console.log('promise b')
  queueMicrotask(() => {
    console.log('microtask b')
    resolve('b')
  })
})

console.log('end b')

export default b
