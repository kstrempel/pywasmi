# PYWASMI #

[![Build Status](https://dev.azure.com/kstrempel/kstrempel/_apis/build/status/kstrempel.pywasmi)](https://dev.azure.com/kstrempel/kstrempel/_build/latest?definitionId=1)

Calling WASM from python using the WASM interpreter from [parity.io](https://github.com/paritytech/wasmi).

## USING IT ##

This package contains two python classes ( developed in rust using [pyo3](https://github.com/PyO3/) ).

- pywasmi.WASMModule
- pywasmi.WASMInstance

To create a module you need a wasm file or a binary loaded wasm buffer.

e.g:

```
from pywasmi import WASMModule

wasm_module = WASMModule.from_file('tests/data/add_f32.wasm')
wasm_instance = wasm_module.create_instance()
result = wasm_instance.invoke("add", (10.5, 21.0))
assert 31.5 == result
```


## PLANNING ###

* [x] Read a WASM binary,
* [x] Instanciate a WASM binary,
* [x] Invoke function:
  * [x] with `i32` as arguments or returned value,
  * [x] with `i64` as arguments or returned value,
  * [x] with `f32` as arguments or returned value,
  * [x] with `f64` as arguments or returned value.
*  [] Invoke function with ```__call__```
* [ ] Expose memory:
  * [ ] Readable array view,
  * [ ] Writable array view.
