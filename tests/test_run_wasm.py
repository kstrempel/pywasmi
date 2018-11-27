import pytest

from pywasmi.module import WASMModule
from pywasmi.instance import WASMInstance


def test_run_simple_method_i32():
    wasm_module = WASMModule.from_file('tests/data/test_i32.wasm')
    wasm_instance = WASMInstance.create(wasm_module)
    assert wasm_instance != None
    result = wasm_instance.invoke("test")
    assert 1337 == result

def test_run_simple_method_f32():
    wasm_module = WASMModule.from_file('tests/data/test_f32.wasm')
    wasm_instance = WASMInstance.create(wasm_module)
    assert wasm_instance != None
    result = wasm_instance.invoke("test")
    assert 1337.5 == result

