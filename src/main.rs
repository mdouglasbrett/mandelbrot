use mandelbrot::{calculate_mandelbrot, render_mandelbrot};

fn main() {
    calculate_mandelbrot();
    render_mandelbrot(vec![vec![20 as usize], vec![6 as usize], vec![423 as usize], vec![52 as usize]]);
}
