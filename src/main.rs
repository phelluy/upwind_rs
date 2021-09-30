use std::fs::File;
use std::io::BufWriter;
use std::io::{Error, Write};

const C: f64 = 1.;

const L: f64 = 1.;

const PI: f64 = std::f64::consts::PI;

fn exact_sol(x: f64, t: f64) -> f64 {
    let y = (2. * PI * (x - C * t)).sin();
    y
}

fn main() -> Result<(), Error> {
    let nx = 100;

    let dx = L / nx as f64;

    let mut un = vec![0.; nx + 2];

    let mut xc = vec![0.; nx + 2];

    for i in 0..nx + 2 {
        xc[i] = i as f64 * dx - dx / 2.;
    }

    println!("{:?}", xc);

    for i in 0..nx + 2 {
        un[i] = exact_sol(xc[i], 0.);
    }

    let tmax = 0.5;

    let cfl = 0.8;

    let dt = dx / C * cfl;

    let mut t = 0.;

    while t < tmax {
        for i in 1..nx + 1 {
            un[i] = un[i] - C * dt / dx * (un[i] - un[i - 1]);
        }

        t = t + dt;
        un[0] = exact_sol(xc[0], t);

        println!("t={}, dt={}", t, dt);
    }

    let meshfile = File::create("trans.dat")?;
    let mut meshfile = BufWriter::new(meshfile); // create a buffer for faster writes...

    for i in 0..nx + 2 {
        let uex = exact_sol(xc[i],t);
        let u = un[i];
        writeln!(meshfile, "{} {} {}", xc[i], u, uex)?;
    }

    Ok(())
}
