#![no_std]

// PI constant (since we can't use std::f64::consts::PI)
const PI: f64 = 3.14159265358979323846264338327950288;

/// Calculate sine using Taylor series approximation
/// sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + x⁹/9! - ...
fn sin(x: f64) -> f64 {
    // Normalize x to [-π, π] range for better convergence
    let mut x = x;
    let two_pi = 2.0 * PI;

    // Reduce to [-π, π] using modulo
    x = x % two_pi;
    if x > PI {
        x -= two_pi;
    } else if x < -PI {
        x += two_pi;
    }

    // Use Taylor series (works well for x in [-π, π])
    sin_taylor(x)
}

/// Taylor series approximation for sin(x) where x is in [-π, π]
fn sin_taylor(x: f64) -> f64 {
    let x2 = x * x;
    let x3 = x * x2;
    let x5 = x3 * x2;
    let x7 = x5 * x2;
    let x9 = x7 * x2;
    let x11 = x9 * x2;
    let x13 = x11 * x2;

    // Taylor series: x - x³/3! + x⁵/5! - x⁷/7! + x⁹/9! - x¹¹/11! + x¹³/13!
    x - x3 / 6.0 + x5 / 120.0 - x7 / 5040.0 + x9 / 362880.0 - x11 / 39916800.0 + x13 / 6227020800.0
}

pub fn sin_val(frequency: f64, i: usize, sample_rate: f64, amplitude: f64) -> f64 {
    sin(2.0 * PI * frequency * i as f64 / sample_rate) * amplitude
}

/// Generate a sine wave into a mutable slice without using std
/// Fills the provided slice with sine wave values
pub fn generate_sine_wave(output: &mut [f64], frequency: f64, sample_rate: f64, amplitude: f64) {
    for (i, sample) in output.iter_mut().enumerate() {
        *sample = sin_val(frequency, i, sample_rate, amplitude);
    }
}

pub fn generate_square_wave(output: &mut [f64], amplitude: f64) {
    for (i, sample) in output.iter_mut().enumerate() {
        *sample = square_val(amplitude, i);
    }
}

pub fn generate_triangle_wave(output: &mut [f64], amplitude: f64) {
    for (i, sample) in output.iter_mut().enumerate() {
        *sample = triangle_val(amplitude, i);
    }
}

fn square_val(amplitude: f64, i: usize) -> f64 {
    if i % 2 == 0 {
        amplitude
    } else {
        -amplitude
    }
}

fn triangle_val(amplitude: f64, i: usize) -> f64 {
    if i % 2 == 0 {
        amplitude
    } else {
        -amplitude
    }
}
