use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, StreamConfig,
};
use std::path::Path;
//use tokio::sync::oneshot::Receiver;

pub fn start_recording(path: &Path) {
    let host = cpal::default_host();
    let input_device = host
        .default_input_device()
        .expect("failed to find input device");
    let config = StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(16_000),
        buffer_size: cpal::BufferSize::Default,
    };
    let dflt_config = input_device
        .default_input_config()
        .expect("failed to find config");

    // Initialize the WAV writer
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16_000,
        bits_per_sample: (dflt_config.sample_format().sample_size() * 8) as u16,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(path, spec).expect("failed to open writer");

    // Initialize the CPAL audio input stream
    let input_stream = input_device
        .build_input_stream(
            &config,
            move |data: &[f32], _| {
                // Callback function to receive audio data
                for sample in data {
                    if let Err(err) = writer.write_sample(*sample) {
                        eprintln!("Error writing audio data to WAV file: {err:?}");
                    }
                }
            },
            |err| {
                // Error callback
                eprintln!("Error in audio stream: {:?}", err);
            },
            None,
        )
        .expect("failed to initialize cpal input stream");

    // Start the audio stream
    input_stream.play().expect("failed to start audio");

    // Stop and close the audio stream
    drop(input_stream);
}
