from pywasmi.module import WASMModule


def test_wasmi_module_loading():
    wasm_module = WASMModule.from_file('tests/data/hello.wasm')
    assert wasm_module != None
    assert wasm_module._id != None
