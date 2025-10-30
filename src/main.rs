use std::io::Write;
use std::path::Path;

mod midifile;

use midifile::MIDIFile;

fn main() {
    let midifile: MIDIFile = MIDIFile::load(Path::new(r#"C:\hoge\Fur Elise.mid"#));

    println!(
        "format:{}\ntrack_count:{}\nresolution:{}",
        midifile.header.format, midifile.header.track_count, midifile.header.resolution
    );
    //ファイルに出力
    let mut output_file = std::fs::File::create("output.txt").unwrap();
    writeln!(
        output_file,
        "format:{}\ntrack_count:{}\nresolution:{}",
        midifile.header.format, midifile.header.track_count, midifile.header.resolution
    );
    for i in 0..midifile.header.track_count {
        writeln!(output_file, "---- Track {} ----", i + 1);
        for event in &midifile.tracks[i as usize].events {
            writeln!(output_file, "{}, {}", event.delta_time, event.message);
        }
    }
}

//https://maruyama.breadfish.jp/tech/smf/
