use clap::{Parser, ValueEnum};
use std::io::{self, Write};

use wave_forms::{generate_sine_wave, generate_square_wave, generate_triangle_wave};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    frequency: f64,
    #[clap(short = 'r', long)]
    sample_rate: f64,
    #[clap(short, long)]
    amplitude: f64,
    #[clap(short, long)]
    samples: usize,
    #[clap(short, long)]
    waveform: Waveform,
    #[clap(short, long)]
    output: Option<String>,
}

#[derive(ValueEnum, Clone, Copy)]
enum Waveform {
    Sine,
    Square,
    Triangle,
}

fn main() {
    let args = Args::parse();
    let mut output = vec![0.0; args.samples];
    match args.waveform {
        Waveform::Sine => generate_sine_wave(
            &mut output,
            args.frequency,
            args.sample_rate,
            args.amplitude,
        ),
        Waveform::Square => generate_square_wave(&mut output, args.amplitude),
        Waveform::Triangle => generate_triangle_wave(&mut output, args.amplitude),
    };

    if let Some(output_file) = args.output {
        let mut file = std::fs::File::create(output_file).expect("Failed to create file");
        for sample in output {
            writeln!(file, "{}", sample).expect("Failed to write to file");
        }
    } else {
        for sample in output {
            writeln!(io::stdout(), "{}", sample).expect("Failed to write to stdout");
        }
    }
}
