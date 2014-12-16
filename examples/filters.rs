#![feature(unboxed_closures)]

extern crate synthrs;

use synthrs::synthesizer::{ make_samples, quantize_samples };
use synthrs::filter::{
    convolve, cutoff_from_frequency,
    lowpass_filter, highpass_filter, bandpass_filter, bandreject_filter
};
use synthrs::wave::SineWave;
use synthrs::writer::write_wav;


fn main() {
    // Lowpass/highpass filter convolution example
    let sample = make_samples(1.0, 44100, |t: f64| -> f64 {
        0.33 * (SineWave(6000.0)(t) + SineWave(700.0)(t) + SineWave(80.0)(t))
    });

    let lowpass = lowpass_filter(cutoff_from_frequency(400.0, 44100), 0.01);
    write_wav("out/lowpass.wav", 44100,
        quantize_samples::<i16>(sample.clone()) + &*quantize_samples::<i16>(convolve(lowpass, sample.clone()))
    ).ok().expect("failed");

    let highpass = highpass_filter(cutoff_from_frequency(2000.0, 44100), 0.01);
    write_wav("out/highpass.wav", 44100,
        quantize_samples::<i16>(sample.clone()) + &*quantize_samples::<i16>(convolve(highpass, sample.clone()))
    ).ok().expect("failed");

    let bandpass = bandpass_filter(
        cutoff_from_frequency(500.0, 44100),
        cutoff_from_frequency(3000.0, 44100),
        0.01
    );
    write_wav("out/bandpass.wav", 44100,
        quantize_samples::<i16>(sample.clone()) + &*quantize_samples::<i16>(convolve(bandpass, sample.clone()))
    ).ok().expect("failed");

    let bandreject = bandreject_filter(
        cutoff_from_frequency(400.0, 44100),
        cutoff_from_frequency(2000.0, 44100),
        0.01
    );
    write_wav("out/bandreject.wav", 44100,
        quantize_samples::<i16>(sample.clone()) + &*quantize_samples::<i16>(convolve(bandreject, sample.clone()))
    ).ok().expect("failed");
}
