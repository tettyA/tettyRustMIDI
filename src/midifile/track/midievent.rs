//ref:https://amei.or.jp/midistandardcommittee/MIDI1.0.pdf

pub struct MIDIEvent {
    pub delta_time: u32,
    pub message: Message,
}

//ref:p.104
//最上位ビット(MSB)が必ず1
pub enum Message {
    //8nH kkH vvH
    NoteOff {
        channel: u8,  //n
        note_num: u8, //kk
        velocity: u8, //vv
    },
    //9nH kkH vvH
    NoteOn {
        channel: u8,
        note_num: u8,
        velocity: u8,
    },
    //AnH kkH vvH
    PolyphonicKeyPressure {
        //キーを押した状態でその圧力が変化した
        channel: u8,
        note_num: u8,
        pressure: u8,
    },
    //BnH ccH vvH cc:0~119
    ControlChange_ChannelMode {
        channel: u8,
        control: ControlNumber_ChannelMode,
        controlnum: u8,
    },
    //CnH ppH
    ProgramChange {
        channel: u8,
        program_num: u8,
    },
    //DnH vvH
    ChannelPressure {
        channel: u8,
        pressure: u8,
    },
    //EnH llH hhH
    PitchBentoChange {
        channel: u8,
        lsb: u8,
        msb: u8,
    },
    //F0H ... F7H
    SysMesage(SysMessage),

    //FFH ?????
    Meta(MetaEvent),
    Unsupported,
}
impl Message {
    //2 byte data message
    pub const STATUS_NOTE_OFF2: u8 = 0x80;
    pub const STATUS_NOTE_ON2: u8 = 0x90;
    pub const STATUS_POLYPHONICKEYPRESSURE2: u8 = 0xA0;
    pub const STATUS_CONTROL_CHANGE2: u8 = 0xB0;
    pub const STATUS_PITCH_BEND_CHANGE2: u8 = 0xE0;
    //1 byte data message
    pub const STATUS_PROGRAM_CHANGE1: u8 = 0xC0;
    pub const STATUS_CHANNEL_PRESSURE1: u8 = 0xD0;

    //system message
    pub const STATUS_SYSTEM_MESSAGE: u8 = 0xF0;

    //   pub const STATUS_SYSMSG: u8 = 0xF0;
    /*
    pub const STATUS_SYSEX_END: u8 = 0xF7;
    */
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::NoteOff {
                channel,
                note_num,
                velocity,
            } => write!(
                f,
                "NoteOff: channel {}, note_num {}, velocity {}",
                channel, note_num, velocity
            ),
            Message::NoteOn {
                channel,
                note_num,
                velocity,
            } => write!(
                f,
                "NoteOn: channel {}, note_num {}, velocity {}",
                channel, note_num, velocity
            ),
            _ => write!(f, "Other Message"),
        }
    }
}

