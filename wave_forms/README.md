# wave_forms

A `no_std` Rust library for generating waveform data. This library provides standard-library-free implementations of waveforms.

## Features

- **No standard library** (`#![no_std]`) - suitable for embedded systems and constrained environments
- **Sine wave generation** - using Taylor series approximation
- **Square wave generation** - alternating high/low values
- **Triangle wave generation** - linear ramp waveform
- **Zero dependencies** - pure Rust implementation

### Generating a Sine Wave

```rust
use wave_forms::generate_sine_wave;

let mut buffer = [0.0; 1000];
generate_sine_wave(&mut buffer, 440.0, 44100.0, 1.0);
// buffer now contains 1000 samples of a 440Hz sine wave
```

### Generating a Square Wave

```rust
use wave_forms::generate_square_wave;

let mut buffer = [0.0; 1000];
generate_square_wave(&mut buffer, 1.0);
// buffer now contains 1000 samples of a square wave
```

### Generating a Triangle Wave

```rust
use wave_forms::generate_triangle_wave;

let mut buffer = [0.0; 1000];
generate_triangle_wave(&mut buffer, 1.0);
// buffer now contains 1000 samples of a triangle wave
```

### Single Sample Calculation

For more control, you can calculate individual samples:

```rust
use wave_forms::sin_val;

let sample = sin_val(440.0, 0, 44100.0, 1.0);
// Calculate the first sample of a 440Hz sine wave
```

## API Reference

### Functions

#### `generate_sine_wave(output: &mut [f64], frequency: f64, sample_rate: f64, amplitude: f64)`

Generates a sine wave into the provided mutable slice.

- `output`: Mutable slice to fill with waveform samples
- `frequency`: Frequency in Hz
- `sample_rate`: Sample rate in Hz (e.g., 44100.0)
- `amplitude`: Amplitude of the waveform

#### `generate_square_wave(output: &mut [f64], amplitude: f64)`

Generates a square wave into the provided mutable slice.

- `output`: Mutable slice to fill with waveform samples
- `amplitude`: Amplitude of the waveform

#### `generate_triangle_wave(output: &mut [f64], amplitude: f64)`

Generates a triangle wave into the provided mutable slice.

- `output`: Mutable slice to fill with waveform samples
- `amplitude`: Amplitude of the waveform

#### `sin_val(frequency: f64, i: usize, sample_rate: f64, amplitude: f64) -> f64`

Calculates a single sine wave sample value.

- `frequency`: Frequency in Hz
- `i`: Sample index
- `sample_rate`: Sample rate in Hz
- `amplitude`: Amplitude of the waveform
- Returns: The sample value at index `i`

## Implementation Details

### Sine Wave

The sine wave implementation uses a Taylor series approximation:

```
sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + x⁹/9! - x¹¹/11! + x¹³/13!
```

The input is normalized to the range [-π, π] for optimal convergence of the series.

## License

MIT

