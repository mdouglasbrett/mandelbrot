use num::complex::Complex;

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            // has escaped the set
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

pub fn calculate_mandelbrot() {
    println!("Calculating!")
}

pub fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                2..=5 => '.',
                5..=10 => '😊',
                11..=30 => '*',
                30..=100 => '+',
                100..=200 => 'X',
                200..=400 => '$',
                400..=700 => '#',
                _ => '🙃'
            };

            line.push(val);
        }
        println!("{}", line);
    }
}
