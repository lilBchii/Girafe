use hound::WavReader;
use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

/// Transcribes the audio in the given file to a string of text.
pub fn transcribe(file: &Path, ctx: &WhisperContext) -> String {
    // Create a state for this transcription run
    let mut state = ctx.create_state().expect("failed to create state");

    // Sampling parameters for the model
    // TODO Configure differently?
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });

    params.set_translate(false);
    params.set_language(Some("fr"));
    // Disable any printing to stdout (this is what we get for a wrapper over C++!)
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    // Open the audio file (we've already guaranteed that this is mono f32 audio in 16kHz)
    let mut reader = WavReader::open(file).expect("failed to open audio file");
    let audio = whisper_rs::convert_integer_to_float_audio(
        &reader
            .samples::<i16>()
            .map(|s| s.expect("invalid sample"))
            .collect::<Vec<_>>(),
    );

    // Run the inference (this is blocking, and should be called in a blocking task)
    state.full(params, &audio[..]).expect("failed to run model");

    // Iterate through the segments of the transcript to extract the actual text
    let num_segments = state.full_n_segments().expect("failed to count segments");
    let mut segments = Vec::new();
    for i in 0..num_segments {
        let segment = state
            .full_get_segment_text(i)
            .expect("failed to write segment");
        segments.push(segment);
    }
    segments.join("")
}
