#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn dct64(out0: *mut libc::c_float, out1: *mut libc::c_float,
             samples: *mut libc::c_float);
    #[no_mangle]
    fn make_decode_tables(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn init_layer3_stuff(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn frame_buffers(fr: *mut mpg123_handle_t) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpg123_handle_s {
    pub fresh: libc::c_int,
    pub new_format: libc::c_int,
    pub hybrid_block: [[[libc::c_float; 576]; 2]; 2],
    pub hybrid_blc: [libc::c_int; 2],
    pub short_buffs: [[*mut libc::c_short; 2]; 2],
    pub float_buffs: [[*mut libc::c_float; 2]; 2],
    pub rawbuffs: *mut byte,
    pub rawbuffss: libc::c_int,
    pub bo: libc::c_int,
    pub rawdecwin: *mut byte,
    pub rawdecwins: libc::c_int,
    pub decwin: *mut libc::c_float,
    pub ssave: [byte; 34],
    pub halfphase: libc::c_int,
    pub longLimit: [[libc::c_int; 23]; 9],
    pub shortLimit: [[libc::c_int; 14]; 9],
    pub gainpow2: [libc::c_float; 378],
    pub synths: synth_t,
    pub verbose: libc::c_int,
    pub alloc: *const al_table_t,
    pub synth: func_synth,
    pub synth_stereo: func_synth_stereo,
    pub synth_mono: func_synth_mono,
    pub make_decode_tables: Option<unsafe extern "C" fn(_:
                                                            *mut mpg123_handle_t)
                                       -> ()>,
    pub stereo: libc::c_int,
    pub jsbound: libc::c_int,
    pub single: libc::c_int,
    pub II_sblimit: libc::c_int,
    pub down_sample_sblimit: libc::c_int,
    pub lsf: libc::c_int,
    pub mpeg25: libc::c_int,
    pub down_sample: libc::c_int,
    pub header_change: libc::c_int,
    pub lay: libc::c_int,
    pub spf: libc::c_long,
    pub do_layer: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t)
                             -> libc::c_int>,
    pub error_protection: libc::c_int,
    pub bitrate_index: libc::c_int,
    pub sampling_frequency: libc::c_int,
    pub padding: libc::c_int,
    pub extension: libc::c_int,
    pub mode: libc::c_int,
    pub mode_ext: libc::c_int,
    pub copyright: libc::c_int,
    pub original: libc::c_int,
    pub emphasis: libc::c_int,
    pub framesize: libc::c_int,
    pub freesize: libc::c_int,
    pub vbr: libc::c_int,
    pub num: mpg_off_t,
    pub input_offset: mpg_off_t,
    pub playnum: mpg_off_t,
    pub audio_start: mpg_off_t,
    pub state_flags: libc::c_int,
    pub silent_resync: libc::c_char,
    pub xing_toc: *mut byte,
    pub freeformat: libc::c_int,
    pub freeformat_framesize: libc::c_long,
    pub bitindex: libc::c_int,
    pub wordpointer: *mut byte,
    pub ultmp: ulong,
    pub uctmp: byte,
    pub maxoutburst: libc::c_double,
    pub lastscale: libc::c_double,
    pub rva: C2RustUnnamed_0,
    pub track_frames: mpg_off_t,
    pub track_samples: mpg_off_t,
    pub mean_framesize: libc::c_double,
    pub mean_frames: mpg_off_t,
    pub fsizeold: libc::c_int,
    pub ssize: libc::c_int,
    pub bitreservoir: uint,
    pub bsspace: [[byte; 3968]; 2],
    pub bsbuf: *mut byte,
    pub bsbufold: *mut byte,
    pub bsnum: libc::c_int,
    pub oldhead: ulong,
    pub firsthead: ulong,
    pub abr_rate: libc::c_int,
    pub index: frame_index_t,
    pub buffer: outbuffer_t,
    pub af: audioformat_t,
    pub own_buffer: libc::c_int,
    pub outblock: size_t,
    pub to_decode: libc::c_int,
    pub to_ignore: libc::c_int,
    pub firstframe: mpg_off_t,
    pub lastframe: mpg_off_t,
    pub ignoreframe: mpg_off_t,
    pub gapless_frames: mpg_off_t,
    pub firstoff: mpg_off_t,
    pub lastoff: mpg_off_t,
    pub begin_s: mpg_off_t,
    pub begin_os: mpg_off_t,
    pub end_s: mpg_off_t,
    pub end_os: mpg_off_t,
    pub fullend_os: mpg_off_t,
    pub crc: uint,
    pub rd: *mut reader_t,
    pub rdat: reader_data_t,
    pub p: mpg123_parm_t,
    pub err: libc::c_int,
    pub decoder_change: libc::c_int,
    pub delayed_change: libc::c_int,
    pub clip: libc::c_long,
    pub metaflags: libc::c_int,
    pub id3buf: [byte; 128],
    pub layerscratch: *mut libc::c_float,
    pub layer3: C2RustUnnamed,
    pub wrapperdata: *mut libc::c_void,
    pub wrapperclean: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub hybrid_in: *mut [[libc::c_float; 18]; 32],
    pub hybrid_out: *mut [[libc::c_float; 32]; 18],
}
pub type byte = libc::c_uchar;
pub type mpg123_parm_t = mpg123_parm_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpg123_parm_s {
    pub verbose: libc::c_int,
    pub flags: libc::c_long,
    pub down_sample: libc::c_int,
    pub rva: libc::c_int,
    pub halfspeed: libc::c_long,
    pub doublespeed: libc::c_long,
    pub timeout: libc::c_long,
    pub audio_caps: [[[libc::c_char; 2]; 10]; 2],
    pub outscale: libc::c_double,
    pub resync_limit: libc::c_long,
    pub index_size: libc::c_long,
    pub preframes: libc::c_long,
    pub feedpool: libc::c_long,
    pub feedbuffer: libc::c_long,
}
pub type reader_data_t = reader_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reader_data_s {
    pub filelen: mpg_off_t,
    pub filepos: mpg_off_t,
    pub filept: libc::c_int,
    pub iohandle: *mut libc::c_void,
    pub flags: libc::c_int,
    pub timeout_sec: libc::c_long,
    pub fdread: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                            _: *mut libc::c_void, _: size_t)
                           -> mpg_ssize_t>,
    pub r_read: Option<unsafe extern "C" fn(_: libc::c_int,
                                            _: *mut libc::c_void, _: size_t)
                           -> mpg_ssize_t>,
    pub r_lseek: Option<unsafe extern "C" fn(_: libc::c_int, _: mpg_off_t,
                                             _: libc::c_int) -> mpg_off_t>,
    pub r_read_handle: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                   _: *mut libc::c_void,
                                                   _: size_t) -> mpg_ssize_t>,
    pub r_lseek_handle: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                    _: mpg_off_t,
                                                    _: libc::c_int)
                                   -> mpg_off_t>,
    pub cleanup_handle: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()>,
    pub read: Option<unsafe extern "C" fn(_: libc::c_int,
                                          _: *mut libc::c_void, _: size_t)
                         -> mpg_ssize_t>,
    pub lseek: Option<unsafe extern "C" fn(_: libc::c_int, _: mpg_off_t,
                                           _: libc::c_int) -> mpg_off_t>,
    pub fullread: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                              _: *mut byte, _: mpg_ssize_t)
                             -> mpg_ssize_t>,
    pub buffer: bufferchain_t,
}
pub type bufferchain_t = bufferchain_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufferchain_s {
    pub first: *mut buffy_s,
    pub last: *mut buffy_s,
    pub size: mpg_ssize_t,
    pub pos: mpg_ssize_t,
    pub firstpos: mpg_ssize_t,
    pub fileoff: mpg_off_t,
    pub bufblock: size_t,
    pub pool_size: size_t,
    pub pool_fill: size_t,
    pub pool: *mut buffy_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffy_s {
    pub data: *mut byte,
    pub size: mpg_ssize_t,
    pub realsize: mpg_ssize_t,
    pub next: *mut buffy_s,
}
pub type mpg_ssize_t = ssize_t;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type mpg_off_t = libc::c_long;
pub type mpg123_handle_t = mpg123_handle_s;
pub type reader_t = reader_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reader_s {
    pub init: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t)
                         -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t) -> ()>,
    pub fullread: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                              _: *mut byte, _: mpg_ssize_t)
                             -> mpg_ssize_t>,
    pub head_read: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                               _: *mut ulong) -> libc::c_int>,
    pub head_shift: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                _: *mut ulong)
                               -> libc::c_int>,
    pub skip_bytes: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                _: mpg_off_t) -> mpg_off_t>,
    pub read_frame_body: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                     _: *mut byte,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
    pub back_bytes: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                _: mpg_off_t) -> libc::c_int>,
    pub seek_frame: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t,
                                                _: mpg_off_t) -> libc::c_int>,
    pub tell: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t)
                         -> mpg_off_t>,
    pub rewind: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t) -> ()>,
    pub forget: Option<unsafe extern "C" fn(_: *mut mpg123_handle_t) -> ()>,
}
pub type ulong = libc::c_ulong;
pub type uint = libc::c_uint;
pub type audioformat_t = audioformat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audioformat_s {
    pub encoding: libc::c_int,
    pub encsize: libc::c_int,
    pub dec_enc: libc::c_int,
    pub dec_encsize: libc::c_int,
    pub channels: libc::c_int,
    pub rate: libc::c_int,
}
pub type outbuffer_t = outbuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outbuffer_s {
    pub data: *mut byte,
    pub p: *mut byte,
    pub fill: size_t,
    pub size: size_t,
    pub rdata: *mut byte,
}
pub type frame_index_t = frame_index_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_index_s {
    pub data: *mut mpg_off_t,
    pub step: mpg_off_t,
    pub next: mpg_off_t,
    pub size: size_t,
    pub fill: size_t,
    pub grow_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub level: [libc::c_int; 2],
    pub gain: [libc::c_float; 2],
    pub peak: [libc::c_float; 2],
}
pub type func_synth_mono
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                _: *mut mpg123_handle_t) -> libc::c_int>;
pub type func_synth_stereo
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_float, _: *mut libc::c_float,
                                _: *mut mpg123_handle_t) -> libc::c_int>;
