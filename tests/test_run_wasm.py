from pywasmi import WASMModule


def test_run_simple_method_i32():
    wasm_module = WASMModule.from_file('tests/data/test_i32.wasm')
    wasm_instance = wasm_module.create_instance()
    assert wasm_instance != None
    result = wasm_instance.invoke("test")
    assert 1337 == result

def test_run_simple_method_f32():
    wasm_module = WASMModule.from_file('tests/data/test_f32.wasm')
    wasm_instance = wasm_module.create_instance()
    assert wasm_instance != None
    result = wasm_instance.invoke("test")
    assert 1337.5 == result

def test_run_add_i32():
    wasm_module = WASMModule.from_file('tests/data/add_i32.wasm')
    wasm_instance = wasm_module.create_instance()
    assert wasm_instance != None
    result = wasm_instance.invoke("add", 10, 20)
    assert 30 == result

def test_run_add_i64():
    wasm_module = WASMModule.from_file('tests/data/add_i64.wasm')
    wasm_instance = wasm_module.create_instance()
    assert wasm_instance != None
    result = wasm_instance.invoke("add", 10, 20)
    assert 30 == result

def test_run_add_f32(): 
    wasm_module = WASMModule.from_file('tests/data/add_f32.wasm')
    wasm_instance = wasm_module.create_instance()
    assert wasm_instance != None
    result = wasm_instance.invoke("add", 10.5, 21.0)
    assert 31.5 == result

def test_run_add_f64():
    wasm_module = WASMModule.from_file('tests/data/add_f64.wasm')
    wasm_instance = wasm_module.create_instance()
    assert wasm_instance != None
    result = wasm_instance.invoke("add", 10.5, 21.0)
    assert 31.5 == result

def test_run_add_fact():
    wasm_module = WASMModule.from_file('tests/data/fact.wasm')
    wasm_instance = wasm_module.create_instance()
    assert wasm_instance != None
    result = wasm_instance.invoke("fac", 20)
    assert 2432902008176640000 == result