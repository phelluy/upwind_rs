const C: f64 = -1.;

const L: f64 = 1.;

fn peak(x: f64) -> f64 {
    let r2 = x * x;
    let eps = 0.1;
    let eps2 = eps * eps;
    if r2 / eps2 < 1. {
        (1. - r2 / eps2).powi(4)
    } else {
        0.
    }
}

fn exact_sol(x: f64, t: f64) -> f64 {
    peak(x - C * t - 0.8)
}

use std::io::Error;

fn main() -> Result<(), Error> {
    let nx = 1000;

    let dx = L / nx as f64;

    let mut un = vec![0.; nx + 1];

    let mut xc = vec![0.; nx + 1];

    for i in 0..nx + 1 {
        xc[i] = i as f64 * dx;
    }

    for i in 0..nx + 1 {
        un[i] = exact_sol(xc[i], 0.);
    }

    let tmax = 0.6;

    let cfl = 0.8;

    let dt = dx / C.abs() * cfl;

    let mut t = 0.;

    while t < tmax {
        for i in 0..nx {
            un[i] = un[i] - C * dt / dx * (un[i + 1] - un[i]);
        }
        t = t + dt;
        un[nx] = exact_sol(xc[0], t);

        println!("t={}, dt={}", t, dt);
    }

    use std::fs::File;
    use std::io::{Write};

    let mut meshfile = File::create("trans.dat")?;
    //let mut meshfile = BufWriter::new(meshfile); // create a buffer for faster writes...

    for i in 0..nx + 1 {
        let uex = exact_sol(xc[i], t);
        let u = un[i];
        writeln!(meshfile, "{} {} {}", xc[i], u, uex)?;
    }

    Ok(())
}
