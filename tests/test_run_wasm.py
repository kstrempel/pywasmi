import pytest

from pywasmi.module import WASMModule
from pywasmi.instance import WASMInstance


@pytest.fixture
def wasm_instance():
    wasm_module = WASMModule.from_file('tests/data/hello.wasm')
    wasm_instance = WASMInstance.create(wasm_module)

    return wasm_instance


def test_run_simple_method(wasm_instance):
    assert wasm_instance != None

