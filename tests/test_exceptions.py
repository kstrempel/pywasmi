from pywasmi import WASMModule

def test_run_add_i32():
    wasm_module = WASMModule.from_file('tests/data/add_i32.wasm')
    wasm_instance = wasm_module.create_instance()
    assert wasm_instance != None
    result = wasm_instance.invoke("add", ('10', 20))
    assert 30 == result
