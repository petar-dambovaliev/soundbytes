mod play;

extern crate crossbeam_channel;
extern crate nannou_audio;

use play::{Audio, Note, Octave, Player};
use std::time::Duration;

fn main() {
    let player = Player::new(Default::default()).unwrap();
    let (note_sender, err_receiver) = player.spawn();
    let notes = vec![
        (Note::C, Octave::Four),
        (Note::C, Octave::Five),
        (Note::G, Octave::Four),
        (Note::G, Octave::Five),
    ];

    for (note, octave) in notes {
        if let Err(e) = note_sender.send(Audio {
            phase: 0.0,
            hz: note.clone().frequency(octave),
        }) {
            println!("could not send note {:?} error {:?}", note, e);
        }

        if let Ok(e) = err_receiver.try_recv() {
            println!("{:?}", e);
        }
        std::thread::sleep(Duration::from_millis(200));
    }
}
