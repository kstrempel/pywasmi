from datetime import datetime

from pywasmi import WASMModule

def main(): 
    heigth = 100.0
    width = 200.0
    for x in range(0, int(heigth)):
        line = ''
        for y in range(0, int(width)):
            line += chr(instance.invoke("calc", (-2.5 + y * (2.0/heigth), -1.0 + x*(3.5/width))))
        print(line)

start = datetime.now()
module = WASMModule.from_file("mandel.wasm")
instance = module.create_instance()

main()
print(f"Time: {datetime.now() - start}")
