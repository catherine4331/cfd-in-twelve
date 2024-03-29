use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    series::LineSeries,
    style::{RED, WHITE},
};

fn main() {
    let nx = 41; // Number of grid points
    let dx = 2.0 / (nx as f64 - 1.0); // Distance between points
    let timesteps = 25; // Total timesteps
    let dt = 0.025; // Timestep time delta
    let c = 1.0; // Wave speed

    // Initial condition grid
    let mut u = vec![1.0; nx];
    u[(0.5 / dx) as usize..(1.0 / dx) as usize].fill(2.0);

    // Run the linear convection method. We are using forward difference for the time derivative and backward difference for the space derivative
    for _ in 0..timesteps {
        let un = u.clone();
        for i in 1..nx {
            u[i] = un[i] - c * dt / dx * (un[i] - un[i - 1])
        }
    }

    plot(&u);
}

fn plot(data: &Vec<f64>) {
    let area = BitMapBackend::new("output/image.png", (1024, 768)).into_drawing_area();

    area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_2d(0.0_f64..2.0, 0.8_f64..2.2)
        .unwrap();

    let dx = 2.0 / data.len() as f64;

    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(i, u)| (i as f64 * dx, *u)),
            &RED,
        ))
        .unwrap();
}
