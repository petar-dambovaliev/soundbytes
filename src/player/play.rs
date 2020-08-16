use crate::player::clamp::Clamp;
use crate::player::instrument::Instruments;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BuildStreamError, DefaultStreamConfigError, Device, PlayStreamError};
use crossbeam_channel::{unbounded, Receiver};
use log::warn;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub enum PlayErr {
    StreamErr(PlayStreamError),
    BuildStream(BuildStreamError),
    EndOfSong,
}
#[allow(dead_code)]
pub struct Player {
    device: Device,
}

#[allow(dead_code)]
impl Player {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("failed to find a default output device");

        Self { device }
    }

    pub fn spawn(
        self,
        mut instruments: Instruments,
        tempo: f32,
    ) -> Result<Receiver<PlayErr>, DefaultStreamConfigError> {
        assert!(tempo > 0.0);
        let (end_send, end_recv) = unbounded();
        let config = self.device.default_output_config()?;

        thread::spawn(move || {
            let finished = Arc::new(AtomicUsize::new(0));
            let finished_clone = Arc::clone(&finished);
            let mut finished_vec = vec![false; instruments.len()];
            let channels = config.channels() as usize;
            let volume = 0.5;
            let sample_rate = config.sample_rate().0 as f32;
            let instr_len = instruments.len();

            let err_fn = |err| warn!("an error occurred on stream: {}", err);

            let stream_res = self.device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {
                        let mut sine_amp = 0.0;

                        for (i, instrument) in instruments.iter_mut().enumerate() {
                            if !finished_vec[i] && instrument.is_finished() {
                                finished_vec[i] = true;
                                finished_clone.fetch_add(1, Ordering::SeqCst);
                                continue;
                            }

                            sine_amp += instrument.next_freq(sample_rate, tempo);
                        }

                        let next_freq = (sine_amp * volume * 0.1).my_clamp(-1.0, 1.0);

                        for sample in frame.iter_mut() {
                            *sample = next_freq;
                        }
                    }
                },
                err_fn,
            );

            let stream = match stream_res {
                Ok(str) => str,
                Err(e) => {
                    if let Err(ee) = end_send.send(PlayErr::BuildStream(e)) {
                        warn!("could not send stream error {}", ee);
                    }
                    return;
                }
            };

            loop {
                if let Err(e) = stream.play() {
                    if let Err(ee) = end_send.send(PlayErr::StreamErr(e)) {
                        warn!("could not send stream error {}", ee);
                    }
                }
                if finished.load(Ordering::SeqCst) == instr_len {
                    if let Err(ee) = end_send.send(PlayErr::EndOfSong) {
                        warn!("could not send end of stream {}", ee);
                    }
                    return;
                }
            }
        });
        Ok(end_recv)
    }
}
