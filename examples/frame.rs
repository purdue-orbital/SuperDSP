use std::thread::{sleep, spawn};
use std::time::Duration;
use num_complex::Complex;
use rand::random;
use rustdsp::ui::charts::builder::WindowBuilder;
use rustdsp::ui::charts::line_chart::LineChart;

fn main() {

    let mut win = WindowBuilder::new();

    let line_chart = LineChart::new(100);

    let mut boxxed = win.add_chart(line_chart);

    spawn(move || {
        loop{
            sleep(Duration::from_millis(10));

            let re = random();
            let im = random();

            boxxed.add(Complex::new(re,im));
        }
    });

    win.build();
}