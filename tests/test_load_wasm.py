from pywasmi import WASMModule


def test_wasmi_module_loading():
    wasm_module = WASMModule.from_file('tests/data/test_i32.wasm')
    assert wasm_module != None
