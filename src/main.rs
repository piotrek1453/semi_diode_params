use diode::Diode;
use plotters::prelude::*;

mod diode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let diode = Diode::default();

    // Voltage range for the plot (from 0V to 1V with increments of 0.01V)
    let voltages: Vec<f64> = (0..100).map(|v| v as f64 * 0.01).collect();
    let currents: Vec<f64> = voltages
        .iter()
        .map(|&v| {
            let diode_with_voltage = Diode {
                voltage: v,
                ..diode
            };
            diode_with_voltage.current().unwrap_or(0.0)
        })
        .collect();

    // Create a drawing area for the plot
    let root_area =
        BitMapBackend::new("diode_current_vs_voltage.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("f(Ud)=Id", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0.0..1.0,
            0.0..*currents
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
        )?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            voltages.iter().zip(currents.iter()).map(|(&v, &i)| (v, i)),
            RED,
        ))?
        .label("Diode Current [A]")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    println!("Plot saved to diode_current_vs_voltage.png");
    Ok(())
}
