use std::f32::consts::PI;
use num_complex::Complex;
use plotters::prelude::*;
use rustdsp::common::constellation::Constellation;
use rustdsp::common::generate_wave::generate_wave;
use rustdsp::Modulators;

fn main() {
    let modulators = Modulators::new(10,10000.0);
    let bpsk_signal = modulators.bpsk(&[138]);

    let reference = generate_wave(
        16e3,
        10000.0,
        40,
        0,
        1.0,
        0.0,
        0.0,
        0.0
    );

    let mut to_constellation_map = vec![];

    for index in (0..reference.len()).step_by(10){
        let mut map = Constellation::calculate_phase_offset(&reference[index..index+10],&bpsk_signal[index..index+10]);

        map.0 = map.0.cos();
        map.1 = map.1.cos();

        to_constellation_map.push(map);
    }
    draw_constellation_map(to_constellation_map.as_slice());

    draw_complex(bpsk_signal.as_slice());
}

pub fn draw_constellation_map(arr: &[(f32,f32)]) -> Result<(), Box<dyn std::error::Error>>{
    let root = BitMapBackend::new("map.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Constellation Map", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(PointSeries::of_element(
            arr.to_vec(),
            arr.len() as u32,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },

        ))?
        .label("Constellation")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

pub fn draw_complex(arr: &[Complex<f32>]) -> Result<(), Box<dyn std::error::Error>> {
    let real: Vec<(usize,f32)> = arr.iter().map(|&b| b.re).enumerate().collect();
    let imag: Vec<(usize,f32)> =  arr.iter().map(|&b| b.im).enumerate().collect();

    let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Complex Graph", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..arr.len(), -1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            real,
            &RED,
        ))?
        .label("I")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            imag,
            &BLUE,
        ))?
        .label("Q")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
