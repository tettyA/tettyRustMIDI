use core::panic;
use std::fs;
use std::io::{BufReader, Read, Seek};

pub struct Header {
    pub format: u16, //0 or 1 (Not 2)
    pub track_count: u16,
    pub resolution: u16, //分解能
}

impl Header {
    pub fn load(f: &mut BufReader<fs::File>, sum_data_length: &mut u64) -> Self {
        let mut buf4: [u8; 4] = [0, 0, 0, 0]; //4 bytes buf
        let mut buf2: [u8; 2] = [0, 0]; //2 bytes buf
        //Header Chank (4byte)
        f.read_exact(&mut buf4).unwrap();

        if !(buf4[0] == b'M' && buf4[1] == b'T' && buf4[2] == b'h' && buf4[3] == b'd') {
            panic!("Loaded Non-MIDI File!");
        }

        //Header Bytes Length (4byte);
        f.read_exact(&mut buf4).unwrap();
        let header_bytes_length = ((buf4[0] as u64) << 24)
            + ((buf4[1] as u64) << 16)
            + ((buf4[2] as u64) << 8)
            + buf4[3] as u64;

        //Format (2byte)
        f.read_exact(&mut buf2).unwrap();
        let format = (buf2[0] as u16) * 0x1_00 + buf2[1] as u16;

        //Track Count (2byte)
        f.read_exact(&mut buf2).unwrap();
        let track_count = (buf2[0] as u16) * 0x1_00 + buf2[1] as u16;

        //Resolution (2byte)
        f.read_exact(&mut buf2).unwrap();
        if buf2[0] & 0b1000_0000 == 0b1000_0000 {
            panic!("This MIDI file is not supported!");
        }
        let resolution = (buf2[0] as u16) * 0x1_00 + buf2[1] as u16;

        //+ Mthd(4) + length(4) + [format(2) + trackcount(2) + resolution(2);=heaer_bytes_length]
        f.seek(std::io::SeekFrom::Start(header_bytes_length + 4 + 4));
        *sum_data_length += header_bytes_length + 4 + 4;

        Header {
            format: format,
            track_count: track_count,
            resolution: resolution,
        }
    }
}
