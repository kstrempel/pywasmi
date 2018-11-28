import pywasmi_lib as lib


class WASMModule:

    def call(self, method):
        pass


    @classmethod
    def from_file(cls, path):
        with open(path, 'rb') as f:
            buffer = f.read()
        return cls.from_buffer(buffer)


    @staticmethod
    def from_buffer(buffer):
        id = lib.module_from_buffer(buffer)
        result = WASMModule()
        result._id = id
        return result   
        
    

        