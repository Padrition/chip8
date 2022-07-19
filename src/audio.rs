use rodio::source::{SineWave, Source};
use rodio::{OutputStreamHandle};
use std::time::Duration;

use crate::processor::Cpu;
use super::TIMER_RATE;

pub struct Audio{
    frequency: f32,
    duration_ms: u64,
    stream_handle: OutputStreamHandle,
    playing: bool,
}

impl Audio {
    pub fn new(frequency: f32, stream_handle: OutputStreamHandle) -> Audio{
        Audio{
            frequency: frequency,
            duration_ms: 0,
            stream_handle: stream_handle,
            playing: false,
        }
    }

    fn calc_audio_duration(&self, cpu: &Cpu) -> u64{
        let sound_timer = cpu.check_sound_timer() as u64;
        let micros_left = sound_timer * TIMER_RATE;
        let millis_left = micros_left / 1000;
        millis_left
    }

    pub fn play(&mut self, cpu: &Cpu){
        if !self.playing && cpu.check_sound_timer() > 0{
            self.duration_ms = self.calc_audio_duration(cpu);
            self.playing = true;
            let source = SineWave::new(self.frequency).take_duration(Duration::from_millis(self.duration_ms)).amplify(0.20);
            self.stream_handle.play_raw(source.convert_samples()).unwrap();
        }
        if self.playing && cpu.check_sound_timer() == 0{
            self.playing = false;
        }
    }
}