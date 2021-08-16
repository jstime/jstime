import b from './b.mjs'

console.log('start a')

console.log(b)

let a = await new Promise(resolve => {
  console.log('promise a')
  queueMicrotask(() => {
    console.log('microtask a')
    resolve('a')
  })
})

console.log('end a')

export default a
