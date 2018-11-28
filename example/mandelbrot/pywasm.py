from datetime import datetime

from pywasmi.module import WASMModule
from pywasmi.instance import WASMInstance

def main(): 
    heigth = 100.0
    width = 200.0
    for x in range(0, int(heigth)):
        line = ''
        for y in range(0, int(width)):
            line += chr(instance.invoke("calc", -2.5 + y * (2.0/heigth), -1.0 + x*(3.5/width)))
        print(line)

start = datetime.now()
module = WASMModule.from_file("mandel.wasm")
instance = WASMInstance.create(module)

main()
print(f"Time: {datetime.now() - start}")