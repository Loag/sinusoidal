# sinusodial

tooling for generating wave form data

## Running

```bash
cargo run -- --frequency 440.0 --sample-rate 44100.0 --amplitude 1.0 --samples 1000 --waveform sine
```

Or using short flags:
```bash
cargo run -- -f 440.0 -s 44100.0 -a 1.0 --samples 1000 -w sine
```

Available waveforms: `sine`, `square`, `triangle`