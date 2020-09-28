const ffi = require('ffi')
const ref = require('ref')
const StructType = require('ref-struct')
const ArrayType = require('ref-array')

// Initialize the C-like array
const OutputArrayType = ArrayType(ref.types.int64, 2)

// Initialize the C-like data struct
const OutputType = StructType({
  result: ref.types.int64,
  operands: OutputArrayType,
  description: ref.types.CString
})

// Initialize the C-like data struct
const OperationType = StructType({
  operand_a: ref.types.int64,
  operand_b: ref.types.int64,
  result: ref.types.int64,
})

// Accessing the library
// See its signature https://github.com/node-ffi/node-ffi/wiki/Node-FFI-Tutorial#signature
const lib = ffi.Library('target/release/lib_node_rust', {
  hello: ['string', ['string']],
  multiply: [OutputType, [OperationType]],
})

process.stdout.write('\nCall the \'hello\' function:\n')

const stringToRust = 'Node.js'

// Call the library function "hello"
let stringFromRust = lib.hello(stringToRust)
console.log(stringFromRust)

process.stdout.write('\nCall the \'sum\' function:\n')

// Create the structure
const multiplication = new OperationType({ operand_a: 50, operand_b: -5 })

// Call the library function "multiply"
const result = lib.multiply(multiplication)

console.log(result.result)
console.log(result.operands.toArray())
console.log(result.description)

// Print as an object
console.dir(result.toObject())
