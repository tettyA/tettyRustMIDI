mod midievent;

use crate::midifile::track::midievent::{ControlNumber_ChannelMode, MIDIEvent, Message, SysCommonMessage};

use std::fs;
use std::io::{BufReader, Read, Seek};
pub struct Track {
    pub events: Vec<MIDIEvent>,
}

impl Track {
    pub fn load(f: &mut BufReader<fs::File>,sum_data_length:&mut u64) -> Self {
        let mut events = Vec::<MIDIEvent>::new();

        let mut buf4: [u8; 4] = [0, 0, 0, 0]; //4 bytes buf HH HH HH HH
        let mut buf2: [u8; 2] = [0, 0];
        let mut buf1: [u8; 1] = [0];
        //Header Chank (4byte)
        f.read_exact(&mut buf4).unwrap();

        if !(buf4[0] == b'M' && buf4[1] == b'T' && buf4[2] == b'r' && buf4[3] == b'k') {
            panic!("Loaded Non-MIDI File!");
        }
        //Data Length (4byte)
        f.read_exact(&mut buf4).unwrap();
        let data_length = (buf4[0] as u64) * 0x1_00_00_00
            + (buf4[1] as u64) * 0x1_00_00
            + (buf4[2] as u64) * 0x1_00
            + buf4[3] as u64;

        let mut events = Vec::<MIDIEvent>::new();
        loop{
            //[delta time][event][delta time][event]...
            
            //----Read Delta time----
            let deltatime = Self::ReadVarLen(f);
            //++++Read Delta time++++

            //----Read Event----
          match  f.read_exact(&mut buf1){
            Err(_e)=>{
                break;
            },
            Ok(_)=>{
          }
        }
            if (buf1[0] & 0x80 == 0) {
                continue;
            }
            let statusbyte = buf1[0] & 0xF0;
            let channelbyte = buf1[0] & 0x0F;
            let message: Message = match statusbyte {
                //---2 bytes messages---
                Message::STATUS_NOTE_OFF2
                | Message::STATUS_NOTE_ON2
                | Message::STATUS_POLYPHONICKEYPRESSURE2
                | Message::STATUS_CONTROL_CHANGE2
                | Message::STATUS_PITCH_BEND_CHANGE2 => {
                    f.read_exact(&mut buf2);
                    let bufupper = buf2[0];
                    let buflower = buf2[1];
                    match statusbyte {
                        Message::STATUS_NOTE_OFF2 => Message::NoteOff {
                            channel: channelbyte,
                            note_num: bufupper,
                            velocity: buflower,
                        },
                        Message::STATUS_NOTE_ON2 => {
                            if buflower == 0 {
                                Message::NoteOff {
                                    channel: channelbyte,
                                    note_num: buflower,
                                    velocity: bufupper,
                                }
                            } else {
                                Message::NoteOn {
                                    channel: channelbyte,
                                    note_num: bufupper,
                                    velocity: buflower,
                                }
                            }
                        }
                        Message::STATUS_POLYPHONICKEYPRESSURE2 => Message::PolyphonicKeyPressure {
                            channel: channelbyte,
                            note_num: bufupper,
                            pressure: buflower,
                        },
                        Message::STATUS_CONTROL_CHANGE2 => Message::ControlChange_ChannelMode {
                            channel: channelbyte,
                            control:match bufupper{
                                0x00=>ControlNumber_ChannelMode::BankSelect_LSB,
                                0x01=>ControlNumber_ChannelMode::ModulationWheel_Lever,
                                0x02=>ControlNumber_ChannelMode::BreathController,

                                0x04=>ControlNumber_ChannelMode::FootController,
                                0x05=>ControlNumber_ChannelMode::PoltamentoTime,
                                0x06=>ControlNumber_ChannelMode::DataEntry_MSB,
                                0x07=>ControlNumber_ChannelMode::ChannelVolume,
                                0x08=>ControlNumber_ChannelMode::Balance,

                                0x0A=>ControlNumber_ChannelMode::Pan,
                                0x0B=>ControlNumber_ChannelMode::ExpressionControloller,
                                0x0C=>ControlNumber_ChannelMode::EffectControl1,
                                0x0D=>ControlNumber_ChannelMode::EffectControl2,

                                0x10=>ControlNumber_ChannelMode::GeneralControler1,
                                0x11=>ControlNumber_ChannelMode::GeneralControler2,
                                0x12=>ControlNumber_ChannelMode::GeneralControler3,
                                0x13=>ControlNumber_ChannelMode::GeneralControler4,

                                0x20=>ControlNumber_ChannelMode::BankSelect_LSB,
                                0x21=>ControlNumber_ChannelMode::Control01_LSB,
                                0x22=>ControlNumber_ChannelMode::Control02_LSB,
                                0x23=>ControlNumber_ChannelMode::Control03_LSB,
                                0x24=>ControlNumber_ChannelMode::Control04_LSB,
                                0x25=>ControlNumber_ChannelMode::Control05_LSB,
                                0x26=>ControlNumber_ChannelMode::Control06_LSB,
                                0x27=>ControlNumber_ChannelMode::Control07_LSB,
                                0x28=>ControlNumber_ChannelMode::Control08_LSB,
                                0x29=>ControlNumber_ChannelMode::Control09_LSB,
                                0x2A=>ControlNumber_ChannelMode::Control0A_LSB,
                                0x2B=>ControlNumber_ChannelMode::Control0B_LSB,
                                0x2C=>ControlNumber_ChannelMode::Control0C_LSB,
                                0x2D=>ControlNumber_ChannelMode::Control0D_LSB,
                                0x2E=>ControlNumber_ChannelMode::Control0E_LSB,
                                0x2F=>ControlNumber_ChannelMode::Control0F_LSB,
                                0x30=>ControlNumber_ChannelMode::Control10_LSB,
                                0x31=>ControlNumber_ChannelMode::Control11_LSB,
                                0x32=>ControlNumber_ChannelMode::Control12_LSB,
                                0x33=>ControlNumber_ChannelMode::Control13_LSB,
                                0x34=>ControlNumber_ChannelMode::Control14_LSB,
                                0x35=>ControlNumber_ChannelMode::Control15_LSB,
                                0x36=>ControlNumber_ChannelMode::Control16_LSB,
                                0x37=>ControlNumber_ChannelMode::Control17_LSB,
                                0x38=>ControlNumber_ChannelMode::Control18_LSB,
                                0x39=>ControlNumber_ChannelMode::Control19_LSB,
                                0x3A=>ControlNumber_ChannelMode::Control1A_LSB,
                                0x3B=>ControlNumber_ChannelMode::Control1B_LSB,
                                0x3C=>ControlNumber_ChannelMode::Control1C_LSB,
                                0x3D=>ControlNumber_ChannelMode::Control1D_LSB,
                                0x3E=>ControlNumber_ChannelMode::Control1E_LSB,
                                0x3F=>ControlNumber_ChannelMode::Control1F_LSB,
                                0x40=>ControlNumber_ChannelMode::Hold,
                                0x41=>ControlNumber_ChannelMode::PoltamentoOnOff,
                                0x42=>ControlNumber_ChannelMode::Sustenuto,
                                0x43=>ControlNumber_ChannelMode::Softpedal,
                                0x44=>ControlNumber_ChannelMode::RegardFootSwitch,
                                0x45=>ControlNumber_ChannelMode::Hold2,
                                0x46=>ControlNumber_ChannelMode::SoundController1,
                                0x47=>ControlNumber_ChannelMode::SoundController2,
                                0x48=>ControlNumber_ChannelMode::SoundController3,
                                0x49=>ControlNumber_ChannelMode::SoundController4,
                                0x4A=>ControlNumber_ChannelMode::SoundController5,
                                0x4B=>ControlNumber_ChannelMode::SoundController6,
                                0x4C=>ControlNumber_ChannelMode::SoundController7,
                                0x4D=>ControlNumber_ChannelMode::SoundController8,
                                0x4E=>ControlNumber_ChannelMode::SoundController9,
                                0x4F=>ControlNumber_ChannelMode::SoundControllerA,
                                0x50=>ControlNumber_ChannelMode::GeneralController5,
                                0x51=>ControlNumber_ChannelMode::GeneralController6,
                                0x52=>ControlNumber_ChannelMode::GeneralController7,
                                0x53=>ControlNumber_ChannelMode::GeneralController8,
                                0x54=>ControlNumber_ChannelMode::PoltamentoControl,

                                0x5B=>ControlNumber_ChannelMode::Effect1Depth,
                                0x5C=>ControlNumber_ChannelMode::Effect2Depth,
                                0x5D=>ControlNumber_ChannelMode::Effect3Depth,
                                0x5E=>ControlNumber_ChannelMode::Effect4Depth,
                                0x5F=>ControlNumber_ChannelMode::Effect5Depth,
                                0x60=>ControlNumber_ChannelMode::DataIncrement,
                                0x61=>ControlNumber_ChannelMode::DataDecrement,
                                0x62=>ControlNumber_ChannelMode::NonRegisteredPalameterNumber_LSB,
                                0x63=>ControlNumber_ChannelMode::NonRegisteredPalameterNumber_MSB,
                                0x64=>ControlNumber_ChannelMode::RegisteredPalameterNumber_LSB,
                                0x65=>ControlNumber_ChannelMode::RegisteredPalameterNumber_MSB,

                                0x78=>ControlNumber_ChannelMode::ChannelModeMessage1_AllSoundOf,
                                0x79=>ControlNumber_ChannelMode::ChannelModeMessage2_ResetAllControler,
                                0x7A=>ControlNumber_ChannelMode::ChannelModeMessage3_LocalControl,
                                0x7B=>ControlNumber_ChannelMode::ChannelModeMessage4_AllNoteOff,
                                0x7C=>ControlNumber_ChannelMode::ChannelModeMessage5_OmniOff,
                                0x7D=>ControlNumber_ChannelMode::ChannelModeMessage6_OmniOn,
                                0x7E=>ControlNumber_ChannelMode::ChannelModeMessage7_MonoModeOn,
                                0x7F=>ControlNumber_ChannelMode::ChannelModeMessage8_PolyModeOn,

                                _=>ControlNumber_ChannelMode::Undefined
                                
                            },
                            controlnum: buflower,
                        },
                        Message::STATUS_PITCH_BEND_CHANGE2 => Message::PitchBentoChange {
                            channel: channelbyte,
                            lsb: bufupper,
                            msb: buflower,
                        },

                        _ => unreachable!(),
                    }
                }
                //---1 byte messages---
                Message::STATUS_PROGRAM_CHANGE1 | Message::STATUS_CHANNEL_PRESSURE1 => {
                    f.read_exact(&mut buf1);
                    let data = buf1[0];
                    match statusbyte {
                        Message::STATUS_PROGRAM_CHANGE1 => Message::ProgramChange {
                            channel: channelbyte,
                            program_num: data,
                        },
                        Message::STATUS_CHANNEL_PRESSURE1 => Message::ChannelPressure {
                            channel: channelbyte,
                            pressure: data,
                        },
                        _ => unreachable!(),
                    }
                }

                Message::STATUS_SYSTEM_MESSAGE|0xF7=>{
                    //SysEx
                if    channelbyte==0x00||channelbyte==0x07{
                     let len = Self::ReadVarLen(f);
                    let mut data: Vec<u8> = Vec::with_capacity(len as usize);
                    for i in 0..len {
                        f.read_exact(&mut buf1);
                        data[i as usize] = buf1[0];
                    }
                    if statusbyte == 0x00{
                        f.read_exact(&mut buf1);
                        if buf1[0] != 0x07 {
                            panic!("MIDI file is broken!");
                        }
                    }
                    Message::SysMesage(midievent::SysMessage::SysEx { len, message: data })

                }
                /*//SysRealTime
                else if channelbyte & 0b0000_1000!=0{
                    match statusbyte|channelbyte {
                        SysRealTimeMessage::STATUS_Timing_Clock=>{
                            Message::SysMesage(midievent::SysMessage::SysRealTime(SysRealTimeMessage::Timing_Clock))
                        },
                        SysRealTimeMessage::STATUS_F9Undefined=>{
                            Message::SysMesage(midievent::SysMessage::SysRealTime(SysRealTimeMessage::F9Undefined))
                        },
                        SysRealTimeMessage::STATUS_Start=>{
                            Message::SysMesage(midievent::SysMessage::SysRealTime(SysRealTimeMessage::Start))
                        },
                        SysRealTimeMessage::STATUS_Continue=>{
                            Message::SysMesage(midievent::SysMessage::SysRealTime(SysRealTimeMessage::Continue))
                        },
                        SysRealTimeMessage::STATUS_Stop=>{
                            Message::SysMesage(midievent::SysMessage::SysRealTime(SysRealTimeMessage::Stop))
                        },
                         SysRealTimeMessage::STATUS_FDUndefined=>{
                            Message::SysMesage(midievent::SysMessage::SysRealTime(SysRealTimeMessage::FDUndefined))
                        },
                        SysRealTimeMessage::STATUS_Active_Sensing=>{
                            Message::SysMesage(midievent::SysMessage::SysRealTime(SysRealTimeMessage::Active_Sensing))
                        },
                        SysRealTimeMessage::STATUS_System_Reset=>{
                            Message::SysMesage(midievent::SysMessage::SysRealTime(SysRealTimeMessage::System_Reset))
                        },
                       
                        _=>unreachable!()

                    }
                }*/
                //MetaEvent
                else if statusbyte==0xF0&& channelbyte==0x0F{
                     //Meta Event
                f.read_exact(&mut buf1);
                let metaid= buf1[0];
                
                    let len = Self::ReadVarLen(f);
                    match metaid {
                        //SequenceNum
                        0x00 => {
                            f.read_exact(&mut buf2);
                            Message::Meta(midievent::MetaEvent::SequenceNum(
                                ((buf2[0] as u16) << 8) + buf2[1] as u16,
                            ))
                        }
                        0x20=>{
                             f.read_exact(&mut buf1);
                             Message::Meta(midievent::MetaEvent::MIDIChannelPlyfix(buf1[0]))
                        }
                        0x2F=>{

                            Message::Meta(midievent::MetaEvent::EndOfTrack);
                            break;
                        }
                        0x51=>{
                            let mut buf3: [u8; 3] = [0,0,0];
                            f.read_exact(&mut buf3);
                            Message::Meta(
                                midievent::MetaEvent::SetTempo((
                                    (buf3[0] as u32)<<16)+ 
                                    ((buf3[1] as u32)<<8)+
                                     buf3[2] as u32))
                        }
                        0x54=>{
                            let mut buf5: [u8; 5] = [0,0,0,0,0];
                            f.read_exact(&mut buf5);

                            Message::Meta(midievent::MetaEvent::SMPTEOffset
                                 { hr: buf5[0], mn: buf5[1], se: buf5[2], fr: buf5[3], ff: buf5[4] })
                        }
                        0x58=>{
                            f.read_exact(&mut buf4);
                            Message::Meta(midievent::MetaEvent::TimeSignature { nn: buf4[0], dd: buf4[1], cc: buf4[2], bb: buf4[3] })
                        }
                        0x59=>{
                            f.read_exact(&mut buf2);
                            Message::Meta(midievent::MetaEvent::KeySignature { sf: buf2[0], mi: buf2[1] })
                        }
                        0x01 //Text
                        |0x02//copyright
                        |0x03//シーケンストラックネーム
                        |0x04//Instrument Name
                        |0x05//kashi
                        |0x06//marker
                        |0x07//queuepoint
                        |0x7F//シーケンサ'sspecific meta event 
                        |_//Others
                         => {
                            let mut data: Vec<u8> =vec![0;len as usize];
                            for i in 0..len {
                                f.read_exact(&mut buf1);
                                data[i as usize] = buf1[0];
                            }

                            match channelbyte{
                                0x01=>Message::Meta(midievent::MetaEvent::TextEvent { len, text: data }),
                                0x02=>Message::Meta(midievent::MetaEvent::Copyright { len, text: data }),
                                0x03=>Message::Meta(midievent::MetaEvent::Sequence_TrackName { len, text: data }),
                                0x04=>Message::Meta(midievent::MetaEvent::InstrumentalName { len, text: data }),
                                0x05=>Message::Meta(midievent::MetaEvent::Lyrics { len, text: data }),
                                0x06=>Message::Meta(midievent::MetaEvent::Marker { len, text: data }),
                                0x07=>Message::Meta(midievent::MetaEvent::QueuePoint { len, text: data }),
                                0x7F=>Message::Meta(midievent::MetaEvent::Sequence_TrackName { len, text: data }),
                                _=>Message::Meta(midievent::MetaEvent::Unsupported { len, data }),
                            }
                         },
                         
                       
                    }
                
                }
                //SysCommon
                else if channelbyte & 0b0000_0111!=0{
                    match statusbyte|channelbyte {
                        SysCommonMessage::STATUS_MIDI_Time_Code_Qtr_Frame=>{
                            f.read_exact(&mut buf1);
                            Message::SysMesage(midievent::SysMessage::SysCommon(
                                SysCommonMessage::MIDI_Time_Code_Qtr_Frame { message_type: buf1[0]>>4, values: buf1[0]&0b0000_1111 }
                            ))
                        }
                        SysCommonMessage::STATUS_Song_Position_Pointer=>{
                            f.read_exact(&mut buf2);
                            Message::SysMesage(midievent::SysMessage::SysCommon(
                                SysCommonMessage::Song_Position_Pointer { lsb: buf2[0], msb: buf2[1] }
                            ))
                        }
                        SysCommonMessage::STATUS_Song_Select=>{
                            f.read_exact(&mut buf1);
                            Message::SysMesage(midievent::SysMessage::SysCommon(
                                SysCommonMessage::Song_Select { song_num: buf1[0] }
                            ))
                        }
                        SysCommonMessage::STATUS_Undefined_F4=>{
                            Message::SysMesage(midievent::SysMessage::SysCommon(
                                SysCommonMessage::Undefined_F4
                            ))
                        }
                        SysCommonMessage::STATUS_Undefined_F5=>{
                            Message::SysMesage(midievent::SysMessage::SysCommon(
                                SysCommonMessage::Undefined_F5
                            ))
                        }
                        SysCommonMessage::STATUS_Tune_Request=>{
                            Message::SysMesage(midievent::SysMessage::SysCommon(
                                SysCommonMessage::Tune_Request
                            ))
                        }
                        SysCommonMessage::STATUS_End_of_SysEx=>{
                            Message::SysMesage(midievent::SysMessage::SysCommon(
                                SysCommonMessage::End_of_SysEx
                            ))
                        }
                        _=>unreachable!()
                    }
                }else {
                    unreachable!()
                }
            },
               
                _ => {
                    panic!("Unsupported MIDI Event found!");
                    Message::Unsupported},
            };
            //++++Read Event++++

            events.push(MIDIEvent {
                delta_time: deltatime,
                message,
            });
        }

       f.seek(std::io::SeekFrom::Start(
        *sum_data_length+
            (data_length + 4 + 4),
        ));
        *sum_data_length+=data_length + 4 + 4;
        Track { events }
    }

    //ref: p.144 of https://amei.or.jp/midistandardcommittee/MIDI1.0.pdf
    fn ReadVarLen(f: &mut BufReader<fs::File>) -> u32 {
        let mut value: u32 = 0;

        let mut buf: [u8; 1] = [0];
        f.read_exact(&mut buf);
        value= buf[0] as u32;
        if buf[0] & 0x80 != 0 {
            value &= 0x7F;

            f.read_exact(&mut buf);
            value = (value << 7) + (buf[0] & 0x7F) as u32;
            while value & 0x80 != 0 {
                f.read_exact(&mut buf);
                value = (value << 7) + (buf[0] & 0x7F) as u32;
            }
        }
        value
    }
}
