use std::io::Write;
use std::path::Path;

mod midifile;

use midifile::MIDIFile;
use midifile::track::midievent;
fn main() {
    let midifile: MIDIFile = MIDIFile::load(Path::new(
        r#"C:\hoge\fuga.mid"#,
    ));

    println!(
        "format:{}\ntrack_count:{}\nresolution:{}",
        midifile.header.format, midifile.header.track_count, midifile.header.resolution
    );
    for i in 0..midifile.header.track_count {
        println!("--- Track {} ---", i);
        for event in &midifile.tracks[i as usize].events {
            // println!(output_file, "{}, {}", event.delta_time, event.message);

            match &event.message {
                midievent::Message::Meta(meta_event) => match meta_event {
                    midievent::MetaEvent::SetTempo(tempo) => {
                        println!("Tempo Change: {}", tempo);
                    }
                    midievent::MetaEvent::Lyrics { len, text } => {
                        print!("Lyrics: len {}, text: ", len);
                        for c in text {
                            print!("{}", *c as char);
                        }
                        println!();
                    }
                    midievent::MetaEvent::EndOfTrack => {
                        println!("End of Track");
                    }
                    midievent::MetaEvent::KeySignature { sf, mi } => {
                        println!("Key Signature: sf {}, mi {}", sf, mi);
                    }
                    _ => {}
                },
                midievent::Message::PitchBentoChange { channel, lsb, msb } => {
                    println!(
                        "PitchBentoChange: channel {}, lsb {}, msb {}",
                        channel, lsb, msb
                    );
                }
                _ => {}
            }
        }
    }
}

//https://maruyama.breadfish.jp/tech/smf/
