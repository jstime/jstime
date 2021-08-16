console.log('start c')

let c = await new Promise(resolve => {
  console.log('promise c')
  queueMicrotask(() => {
    console.log('microtask c')
    resolve('c')
  })
})

console.log('end c')

export default c
