const addon = require('./index')

function fibonacci40() {
  addon.fibonacci(40)
}

function fibonacci30() {
  addon.fibonacci(30)
}
const Benchmark = require('benchmark')

const suite = new Benchmark.Suite()

// 添加测试
suite
  .add('fibonacci30', fibonacci30)
  .add('fibonacci40', fibonacci40)
  // add listeners
  .on('cycle', function (event) {
    console.log(String(event.target))
  })
  // run async
  .run({ async: true })
