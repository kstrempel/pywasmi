import pywasmi.pywasmi_lib as lib


class WASMInstance:

    @staticmethod
    def create(wasm_module):
        module_id = wasm_module._id
        id = lib.create_instance(module_id)
        result = WASMInstance()
        result._id = id
        
        return result

    def invoke(self, method):
        return lib.invoke_export(self._id, method)