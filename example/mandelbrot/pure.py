from datetime import datetime

palette = [' ',' ',' ',' ','.','-','o','*','#','@','+']
max_iteration = 10

def calc(x0, y0):
    x = 0.0
    y = 0.0
    iteration = 0
    while x*x + y*y < 2*2 and iteration < max_iteration:
        xtemp = x*x - y*y + x0
        y = 2*x*y + y0
        x = xtemp
        iteration = iteration + 1

    return palette[iteration]

def main(): 
    heigth = 100.0
    width = 200.0
    for x in range(0, int(heigth)):
        line = ''
        for y in range(0, int(width)):
            line += calc(-2.5 + y * (2.0/heigth), -1.0 +(x*(3.5/width)))
        print(line)

start = datetime.now()
main()
print(f"Time: {datetime.now() - start}")