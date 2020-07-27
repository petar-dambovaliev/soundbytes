use crossbeam_channel::{unbounded, Receiver, Sender};
use nannou_audio as audio;
use nannou_audio::stream::BuildError;
use nannou_audio::{Buffer, PlayStreamError, Stream};
use std::f64::consts::PI;
use std::thread;

const A: usize = 440;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Note {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Octave {
    One = -36,
    Two = -24,
    Three = -12,
    Four = 0,
    Five = 12,
    Six = 24,
    Seven = 36,
    Eight = 48,
}

impl Note {
    pub fn frequency(self, octave: Octave) -> f64 {
        let float = (self as isize + octave as isize) as f64 / 12.0;
        A as f64 * float.exp2()
    }
}

pub struct Player {
    stream: Stream<Audio>,
}

impl Player {
    pub fn new(state: Audio) -> Result<Self, BuildError> {
        let stream = audio::Host::new()
            .new_output_stream(state)
            .render(render)
            .build()?;
        Ok(Self { stream })
    }

    pub fn spawn(self) -> (Sender<Audio>, Receiver<PlayStreamError>) {
        let (audio_sender, audio_receiver): (Sender<Audio>, Receiver<Audio>) = unbounded();
        let (err_sender, err_receiver) = unbounded();

        thread::spawn(move || loop {
            if let Err(e) = self.stream.play() {
                if let Err(e) = err_sender.send(e) {
                    println!("cannot write PlayStreamError {}", e);
                }
            }

            if let Ok(a) = audio_receiver.try_recv() {
                if let Err(e) = self.stream.send(move |mut audio| {
                    audio.hz = a.hz;
                    audio.phase = a.phase;
                }) {
                    println!("cannot read Audio {}", e);
                }
            }
        });
        (audio_sender, err_receiver)
    }
}

pub struct Audio {
    pub phase: f64,
    pub hz: f64,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            phase: 0.0,
            hz: A as f64,
        }
    }
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play a simple sine wave at the audio's current frequency in `hz`.
fn render(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        let sine_amp = (2.0 * PI * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = sine_amp * volume;
        }
    }
}
