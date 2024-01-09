use core::f32::consts::PI;

use easybench::bench_env;

use idsp::{atan2, cossin, iir, Filter, Lowpass, PLL, RPLL};

fn atan2_bench() {
    let xi = (10 << 16) as i32;
    let xf = xi as f32 / i32::MAX as f32;

    let yi = (-26_328 << 16) as i32;
    let yf = yi as f32 / i32::MAX as f32;

    println!(
        "atan2(yi, xi): {}",
        bench_env((yi, xi), |(yi, xi)| atan2(*yi, *xi))
    );
    println!(
        "yf.atan2(xf): {}",
        bench_env((yf, xf), |(yf, xf)| yf.atan2(*xf))
    );
}

fn cossin_bench() {
    let zi = -0x7304_2531_i32;
    let zf = zi as f32 / i32::MAX as f32 * PI;
    println!("cossin(zi): {}", bench_env(zi, |zi| cossin(*zi)));
    println!("zf.sin_cos(): {}", bench_env(zf, |zf| zf.sin_cos()));
}

fn rpll_bench() {
    let mut dut = RPLL::new(8);
    println!(
        "RPLL::update(Some(t), 21, 20): {}",
        bench_env(Some(0x241), |x| dut.update(*x, 21, 20))
    );
    println!(
        "RPLL::update(Some(t), sf, sp): {}",
        bench_env((Some(0x241), 21, 20), |(x, p, q)| dut.update(*x, *p, *q))
    );
}

fn pll_bench() {
    let mut dut = PLL::default();
    println!(
        "PLL::update(Some(t), 12, 12): {}",
        bench_env(Some(0x241), |x| dut.update(*x, 12))
    );
    println!(
        "PLL::update(Some(t), sf, sp): {}",
        bench_env((Some(0x241), 21), |(x, p)| dut.update(*x, *p))
    );
}

fn iir_int_bench() {
    let dut = iir::Biquad::default();
    let mut xy = [0; 4];
    println!(
        "int_iir::IIR::update(s, x): {}",
        bench_env(0x2832, |x| dut.update(&mut xy, *x))
    );
}

fn iir_f32_bench() {
    let dut = iir::Biquad::<f32>::default();
    let mut xy = [0.0; 4];
    println!(
        "int::IIR::<f32>::update(s, x): {}",
        bench_env(0.32241, |x| dut.update(&mut xy, *x))
    );
}

fn iir_f64_bench() {
    let dut = iir::Biquad::<f64>::default();
    let mut xy = [0.0; 4];
    println!(
        "int::IIR::<f64>::update(s, x): {}",
        bench_env(0.32241, |x| dut.update(&mut xy, *x))
    );
}

fn lowpass_bench() {
    let mut dut = Lowpass::<1>::default();
    println!(
        "Lowpass::<1>::update(x, k): {}",
        bench_env((0x32421, 14), |(x, k)| dut.update(*x, &[*k]))
    );
    println!(
        "Lowpass::<1>::update(x, 14): {}",
        bench_env(0x32421, |x| dut.update(*x, &[14]))
    );
}

fn main() {
    atan2_bench();
    cossin_bench();
    rpll_bench();
    pll_bench();
    iir_int_bench();
    iir_f32_bench();
    iir_f64_bench();
    lowpass_bench();
}