pub type func_synth
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_float, _: libc::c_int,
                                _: *mut mpg123_handle_t, _: libc::c_int)
               -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct al_table_t {
    pub bits: libc::c_short,
    pub d: libc::c_short,
}
pub type synth_t = synth_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct synth_s {
    pub plain: [[func_synth; 1]; 1],
    pub stereo: [[func_synth_stereo; 1]; 1],
    pub mono2stereo: [[func_synth_mono; 1]; 1],
    pub mono: [[func_synth_mono; 1]; 1],
}
pub type mpg123_enc_enum = libc::c_uint;
pub const MPG123_ENC_ALAW_8: mpg123_enc_enum = 8;
pub const MPG123_ENC_ULAW_8: mpg123_enc_enum = 4;
pub const MPG123_ENC_SIGNED_8: mpg123_enc_enum = 130;
pub const MPG123_ENC_UNSIGNED_8: mpg123_enc_enum = 1;
pub const MPG123_ENC_UNSIGNED_16: mpg123_enc_enum = 96;
pub const MPG123_ENC_SIGNED_16: mpg123_enc_enum = 208;
pub const MPG123_ENC_SIGNED: mpg123_enc_enum = 128;
pub const MPG123_ENC_16: mpg123_enc_enum = 64;
pub const MPG123_ENC_8: mpg123_enc_enum = 15;
pub type synth_resample = libc::c_int;
pub const r_limit: synth_resample = 1;
pub const r_1to1: synth_resample = 0;
pub const r_none: synth_resample = -1;
pub type synth_format = libc::c_int;
pub const f_limit: synth_format = 1;
pub const f_16: synth_format = 0;
pub const f_none: synth_format = -1;
pub const MPG123_ERR: mpg123_errors = -1;
pub const MPG123_NO_BUFFERS: mpg123_errors = 11;
pub const MPG123_BAD_DECODER_SETUP: mpg123_errors = 37;
pub const MPG123_OK: mpg123_errors = 0;
pub const nodec: optdec = 2;
pub type optdec = libc::c_uint;
pub const generic: optdec = 1;
pub const autodec: optdec = 0;
pub type mpg123_errors = libc::c_int;
pub const MPG123_INT_OVERFLOW: mpg123_errors = 43;
pub const MPG123_LFS_OVERFLOW: mpg123_errors = 42;
pub const MPG123_BAD_CUSTOM_IO: mpg123_errors = 41;
pub const MPG123_LSEEK_FAILED: mpg123_errors = 40;
pub const MPG123_BAD_VALUE: mpg123_errors = 39;
pub const MPG123_MISSING_FEATURE: mpg123_errors = 38;
pub const MPG123_INDEX_FAIL: mpg123_errors = 36;
pub const MPG123_NO_INDEX: mpg123_errors = 35;
pub const MPG123_BAD_KEY: mpg123_errors = 34;
pub const MPG123_NULL_POINTER: mpg123_errors = 33;
pub const MPG123_NO_RELSEEK: mpg123_errors = 32;
pub const MPG123_NULL_BUFFER: mpg123_errors = 31;
pub const MPG123_BAD_ALIGN: mpg123_errors = 30;
pub const MPG123_NO_8BIT: mpg123_errors = 29;
pub const MPG123_RESYNC_FAIL: mpg123_errors = 28;
pub const MPG123_OUT_OF_SYNC: mpg123_errors = 27;
pub const MPG123_BAD_INDEX_PAR: mpg123_errors = 26;
pub const MPG123_BAD_PARS: mpg123_errors = 25;
pub const MPG123_NO_READER: mpg123_errors = 24;
pub const MPG123_NO_SEEK: mpg123_errors = 23;
pub const MPG123_BAD_FILE: mpg123_errors = 22;
pub const MPG123_NO_TIMEOUT: mpg123_errors = 21;
pub const MPG123_BAD_WHENCE: mpg123_errors = 20;
pub const MPG123_NO_SEEK_FROM_END: mpg123_errors = 19;
pub const MPG123_ERR_READER: mpg123_errors = 18;
pub const MPG123_ERR_NULL: mpg123_errors = 17;
pub const MPG123_BAD_BAND: mpg123_errors = 16;
pub const MPG123_BAD_TYPES: mpg123_errors = 15;
pub const MPG123_NO_SPACE: mpg123_errors = 14;
pub const MPG123_NO_GAPLESS: mpg123_errors = 13;
pub const MPG123_BAD_RVA: mpg123_errors = 12;
pub const MPG123_BAD_HANDLE: mpg123_errors = 10;
pub const MPG123_BAD_DECODER: mpg123_errors = 9;
pub const MPG123_NOT_INITIALIZED: mpg123_errors = 8;
pub const MPG123_OUT_OF_MEM: mpg123_errors = 7;
pub const MPG123_BAD_BUFFER: mpg123_errors = 6;
pub const MPG123_BAD_PARAM: mpg123_errors = 5;
pub const MPG123_ERR_16TO8TABLE: mpg123_errors = 4;
pub const MPG123_BAD_RATE: mpg123_errors = 3;
pub const MPG123_BAD_CHANNEL: mpg123_errors = 2;
pub const MPG123_BAD_OUTFORMAT: mpg123_errors = 1;
pub const MPG123_NEED_MORE: mpg123_errors = -10;
pub const MPG123_NEW_FORMAT: mpg123_errors = -11;
pub const MPG123_DONE: mpg123_errors = -12;
// one decoding block is 64 samples.
// main synth function, uses the plain dct64
#[no_mangle]
pub unsafe extern "C" fn synth_1to1(mut bandPtr: *mut libc::c_float,
                                    mut channel: libc::c_int,
                                    mut fr: *mut mpg123_handle_t,
                                    mut final_0: libc::c_int) -> libc::c_int {
    static mut step: libc::c_int = 2 as libc::c_int; // (*buf)[0x110];
    let mut samples: *mut libc::c_short =
        (*fr).buffer.data.offset((*fr).buffer.fill as isize) as
            *mut libc::c_short;
    let mut b0: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut buf: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut clip: libc::c_int = 0 as libc::c_int;
    let mut bo1: libc::c_int = 0;
    if channel == 0 {
        (*fr).bo -= 1;
        (*fr).bo &= 0xf as libc::c_int;
        buf = (*fr).float_buffs[0 as libc::c_int as usize].as_mut_ptr()
    } else {
        samples = samples.offset(1);
        buf = (*fr).float_buffs[1 as libc::c_int as usize].as_mut_ptr()
    }
    if (*fr).bo & 0x1 as libc::c_int != 0 {
        b0 = *buf.offset(0 as libc::c_int as isize);
        bo1 = (*fr).bo;
        dct64((*buf.offset(1 as libc::c_int as
                               isize)).offset(((*fr).bo + 1 as libc::c_int &
                                                   0xf as libc::c_int) as
                                                  isize),
              (*buf.offset(0 as libc::c_int as
                               isize)).offset((*fr).bo as isize), bandPtr);
    } else {
        b0 = *buf.offset(1 as libc::c_int as isize);
        bo1 = (*fr).bo + 1 as libc::c_int;
        dct64((*buf.offset(0 as libc::c_int as
                               isize)).offset((*fr).bo as isize),
              (*buf.offset(1 as libc::c_int as
                               isize)).offset((*fr).bo as
                                                  isize).offset(1 as
                                                                    libc::c_int
                                                                    as isize),
              bandPtr);
    }
    let mut window: *mut libc::c_float =
        (*fr).decwin.offset(16 as libc::c_int as
                                isize).offset(-(bo1 as isize));
    let mut j: libc::c_int = 0;
    j = 0x40 as libc::c_int / 4 as libc::c_int;
    while j != 0 {
        let mut sum: libc::c_float = 0.;
        let fresh0 = window;
        window = window.offset(1);
        let fresh1 = b0;
        b0 = b0.offset(1);
        sum = *fresh0 * *fresh1;
        let fresh2 = window;
        window = window.offset(1);
        let fresh3 = b0;
        b0 = b0.offset(1);
        sum -= *fresh2 * *fresh3;
        let fresh4 = window;
        window = window.offset(1);
        let fresh5 = b0;
        b0 = b0.offset(1);
        sum += *fresh4 * *fresh5;
        let fresh6 = window;
        window = window.offset(1);
        let fresh7 = b0;
        b0 = b0.offset(1);
        sum -= *fresh6 * *fresh7;
        let fresh8 = window;
        window = window.offset(1);
        let fresh9 = b0;
        b0 = b0.offset(1);
        sum += *fresh8 * *fresh9;
        let fresh10 = window;
        window = window.offset(1);
        let fresh11 = b0;
        b0 = b0.offset(1);
        sum -= *fresh10 * *fresh11;
        let fresh12 = window;
        window = window.offset(1);
        let fresh13 = b0;
        b0 = b0.offset(1);
        sum += *fresh12 * *fresh13;
        let fresh14 = window;
        window = window.offset(1);
        let fresh15 = b0;
        b0 = b0.offset(1);
        sum -= *fresh14 * *fresh15;
        let fresh16 = window;
        window = window.offset(1);
        let fresh17 = b0;
        b0 = b0.offset(1);
        sum += *fresh16 * *fresh17;
        let fresh18 = window;
        window = window.offset(1);
        let fresh19 = b0;
        b0 = b0.offset(1);
        sum -= *fresh18 * *fresh19;
        let fresh20 = window;
        window = window.offset(1);
        let fresh21 = b0;
        b0 = b0.offset(1);
        sum += *fresh20 * *fresh21;
        let fresh22 = window;
        window = window.offset(1);
        let fresh23 = b0;
        b0 = b0.offset(1);
        sum -= *fresh22 * *fresh23;
        let fresh24 = window;
        window = window.offset(1);
        let fresh25 = b0;
        b0 = b0.offset(1);
        sum += *fresh24 * *fresh25;
        let fresh26 = window;
        window = window.offset(1);
        let fresh27 = b0;
        b0 = b0.offset(1);
        sum -= *fresh26 * *fresh27;
        let fresh28 = window;
        window = window.offset(1);
        let fresh29 = b0;
        b0 = b0.offset(1);
        sum += *fresh28 * *fresh29;
        let fresh30 = window;
        window = window.offset(1);
        let fresh31 = b0;
        b0 = b0.offset(1);
        sum -= *fresh30 * *fresh31;
        if sum > 32767.0f32 {
            *samples = 0x7fff as libc::c_int as libc::c_short;
            clip += 1
        } else if sum < -32768.0f32 {
            *samples = -(0x8000 as libc::c_int) as libc::c_short;
            clip += 1
        } else {
            *samples =
                if sum > 0.0f32 { (sum) + 0.5f32 } else { (sum) - 0.5f32 } as
                    libc::c_short
        }
        j -= 1;
        b0 =
            b0.offset((0x400 as libc::c_int / 0x40 as libc::c_int -
                           0x10 as libc::c_int) as isize);
        window =
            window.offset((0x800 as libc::c_int / 0x40 as libc::c_int -
                               0x10 as libc::c_int) as isize);
        samples = samples.offset(step as isize)
    }
    let mut sum_0: libc::c_float = 0.;
    sum_0 =
        *window.offset(0 as libc::c_int as isize) *
            *b0.offset(0 as libc::c_int as isize);
    sum_0 +=
        *window.offset(0x2 as libc::c_int as isize) *
            *b0.offset(0x2 as libc::c_int as isize);
    sum_0 +=
        *window.offset(0x4 as libc::c_int as isize) *
            *b0.offset(0x4 as libc::c_int as isize);
    sum_0 +=
        *window.offset(0x6 as libc::c_int as isize) *
            *b0.offset(0x6 as libc::c_int as isize);
    sum_0 +=
        *window.offset(0x8 as libc::c_int as isize) *
            *b0.offset(0x8 as libc::c_int as isize);
    sum_0 +=
        *window.offset(0xa as libc::c_int as isize) *
            *b0.offset(0xa as libc::c_int as isize);
    sum_0 +=
        *window.offset(0xc as libc::c_int as isize) *
            *b0.offset(0xc as libc::c_int as isize);
    sum_0 +=
        *window.offset(0xe as libc::c_int as isize) *
            *b0.offset(0xe as libc::c_int as isize);
    if sum_0 > 32767.0f32 {
        *samples = 0x7fff as libc::c_int as libc::c_short;
        clip += 1
    } else if sum_0 < -32768.0f32 {
        *samples = -(0x8000 as libc::c_int) as libc::c_short;
        clip += 1
    } else {
        *samples =
            if sum_0 > 0.0f32 { (sum_0) + 0.5f32 } else { (sum_0) - 0.5f32 }
                as libc::c_short
    }
    samples = samples.offset(step as isize);
    b0 = b0.offset(-((0x400 as libc::c_int / 0x40 as libc::c_int) as isize));
    window =
        window.offset(-((0x800 as libc::c_int / 0x40 as libc::c_int) as
                            isize));
    window = window.offset((bo1 << 1 as libc::c_int) as isize);
    j = 0x40 as libc::c_int / 4 as libc::c_int - 1 as libc::c_int;
    while j != 0 {
        let mut sum_1: libc::c_float = 0.;
        window = window.offset(-1);
        let fresh32 = b0;
        b0 = b0.offset(1);
        sum_1 = -(*window * *fresh32);
        window = window.offset(-1);
        let fresh33 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh33;
        window = window.offset(-1);
        let fresh34 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh34;
        window = window.offset(-1);
        let fresh35 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh35;
        window = window.offset(-1);
        let fresh36 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh36;
        window = window.offset(-1);
        let fresh37 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh37;
        window = window.offset(-1);
        let fresh38 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh38;
        window = window.offset(-1);
        let fresh39 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh39;
        window = window.offset(-1);
        let fresh40 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh40;
        window = window.offset(-1);
        let fresh41 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh41;
        window = window.offset(-1);
        let fresh42 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh42;
        window = window.offset(-1);
        let fresh43 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh43;
        window = window.offset(-1);
        let fresh44 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh44;
        window = window.offset(-1);
        let fresh45 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh45;
        window = window.offset(-1);
        let fresh46 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh46;
        window = window.offset(-1);
        let fresh47 = b0;
        b0 = b0.offset(1);
        sum_1 -= *window * *fresh47;
        if sum_1 > 32767.0f32 {
            *samples = 0x7fff as libc::c_int as libc::c_short;
            clip += 1
        } else if sum_1 < -32768.0f32 {
            *samples = -(0x8000 as libc::c_int) as libc::c_short;
            clip += 1
        } else {
            *samples =
                if sum_1 > 0.0f32 {
                    (sum_1) + 0.5f32
                } else { (sum_1) - 0.5f32 } as libc::c_short
        }
        j -= 1;
        b0 =
            b0.offset(-((0x400 as libc::c_int / 0x40 as libc::c_int +
                             0x10 as libc::c_int) as isize));
        window =
            window.offset(-((0x800 as libc::c_int / 0x40 as libc::c_int -
                                 0x10 as libc::c_int) as isize));
        samples = samples.offset(step as isize)
    }
    if final_0 != 0 {
        (*fr).buffer.fill =
            ((*fr).buffer.fill as
                 libc::c_ulong).wrapping_add((0x40 as libc::c_int as
                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                                                  as
                                                                                  libc::c_ulong))
                as size_t as size_t
    }
    return clip;
}
// the call of left and right plain synth, wrapped.
// this may be replaced by a direct stereo optimized synth.
unsafe extern "C" fn synth_stereo(mut bandPtr_l: *mut libc::c_float,
                                  mut bandPtr_r: *mut libc::c_float,
                                  mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut clip: libc::c_int = 0;
    clip =
        (*fr).synth.expect("non-null function pointer")(bandPtr_l,
                                                        0 as libc::c_int, fr,
                                                        0 as libc::c_int);
    clip +=
        (*fr).synth.expect("non-null function pointer")(bandPtr_r,
                                                        1 as libc::c_int, fr,
                                                        1 as libc::c_int);
    return clip;
}
// mono to stereo synth, wrapping over synth_1to1
#[no_mangle]
pub unsafe extern "C" fn synth_1to1_m2s(mut bandPtr: *mut libc::c_float,
                                        mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut samples: *mut byte = (*fr).buffer.data;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    ret = synth_1to1(bandPtr, 0 as libc::c_int, fr, 1 as libc::c_int);
    samples =
        samples.offset((*fr).buffer.fill.wrapping_sub((0x40 as libc::c_int as
                                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                                                           as
                                                                                           libc::c_ulong))
                           as isize);
    i = 0 as libc::c_int;
    while i < 0x40 as libc::c_int / 2 as libc::c_int {
        *(samples as *mut libc::c_short).offset(1 as libc::c_int as isize) =
            *(samples as
                  *mut libc::c_short).offset(0 as libc::c_int as isize);
        samples =
            samples.offset((2 as libc::c_int as
                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                                as
                                                                libc::c_ulong)
                               as isize);
        i += 1
    }
    return ret;
}
// mono synth, wrapping over synth_1to1
#[no_mangle]
pub unsafe extern "C" fn synth_1to1_mono(mut bandPtr: *mut libc::c_float,
                                         mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut samples_tmp: [libc::c_short; 64] = [0; 64];
    let mut tmp1: *mut libc::c_short = samples_tmp.as_mut_ptr();
    let mut samples: *mut byte = (*fr).buffer.data;
    let mut pnt: libc::c_int = (*fr).buffer.fill as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    // save buffer stuff, trick samples_tmp into there, decode, restore
    (*fr).buffer.data =
        samples_tmp.as_mut_ptr() as *mut byte; // decode into samples_tmp
    (*fr).buffer.fill = 0 as libc::c_int as size_t; // restore original value
    ret = synth_1to1(bandPtr, 0 as libc::c_int, fr, 0 as libc::c_int);
    (*fr).buffer.data = samples;
    // now append samples from samples_tmp
    samples =
        samples.offset(pnt as isize); // just the next mem in frame buffer
    i = 0 as libc::c_int;
    while i < 0x40 as libc::c_int / 2 as libc::c_int {
        *(samples as *mut libc::c_short) = *tmp1;
        samples =
            samples.offset(::std::mem::size_of::<libc::c_short>() as
                               libc::c_ulong as isize);
        tmp1 = tmp1.offset(2 as libc::c_int as isize);
        i += 1
    }
    (*fr).buffer.fill =
        (pnt as
             libc::c_ulong).wrapping_add(((0x40 as libc::c_int /
                                               2 as libc::c_int) as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                                              as
                                                                              libc::c_ulong));
    return ret;
}
static mut synth_base: synth_s =
    unsafe {
        {
            let mut init =
                synth_s{plain:
                            [[Some(synth_1to1 as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_float,
                                                            _: libc::c_int,
                                                            _:
                                                                *mut mpg123_handle_t,
                                                            _: libc::c_int)
                                           -> libc::c_int)]],
                        stereo:
                            [[Some(synth_stereo as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_float,
                                                            _:
                                                                *mut libc::c_float,
                                                            _:
                                                                *mut mpg123_handle_t)
                                           -> libc::c_int)]],
                        mono2stereo:
                            [[Some(synth_1to1_m2s as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_float,
                                                            _:
                                                                *mut mpg123_handle_t)
                                           -> libc::c_int)]],
                        mono:
                            [[Some(synth_1to1_mono as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_float,
                                                            _:
                                                                *mut mpg123_handle_t)
                                           -> libc::c_int)]],};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn init_synth(mut fr: *mut mpg123_handle_t) {
    (*fr).synths = synth_base;
}
unsafe extern "C" fn find_synth(mut synth: func_synth,
                                mut synths: *const [func_synth; 1])
 -> libc::c_int {
    let mut ri: synth_resample = r_1to1;
    let mut fi: synth_format = f_16;
    ri = r_1to1;
    while (ri as libc::c_int) < r_limit as libc::c_int {
        fi = f_16;
        while (fi as libc::c_int) < f_limit as libc::c_int {
            if synth == (*synths.offset(ri as isize))[fi as usize] {
                return (0 as libc::c_int == 0) as libc::c_int
            }
            fi += 1
        }
        ri += 1
    }
    return 0 as libc::c_int;
}
// determine what kind of decoder is actually active
// this depends on runtime choices which may cause fallback to i386 or generic code.
unsafe extern "C" fn find_dectype(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut type_0: optdec = nodec;
    let mut basic_synth: func_synth = (*fr).synth;
    if find_synth(basic_synth, synth_base.plain.as_ptr()) != 0 {
        type_0 = generic
    }
    if type_0 as libc::c_uint != nodec as libc::c_int as libc::c_uint {
        return MPG123_OK as libc::c_int
    } else {
        (*fr).err = MPG123_BAD_DECODER_SETUP as libc::c_int;
        return MPG123_ERR as libc::c_int
    };
}
// set synth functions for current frame
#[no_mangle]
pub unsafe extern "C" fn set_synth_functions(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut resample: synth_resample =
        r_none; // default is always 16bit, or whatever.
    let mut basic_format: synth_format = f_none;
    if (*fr).af.dec_enc & MPG123_ENC_16 as libc::c_int != 0 {
        basic_format = f_16
    }
    // make sure the chosen format is compiled into this lib.
    if basic_format as libc::c_int == f_none as libc::c_int {
        return -(1 as libc::c_int)
    }
    // be explicit about downsampling variant.
    match (*fr).down_sample { 0 => { resample = r_1to1 } _ => { } }
    if resample as libc::c_int == r_none as libc::c_int {
        return -(1 as libc::c_int)
    }
    // finally selecting the synth functions for stereo / mono.
    (*fr).synth =
        (*fr).synths.plain[resample as usize][basic_format as usize];
    (*fr).synth_stereo =
        (*fr).synths.stereo[resample as usize][basic_format as usize];
    (*fr).synth_mono =
        if (*fr).af.channels == 2 as libc::c_int {
            (*fr).synths.mono2stereo[resample as usize][basic_format as usize]
        } else {
            (*(*fr).synths.mono.as_mut_ptr().offset(resample as
                                                        isize))[basic_format
                                                                    as usize]
        };
    if find_dectype(fr) != MPG123_OK as libc::c_int {
        (*fr).err = MPG123_BAD_DECODER_SETUP as libc::c_int;
        return MPG123_ERR as libc::c_int
    }
    if frame_buffers(fr) != 0 as libc::c_int {
        (*fr).err = MPG123_NO_BUFFERS as libc::c_int;
        return MPG123_ERR as libc::c_int
    }
    init_layer3_stuff(fr);
    (*fr).make_decode_tables =
        Some(make_decode_tables as
                 unsafe extern "C" fn(_: *mut mpg123_handle_t) -> ());
    // we allocated the table buffers just now, so (re)create the tables.
    (*fr).make_decode_tables.expect("non-null function pointer")(fr);
    return 0 as libc::c_int;
}
