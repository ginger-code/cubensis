use crate::AudioStreamSource::{InputStream, WasapiLoopback};
use cpal::traits::{DeviceTrait, HostTrait};
use itertools::Itertools;

pub mod wave_stream;

#[derive(Clone, Debug)]
pub enum AudioStreamSource {
    InputStream(String),
    WasapiLoopback(String),
}

impl AudioStreamSource {
    pub fn all_streams() -> Vec<Self> {
        let host = cpal::default_host();
        let mut streams: Vec<_> = host
            .input_devices()
            .unwrap()
            .map(|d| InputStream(d.name().unwrap()))
            .collect();
        #[cfg(windows)]
        if let Some(dev) = host.default_output_device() {
            if let Ok(name) = dev.name() {
                streams.push(WasapiLoopback(name.to_owned()));
            }
        }
        streams
    }

    pub fn default_stream() -> Self {
        let host = cpal::default_host();
        #[cfg(windows)]
        {
            WasapiLoopback(
                host.default_output_device()
                    .unwrap()
                    .name()
                    .unwrap()
                    .to_owned(),
            )
        }
        #[cfg(not(windows))]
        {
            InputStream(
                host.default_input_device()
                    .unwrap()
                    .name()
                    .unwrap()
                    .to_owned(),
            )
        }
    }
    pub fn name(&self) -> String {
        match self {
            AudioStreamSource::InputStream(name) => name.to_owned(),
            AudioStreamSource::WasapiLoopback(name) => name.to_owned(),
        }
    }
    pub fn is_wasapi(&self) -> bool {
        match self {
            AudioStreamSource::InputStream(_) => false,
            AudioStreamSource::WasapiLoopback(_) => true,
        }
    }
    pub(crate) fn get_audio_device_and_stream_configuration(
        &self,
    ) -> (cpal::Device, cpal::StreamConfig) {
        let host = cpal::default_host();
        let name = self.name();
        if self.is_wasapi() {
            let audio_device = host.default_output_device().unwrap();
            let config = audio_device
                .supported_output_configs()
                .unwrap()
                .into_iter()
                .map(|c| c.with_max_sample_rate())
                .find_or_first(|c| c.sample_format() == cpal::SampleFormat::F32)
                .unwrap()
                .config();
            (audio_device, config)
        } else {
            let audio_device = host
                .input_devices()
                .unwrap()
                .find_or_first(|d| d.name().unwrap().eq_ignore_ascii_case(name.as_str()))
                .unwrap();
            let config = audio_device
                .supported_output_configs()
                .unwrap()
                .into_iter()
                .map(|c| c.with_max_sample_rate())
                .find_or_first(|c| c.sample_format() == cpal::SampleFormat::F32)
                .unwrap()
                .config();
            (audio_device, config)
        }
    }
}
