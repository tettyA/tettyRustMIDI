//ref: https://maruyama.breadfish.jp/tech/smf/
//ref: https://amei.or.jp/midistandardcommittee/MIDI1.0.pdf
use std::fs;
use std::io::BufReader;
use std::path::Path;

use crate::midifile::header::Header;
use crate::midifile::track::Track;

pub mod header;
pub mod track;

pub struct MIDIFile {
    pub header: Header,
    pub tracks: Vec<Track>,
}
impl MIDIFile {
    pub fn load(path: &Path) -> Self {
        let mut f = BufReader::new(fs::File::open(path).unwrap());
        let mut sum_data_length: u64 = 0;
        let h: Header = Header::load(&mut f, &mut sum_data_length);

        let mut ts = Vec::<Track>::with_capacity(h.track_count as usize);
        for _ in 0..h.track_count {
            ts.push(Track::load(&mut f, &mut sum_data_length));
        }

        MIDIFile {
            header: h,
            tracks: ts,
        }
    }
}
