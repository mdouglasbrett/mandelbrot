use num::complex::Complex;

fn mandelbrot_at_point() {}

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