//ref p106
pub enum ControlNumber_ChannelMode {
    BankSelect_MSB = 0x00,
    ModulationWheel_Lever = 0x01,
    BreathController = 0x02,
    //0x03 is Undefined
    FootController = 0x04,
    PoltamentoTime = 0x05,
    DataEntry_MSB = 0x06,
    ChannelVolume = 0x07,
    Balance = 0x08,
    //0x09 is Undefined
    Pan = 0x0A,
    ExpressionControloller = 0x0B,
    EffectControl1 = 0x0C,
    EffectControl2 = 0x0d,
    //0x0E ~ 0x0F are Undefined
    GeneralControler1 = 0x10,
    GeneralControler2 = 0x11,
    GeneralControler3 = 0x12,
    GeneralControler4 = 0x13,
    //0x14 ~ 0x1F are Undefined
    BankSelect_LSB = 0x20,
    Control01_LSB = 0x21,
    Control02_LSB = 0x22,
    Control03_LSB = 0x23,
    Control04_LSB = 0x24,
    Control05_LSB = 0x25,
    Control06_LSB = 0x26,
    Control07_LSB = 0x27,
    Control08_LSB = 0x28,
    Control09_LSB = 0x29,
    Control0A_LSB = 0x2A,
    Control0B_LSB = 0x2B,
    Control0C_LSB = 0x2C,
    Control0D_LSB = 0x2D,
    Control0E_LSB = 0x2E,
    Control0F_LSB = 0x2F,
    Control10_LSB = 0x30,
    Control11_LSB = 0x31,
    Control12_LSB = 0x32,
    Control13_LSB = 0x33,
    Control14_LSB = 0x34,
    Control15_LSB = 0x35,
    Control16_LSB = 0x36,
    Control17_LSB = 0x37,
    Control18_LSB = 0x38,
    Control19_LSB = 0x39,
    Control1A_LSB = 0x3A,
    Control1B_LSB = 0x3B,
    Control1C_LSB = 0x3C,
    Control1D_LSB = 0x3D,
    Control1E_LSB = 0x3E,
    Control1F_LSB = 0x3F,
    Hold = 0x40,
    PoltamentoOnOff = 0x41,
    Sustenuto = 0x42,
    Softpedal = 0x43,
    RegardFootSwitch = 0x44,
    Hold2 = 0x45,
    SoundController1 = 0x46,
    SoundController2 = 0x47,
    SoundController3 = 0x48,
    SoundController4 = 0x49,
    SoundController5 = 0x4A,
    SoundController6 = 0x4B,
    SoundController7 = 0x4C,
    SoundController8 = 0x4D,
    SoundController9 = 0x4E,
    SoundControllerA = 0x4F,
    GeneralController5 = 0x50,
    GeneralController6 = 0x51,
    GeneralController7 = 0x52,
    GeneralController8 = 0x53,
    PoltamentoControl = 0x54,
    //0x55 ~ 0x5A is Undefined
    Effect1Depth = 0x5B,
    Effect2Depth = 0x5C,
    Effect3Depth = 0x5D,
    Effect4Depth = 0x5E,
    Effect5Depth = 0x5F,
    DataIncrement = 0x60,
    DataDecrement = 0x61,
    NonRegisteredPalameterNumber_LSB = 0x62,
    NonRegisteredPalameterNumber_MSB = 0x63,
    RegisteredPalameterNumber_LSB = 0x64,
    RegisteredPalameterNumber_MSB = 0x65,
    //0x66 ~ 0x77 is Undefined
    ChannelModeMessage1_AllSoundOf = 0x78,
    ChannelModeMessage2_ResetAllControler = 0x79,
    ChannelModeMessage3_LocalControl = 0x7A,
    ChannelModeMessage4_AllNoteOff = 0x7B,
    ChannelModeMessage5_OmniOff = 0x7C,
    ChannelModeMessage6_OmniOn = 0x7D,
    ChannelModeMessage7_MonoModeOn = 0x7E,
    ChannelModeMessage8_PolyModeOn = 0x7F,
    Undefined,
}
//ref p.141
pub enum MetaEvent {
    //FF 00 02 ssss (16bit)
    SequenceNum(u16),
    //FF 01 len text
    TextEvent {
        len: u32,
        text: Vec<u8>,
    },
    //FF 02 len text
    Copyright {
        len: u32,
        text: Vec<u8>,
    },
    //FF 03 len text
    Sequence_TrackName {
        len: u32,
        text: Vec<u8>,
    },
    //FF 04 len text
    InstrumentalName {
        len: u32,
        text: Vec<u8>,
    },
    //FF 05 len text
    Lyrics {
        len: u32,
        text: Vec<u8>,
    },
    //FF 06 len text
    Marker {
        len: u32,
        text: Vec<u8>,
    },
    //FF 07 len text
    QueuePoint {
        len: u32,
        text: Vec<u8>,
    },
    //FF 20 01 cc
    MIDIChannelPlyfix(u8),
    //FF 2F 00
    EndOfTrack,
    //FF 51 03 tttttt(24bit)   (MIDI4分音符あたりのマイクロ秒)
    SetTempo(u32),
    //FF 54 hr mn se fr ff
    SMPTEOffset {
        hr: u8,
        mn: u8,
        se: u8,
        fr: u8,
        ff: u8,
    },
    //FF 58 04 nn dd cc bb
    TimeSignature {
        nn: u8,
        dd: u8,
        cc: u8,
        bb: u8,
    },
    //FF 59 02 sf mi
    KeySignature {
        sf: u8,
        mi: u8,
    },
    //FF 7F len data
    SequencerSpecifieMetaEvent {
        len: u32,
        data: Vec<u8>,
    },
    Unsupported {
        len: u32,
        data: Vec<u8>,
    },
}
pub enum SysMessage {
    /// F0H len .... F7H
    SysEx {
        len: u32,
        message: Vec<u8>,
    },
    // FsH 1111_0_sss
    SysCommon(SysCommonMessage),
    // FtH 1111_1_ttt
    SysRealTime(SysRealTimeMessage),
}

//ref p.109
pub enum SysCommonMessage {
    MIDI_Time_Code_Qtr_Frame { message_type: u8, values: u8 },
    Song_Position_Pointer { lsb: u8, msb: u8 },
    Song_Select { song_num: u8 },
    Undefined_F4,
    Undefined_F5,
    Tune_Request,
    End_of_SysEx,
}
impl SysCommonMessage {
    pub const STATUS_MIDI_Time_Code_Qtr_Frame: u8 = 0xF1;
    pub const STATUS_Song_Position_Pointer: u8 = 0xF2;
    pub const STATUS_Song_Select: u8 = 0xF3;
    pub const STATUS_Undefined_F4: u8 = 0xF4;
    pub const STATUS_Undefined_F5: u8 = 0xF5;
    pub const STATUS_Tune_Request: u8 = 0xF6;
    pub const STATUS_End_of_SysEx: u8 = 0xF7;
}
//ref p.110
pub enum SysRealTimeMessage {
    Timing_Clock,
    F9Undefined,
    Start,
    Continue,
    Stop,
    Active_Sensing,
    System_Reset,
    FDUndefined,
}
impl SysRealTimeMessage {
    pub const STATUS_Timing_Clock: u8 = 0xF8;
    pub const STATUS_F9Undefined: u8 = 0xF9;
    pub const STATUS_Start: u8 = 0xFA;
    pub const STATUS_Continue: u8 = 0xFB;
    pub const STATUS_Stop: u8 = 0xFC;
    pub const STATUS_Active_Sensing: u8 = 0xFE;
    pub const STATUS_FDUndefined: u8 = 0xFD;
    pub const STATUS_System_Reset: u8 = 0xFF;
}
