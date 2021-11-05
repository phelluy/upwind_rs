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


fn main()  {
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

    let mut t = 0.;

    //sauv_sol(t, xc, un, "trans0.dat");

    let tmax = 0.;

    let cfl = 0.8;

    let dt = dx / C.abs() * cfl;


    while t < tmax {
        for i in 0..nx {
            un[i] = un[i] - C * dt / dx * (un[i + 1] - un[i]);
        }
        t +=  dt;
        un[nx] = exact_sol(xc[nx], t);

        println!("t={}, dt={}", t, dt);
    }


    sauv_sol(t, xc, un, "trans1.dat");

}

use std::fs::File;
use std::io::Write;

fn sauv_sol(t: f64, xc: Vec<f64>, un: Vec<f64>, filename: &str) {
    let mut meshfile = File::create(filename).unwrap();
    let nx = xc.len();
    for i in 0..nx  {
        let uex = exact_sol(xc[i], t);
        let u = un[i];
        writeln!(meshfile, "{} {} {}", xc[i], u, uex).unwrap();
    }}
