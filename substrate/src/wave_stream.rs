use crate::AudioStreamSource;
use cpal::traits::{DeviceTrait, StreamTrait};
use hyphae::configuration::Configuration;

pub struct WaveStream {
    audio_device: cpal::Device,
    stream_configuration: cpal::StreamConfig,
    audio_stream: std::cell::RefCell<Option<cpal::Stream>>,
    audio_stream_created: std::time::Instant,
    buffer_size: usize,
    wave_data_receiver: single_value_channel::Receiver<Vec<f32>>,
    frequency_data_receiver: single_value_channel::Receiver<Vec<f32>>,
}

impl WaveStream {
    pub fn new(stream_source: AudioStreamSource, configuration: Configuration) -> Self {
        let (audio_device, stream_configuration) =
            stream_source.get_audio_device_and_stream_configuration();
        let buffer_size = configuration.audio.get_buffer_size();
        let mut wave_buffer: dasp_ring_buffer::Fixed<Vec<f32>> =
            dasp_ring_buffer::Fixed::from(vec![0.0; buffer_size]);
        let sample_rate = stream_configuration.sample_rate.0;
        let (frequency_data_receiver, frequency_data_sender) =
            single_value_channel::channel_starting_with::<Vec<f32>>(vec![0.0; buffer_size]);
        let (wave_data_receiver, wave_data_sender) =
            single_value_channel::channel_starting_with::<Vec<f32>>(vec![
                0.0;
                configuration
                    .audio
                    .get_buffer_size()
            ]);
        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let wave_buffer = &mut wave_buffer;
            for float in data {
                wave_buffer.push(*float);
            }
            let data: Vec<f32> = wave_buffer.iter().map(|f| *f).collect();
            let hamming_window = spectrum_analyzer::windows::hamming_window(&data[..]);
            let spectrum_hamming_window = spectrum_analyzer::samples_fft_to_spectrum(
                &hamming_window,
                sample_rate,
                spectrum_analyzer::FrequencyLimit::Range(10.0, 8000.0),
                Some(&scaling_function), //Scales to amplitudes
            )
            .unwrap();
            let frequency_values: Vec<_> = spectrum_hamming_window
                .data()
                .iter()
                .map(|(_, fr_val)| fr_val.val())
                .collect();
            wave_data_sender.update(data).unwrap();
            frequency_data_sender.update(frequency_values).unwrap();
        };
        let audio_stream = std::cell::RefCell::new(
            audio_device
                .build_input_stream(&stream_configuration, input_data_fn, err_fn)
                .map(|s| match s.play() {
                    Ok(_) => Some(s),
                    Err(_) => None,
                })
                .ok()
                .flatten(),
        );
        Self {
            audio_device,
            stream_configuration,
            audio_stream,
            buffer_size,
            wave_data_receiver,
            frequency_data_receiver,
            audio_stream_created: std::time::Instant::now(),
        }
    }

    pub fn get_wave_and_spectrum_data(&mut self) -> (&Vec<f32>, &Vec<f32>) {
        if self.audio_stream.borrow().is_none() {
            let last_checked = std::time::Instant::now() - self.audio_stream_created;
            if last_checked.as_secs_f32() > 5.0 {
                self.audio_stream_created = std::time::Instant::now();
                self.try_recreate_stream();
            }
        }
        (
            self.wave_data_receiver.latest(),
            self.frequency_data_receiver.latest(),
        )
    }

    fn try_recreate_stream(&mut self) {
        let mut wave_buffer: dasp_ring_buffer::Fixed<Vec<f32>> =
            dasp_ring_buffer::Fixed::from(vec![0.0; self.buffer_size]);
        let sample_rate = self.stream_configuration.sample_rate.0;
        let (frequency_data_receiver, frequency_data_sender) =
            single_value_channel::channel_starting_with::<Vec<f32>>(vec![0.0; self.buffer_size]);
        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let wave_buffer = &mut wave_buffer;
            for float in data {
                wave_buffer.push(*float);
            }
            let data: Vec<f32> = wave_buffer.iter().map(|f| *f).collect();
            let hamming_window = spectrum_analyzer::windows::hamming_window(&data[..]);
            let spectrum_hamming_window = spectrum_analyzer::samples_fft_to_spectrum(
                &hamming_window,
                sample_rate,
                spectrum_analyzer::FrequencyLimit::Range(10.0, 8000.0),
                Some(&scaling_function), //Scales to amplitudes
            )
            .unwrap();
            let frequency_values: Vec<_> = spectrum_hamming_window
                .data()
                .iter()
                .map(|(_, fr_val)| fr_val.val())
                .collect();
            frequency_data_sender.update(frequency_values).unwrap();
        };
        self.frequency_data_receiver = frequency_data_receiver;
        self.audio_stream = std::cell::RefCell::new(
            self.audio_device
                .build_input_stream(&self.stream_configuration, input_data_fn, err_fn)
                .map(|s| match s.play() {
                    Ok(_) => Some(s),
                    Err(_) => None,
                })
                .ok()
                .flatten(),
        );
    }
}

fn scaling_function(
    frequency_magnitude: f32,
    _stats: &spectrum_analyzer::scaling::SpectrumDataStats,
) -> f32 {
    debug_assert!(!frequency_magnitude.is_infinite());
    debug_assert!(!frequency_magnitude.is_nan());
    debug_assert!(frequency_magnitude >= 0.0);
    if frequency_magnitude == 0.0 {
        0.0
    } else {
        libm::log10f(frequency_magnitude) - 6.0
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {:?}", err);
}
