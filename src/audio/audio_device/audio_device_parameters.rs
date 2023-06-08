use tinyaudio::prelude::*;



pub struct AudioDeviceParameters {
    pub sample_rate: usize,
    pub block_size: usize,
    pub blocks_count: usize,
}

impl AudioDeviceParameters {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            block_size: 441,
            blocks_count: 4
        }
    }
    pub fn get_output_device_parameters(&self) -> OutputDeviceParameters {
        OutputDeviceParameters{
            sample_rate: self.sample_rate,
            channels_count: 2,
            channel_sample_count: self.block_size * self.blocks_count
        }
    }
}
impl Default for AudioDeviceParameters {
    fn default() -> Self {
        Self::new()
    }
}
