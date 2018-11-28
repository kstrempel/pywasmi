#[no_mangle]
pub extern "C" fn calc(x0 : f32, y0 : f32) -> char {
    let palette = vec![' ', ' ', ' ', ' ', '.', '-', 'o', '*', '#', '@', '+'];
    let max_iteration = 10;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut iteration = 0;
    while x*x + y*y < 2.0*2.0 && iteration < max_iteration {
        let xtemp = x*x - y*y + x0;
        y = 2.0*x*y + y0;
        x = xtemp;
        iteration = iteration + 1;
    }

    palette[iteration as usize]
}

fn main() {
    let heigth = 100;
    let width = 200;
    for x in 0..heigth {
        let mut line = String::new();
        for y in 0..width {
            line.push(calc(-2.5 + y as f32 * (2.0 / heigth as f32),
                           -1.0 +(x as f32 *(3.5 / width as f32 ))));
        }
        println!("{}", line);
    }
}
