const wasm = require("./simple_wasm.wasm");

const result = wasm.add(4, 5);
console.log(result); // Output: 9
