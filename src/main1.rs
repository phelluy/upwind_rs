// command line:
// time RAYON_NUM_THREADS=8 cargo run --release
use rayon::prelude::*;

//extern crate faster;
//use faster::*;

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

fn main() {
    println!("Init...");
    let nx = 100;

    let dx = L / nx as f64;

    let xc: Vec<f64> = (0..nx + 1).map(|i| i as f64 * dx).collect();

    let mut un: Vec<f64> = xc.par_iter().map(|x| exact_sol(*x, 0.)).collect();
    let mut unp1 = un.clone();

    let tmax = 0.6;

    let cfl = 0.8;

    let dt = dx / C.abs() * cfl;

    let mut t = 0.;

    println!("Calcul...");
    while t < tmax {
        // original loop
        // for i in 0..nx {
        //     un[i] = un[i] - C * dt / dx * (un[i + 1] - un[i]);
        // }

        // does not work: immutable borrow
        // for (u0,u1) in un.iter_mut().zip(un.iter().skip(1)) {
        //     *u0 = *u0 - C * dt / dx * (*u1 - *u0);
        // }

        // correct solution with a for loop
        // for ((u1, u0), v0) in unp1.iter_mut().zip(un.iter()).zip(un.iter().skip(1)) {
        //     *u1 = *u0 - C * dt / dx * (*v0 - *u0);
        // }

        // parallel loop
        unp1.par_iter_mut()
            .zip(un.par_iter())
            .zip(un.par_iter().skip(1)) // skip the first element: shifted vector
            .for_each(|((u1, u0), v0)| *u1 = *u0 - C * dt / dx * (*v0 - *u0));

        t = t + dt;
        unp1[nx] = exact_sol(xc[0], t);

        un.par_iter_mut()
            .zip(unp1.par_iter())
            .for_each(|(u0, u1)| *u0 = *u1);

        //println!("t={}, dt={}", t, dt);
    }

    println!("Sauve...");
    sauv_sol(t, xc, un);
    // cannot use xc anymore: xc has been moved.
    //println!("xc={:?}",xc);
}

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn sauv_sol(t: f64, xc: Vec<f64>, un: Vec<f64>) {
    let meshfile = File::create("trans.dat").unwrap();
    let mut meshfile = BufWriter::new(meshfile); // create a buffer for faster writes...

    un.iter().zip(xc.iter()).for_each(|(u, x)| {
        let uex = exact_sol(*x, t);
        writeln!(meshfile, "{} {} {}", *x, *u, uex).unwrap();
    });
}
