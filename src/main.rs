mod algebra;

use crate::algebra::Expression;
use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    series::LineSeries,
    style::{RED, WHITE},
};

const GRID_WIDTH: f64 = 2.0;

#[derive(Debug)]
struct RunConfig {
    nx: usize,      // Number of grid points
    dx: f64,        // Distance between points
    timesteps: i32, // Total timesteps
    dt: f64,        // Timestep time delta
}

impl RunConfig {
    fn new(nx: usize, total_distance: f64, timesteps: i32, sigma: f64) -> Self {
        let dx = total_distance / (nx as f64 - 1.0);
        Self {
            nx,
            dx,
            timesteps,
            dt: sigma * dx.powi(2) / 0.3,
        }
    }
}

fn main() {
    let conf = RunConfig::new(41, GRID_WIDTH, 20, 0.2);

    let a = Expression::Symbol("a".to_string());
    let b = Expression::Symbol("b".to_string());
    let c = a + b;

    println!("{:?}", conf);
    // Initial condition grid
    let mut u = vec![1.0; conf.nx];
    u[(0.5 / conf.dx) as usize..(1.0 / conf.dx) as usize].fill(2.0);

    diffusion(&conf, 0.3, &mut u);

    plot(&u);
}

// 1D Diffusion. We use central difference for the second derivative and forward difference for time.
fn diffusion(conf: &RunConfig, v: f64, n: &mut Vec<f64>) {
    for _ in 0..conf.timesteps {
        let un = n.clone();
        for i in 1..conf.nx - 1 {
            n[i] = un[i] + v * conf.dt / conf.dx.powi(2) * (un[i + 1] - 2.0 * un[i] + un[i - 1]);
        }
    }
}

// 1D nonlinear convection. The wavespeed is now u rather than being fixed. The same finite difference methods are used as for linear convection
fn nonlinear_convection(conf: &RunConfig, n: &mut Vec<f64>) {
    for _ in 0..conf.timesteps {
        let un = n.clone();
        for i in 1..conf.nx {
            n[i] = un[i] - un[i] * conf.dt / conf.dx * (un[i] - un[i - 1]);
        }
    }
}

// 1D linear convection. We are using forward difference for the time derivative and backward difference for the space derivative
fn linear_convection(conf: &RunConfig, c: f64, n: &mut Vec<f64>) {
    for _ in 0..conf.timesteps {
        let un = n.clone();
        for i in 1..conf.nx {
            n[i] = un[i] - c * conf.dt / conf.dx * (un[i] - un[i - 1])
        }
    }
}

fn plot(data: &Vec<f64>) {
    let area = BitMapBackend::new("output/image.png", (1024, 768)).into_drawing_area();

    area.fill(&WHITE).unwrap();

    println!("{:?}", data);

    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_2d(0.0_f64..2.0, 0.8_f64..2.2)
        .unwrap();

    let dx = GRID_WIDTH / data.len() as f64;

    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(i, u)| (i as f64 * dx, *u)),
            &RED,
        ))
        .unwrap();
}
