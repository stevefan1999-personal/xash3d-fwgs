#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn fi_add(fi: *mut frame_index_t, pos: mpg_off_t);
    #[no_mangle]
    fn frame_gapless_init(fr: *mut mpg123_handle_t, framecount: mpg_off_t,
                          bskip: mpg_off_t, eskip: mpg_off_t);
    #[no_mangle]
    fn frame_fill_toc(fr: *mut mpg123_handle_t, in_0: *mut byte)
     -> libc::c_int;
    #[no_mangle]
    fn do_rva(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn do_layer3(fr: *mut mpg123_handle_t) -> libc::c_int;
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
pub type word = libc::c_ushort;
pub type mpg123_errors = libc::c_int;
pub const MPG123_INT_OVERFLOW: mpg123_errors = 43;
pub const MPG123_LFS_OVERFLOW: mpg123_errors = 42;
pub const MPG123_BAD_CUSTOM_IO: mpg123_errors = 41;
pub const MPG123_LSEEK_FAILED: mpg123_errors = 40;
pub const MPG123_BAD_VALUE: mpg123_errors = 39;
pub const MPG123_MISSING_FEATURE: mpg123_errors = 38;
pub const MPG123_BAD_DECODER_SETUP: mpg123_errors = 37;
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
pub const MPG123_NO_BUFFERS: mpg123_errors = 11;
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
pub const MPG123_OK: mpg123_errors = 0;
pub const MPG123_ERR: mpg123_errors = -1;
pub const MPG123_NEED_MORE: mpg123_errors = -10;
pub const MPG123_NEW_FORMAT: mpg123_errors = -11;
pub const MPG123_DONE: mpg123_errors = -12;
pub type mpg123_param_flags = libc::c_uint;
pub const MPG123_AUTO_RESAMPLE: mpg123_param_flags = 32768;
pub const MPG123_IGNORE_INFOFRAME: mpg123_param_flags = 16384;
pub const MPG123_IGNORE_STREAMLENGTH: mpg123_param_flags = 4096;
pub const MPG123_FUZZY: mpg123_param_flags = 512;
pub const MPG123_SEEKBUFFER: mpg123_param_flags = 256;
pub const MPG123_NO_RESYNC: mpg123_param_flags = 128;
pub const MPG123_GAPLESS: mpg123_param_flags = 64;
pub const MPG123_QUIET: mpg123_param_flags = 32;
pub const MPG123_FORCE_STEREO: mpg123_param_flags = 8;
pub const MPG123_MONO_MIX: mpg123_param_flags = 4;
pub const MPG123_MONO_RIGHT: mpg123_param_flags = 2;
pub const MPG123_MONO_LEFT: mpg123_param_flags = 1;
pub const MPG123_FORCE_MONO: mpg123_param_flags = 7;
pub type frame_state_flags = libc::c_uint;
pub const FRAME_FRESH_DECODER: frame_state_flags = 4;
pub const FRAME_FRANKENSTEIN: frame_state_flags = 2;
pub const FRAME_ACCURATE: frame_state_flags = 1;
pub type mpg123_vbr = libc::c_uint;
pub const MPG123_ABR: mpg123_vbr = 2;
pub const MPG123_VBR: mpg123_vbr = 1;
pub const MPG123_CBR: mpg123_vbr = 0;
pub const PARSE_END: parse_codes = 10;
/* Not fine (invalid data). */
pub const PARSE_RESYNC: parse_codes = 2;
/* Header not good, go into resync. */
pub const PARSE_AGAIN: parse_codes = 3;
/* No more audio data to find. */
pub const PARSE_GOOD: parse_codes = 1;
/* Everything's fine. */
pub const PARSE_BAD: parse_codes = 0;
pub const PARSE_ERR: parse_codes = -1;
pub const PARSE_MORE: parse_codes = -10;
// PARSE_GOOD and PARSE_BAD have to be 1 and 0 (TRUE and FALSE), others can vary.
pub type parse_codes = libc::c_int;
unsafe extern "C" fn getbits(mut fr: *mut mpg123_handle_t,
                             mut number_of_bits: libc::c_int) -> uint {
    let mut rval: ulong = 0;
    rval = *(*fr).wordpointer.offset(0 as libc::c_int as isize) as ulong;
    rval <<= 8 as libc::c_int;
    rval |=
        *(*fr).wordpointer.offset(1 as libc::c_int as isize) as libc::c_ulong;
    rval <<= 8 as libc::c_int;
    rval |=
        *(*fr).wordpointer.offset(2 as libc::c_int as isize) as libc::c_ulong;
    rval <<= (*fr).bitindex;
    rval &= 0xffffff as libc::c_int as libc::c_ulong;
    (*fr).bitindex += number_of_bits;
    rval >>= 24 as libc::c_int - number_of_bits;
    (*fr).wordpointer =
        (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int) as
                                     isize);
    (*fr).bitindex &= 7 as libc::c_int;
    return rval as uint;
}
/* Really start over, throw away and read a new header, again. */
// bitrates for [mpeg1/2][layer]
static mut tabsel_123: [[[libc::c_int; 16]; 3]; 2] =
    [[[0 as libc::c_int, 32 as libc::c_int, 64 as libc::c_int,
       96 as libc::c_int, 128 as libc::c_int, 160 as libc::c_int,
       192 as libc::c_int, 224 as libc::c_int, 256 as libc::c_int,
       288 as libc::c_int, 320 as libc::c_int, 352 as libc::c_int,
       384 as libc::c_int, 416 as libc::c_int, 448 as libc::c_int, 0],
      [0 as libc::c_int, 32 as libc::c_int, 48 as libc::c_int,
       56 as libc::c_int, 64 as libc::c_int, 80 as libc::c_int,
       96 as libc::c_int, 112 as libc::c_int, 128 as libc::c_int,
       160 as libc::c_int, 192 as libc::c_int, 224 as libc::c_int,
       256 as libc::c_int, 320 as libc::c_int, 384 as libc::c_int, 0],
      [0 as libc::c_int, 32 as libc::c_int, 40 as libc::c_int,
       48 as libc::c_int, 56 as libc::c_int, 64 as libc::c_int,
       80 as libc::c_int, 96 as libc::c_int, 112 as libc::c_int,
       128 as libc::c_int, 160 as libc::c_int, 192 as libc::c_int,
       224 as libc::c_int, 256 as libc::c_int, 320 as libc::c_int, 0]],
     [[0 as libc::c_int, 32 as libc::c_int, 48 as libc::c_int,
       56 as libc::c_int, 64 as libc::c_int, 80 as libc::c_int,
       96 as libc::c_int, 112 as libc::c_int, 128 as libc::c_int,
       144 as libc::c_int, 160 as libc::c_int, 176 as libc::c_int,
       192 as libc::c_int, 224 as libc::c_int, 256 as libc::c_int, 0],
      [0 as libc::c_int, 8 as libc::c_int, 16 as libc::c_int,
       24 as libc::c_int, 32 as libc::c_int, 40 as libc::c_int,
       48 as libc::c_int, 56 as libc::c_int, 64 as libc::c_int,
       80 as libc::c_int, 96 as libc::c_int, 112 as libc::c_int,
       128 as libc::c_int, 144 as libc::c_int, 160 as libc::c_int, 0],
      [0 as libc::c_int, 8 as libc::c_int, 16 as libc::c_int,
       24 as libc::c_int, 32 as libc::c_int, 40 as libc::c_int,
       48 as libc::c_int, 56 as libc::c_int, 64 as libc::c_int,
       80 as libc::c_int, 96 as libc::c_int, 112 as libc::c_int,
       128 as libc::c_int, 144 as libc::c_int, 160 as libc::c_int, 0]]];
static mut freqs: [libc::c_long; 9] =
    [44100 as libc::c_int as libc::c_long,
     48000 as libc::c_int as libc::c_long,
     32000 as libc::c_int as libc::c_long,
     22050 as libc::c_int as libc::c_long,
     24000 as libc::c_int as libc::c_long,
     16000 as libc::c_int as libc::c_long,
     11025 as libc::c_int as libc::c_long,
     12000 as libc::c_int as libc::c_long,
     8000 as libc::c_int as libc::c_long];
#[no_mangle]
pub unsafe extern "C" fn set_pointer(mut fr: *mut mpg123_handle_t,
                                     mut backstep: libc::c_long) {
    (*fr).wordpointer =
        (*fr).bsbuf.offset((*fr).ssize as isize).offset(-(backstep as isize));
    if backstep != 0 {
        memcpy((*fr).wordpointer as *mut libc::c_void,
               (*fr).bsbufold.offset((*fr).fsizeold as
                                         isize).offset(-(backstep as isize))
                   as *const libc::c_void, backstep as libc::c_ulong);
    }
    (*fr).bitindex = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn frame_bitrate(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    return tabsel_123[(*fr).lsf as
                          usize][((*fr).lay - 1 as libc::c_int) as
                                     usize][(*fr).bitrate_index as usize];
}
#[no_mangle]
pub unsafe extern "C" fn frame_freq(mut fr: *mut mpg123_handle_t)
 -> libc::c_long {
    return freqs[(*fr).sampling_frequency as usize];
}
#[no_mangle]
pub unsafe extern "C" fn compute_bpf(mut fr: *mut mpg123_handle_t)
 -> libc::c_double {
    let mut bpf: libc::c_double = 0.;
    match (*fr).lay {
        1 => {
            bpf =
                tabsel_123[(*fr).lsf as
                               usize][0 as libc::c_int as
                                          usize][(*fr).bitrate_index as usize]
                    as libc::c_double;
            bpf *= 12000.0f64 * 4.0f64;
            bpf /=
                (freqs[(*fr).sampling_frequency as usize] << (*fr).lsf) as
                    libc::c_double
        }
        2 | 3 => {
            bpf =
                tabsel_123[(*fr).lsf as
                               usize][((*fr).lay - 1 as libc::c_int) as
                                          usize][(*fr).bitrate_index as usize]
                    as libc::c_double;
            bpf *= 144000 as libc::c_int as libc::c_double;
            bpf /=
                (freqs[(*fr).sampling_frequency as usize] << (*fr).lsf) as
                    libc::c_double
        }
        _ => { bpf = 1.0f64 }
    }
    return bpf;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_spf(mut mh: *mut mpg123_handle_t)
 -> libc::c_int {
    if mh.is_null() { return MPG123_ERR as libc::c_int }
    return if (*mh).firsthead != 0 {
               (*mh).spf
           } else { MPG123_ERR as libc::c_int as libc::c_long } as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_tpf(mut fr: *mut mpg123_handle_t)
 -> libc::c_double {
    static mut bs: [libc::c_int; 4] =
        [0 as libc::c_int, 384 as libc::c_int, 1152 as libc::c_int,
         1152 as libc::c_int];
    let mut tpf: libc::c_double = 0.;
    if fr.is_null() || (*fr).firsthead == 0 {
        return MPG123_ERR as libc::c_int as libc::c_double
    }
    tpf = bs[(*fr).lay as usize] as libc::c_double;
    tpf /=
        (freqs[(*fr).sampling_frequency as usize] << (*fr).lsf) as
            libc::c_double;
    return tpf;
}
#[no_mangle]
pub unsafe extern "C" fn get_songlen(mut fr: *mut mpg123_handle_t,
                                     mut no: libc::c_int) -> libc::c_int {
    let mut tpf: libc::c_double = 0.;
    if fr.is_null() { return 0 as libc::c_int }
    if no < 0 as libc::c_int {
        if (*fr).rd.is_null() ||
               (*fr).rdat.filelen < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int
        }
        no =
            ((*fr).rdat.filelen as libc::c_double / compute_bpf(fr)) as
                libc::c_int
    }
    tpf = mpg123_tpf(fr);
    return (no as libc::c_double * tpf) as libc::c_int;
}
// just tell if the header is some mono.
unsafe extern "C" fn header_mono(mut newhead: ulong) -> libc::c_int {
    return if (newhead & 0xc0 as libc::c_int as libc::c_ulong) >>
                  6 as libc::c_int == 3 as libc::c_int as libc::c_ulong {
               (0 as libc::c_int == 0) as libc::c_int
           } else { 0 as libc::c_int };
}
unsafe extern "C" fn head_check(mut head: ulong) -> libc::c_int {
    if head & 0xffe00000 as libc::c_uint as libc::c_ulong !=
           0xffe00000 as libc::c_uint as libc::c_ulong ||
           (head & 0x60000 as libc::c_int as libc::c_ulong) >>
               17 as libc::c_int == 0 ||
           (head & 0xf000 as libc::c_int as libc::c_ulong) >>
               12 as libc::c_int == 0xf as libc::c_int as libc::c_ulong ||
           (head & 0xc00 as libc::c_int as libc::c_ulong) >> 10 as libc::c_int
               == 0x3 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
// true if the two headers will work with the same decoding routines
unsafe extern "C" fn head_compatible(mut fred: ulong, mut bret: ulong)
 -> libc::c_int {
    return (fred &
                (0xffe00000 as libc::c_uint |
                     0x180000 as libc::c_int as libc::c_uint |
                     0x60000 as libc::c_int as libc::c_uint |
                     0xc00 as libc::c_int as libc::c_uint) as libc::c_ulong ==
                bret &
                    (0xffe00000 as libc::c_uint |
                         0x180000 as libc::c_int as libc::c_uint |
                         0x60000 as libc::c_int as libc::c_uint |
                         0xc00 as libc::c_int as libc::c_uint) as
                        libc::c_ulong &&
                header_mono(fred) == header_mono(bret)) as libc::c_int;
}
// this is moderately sized buffers. Int offset is enough.
unsafe extern "C" fn bit_read_long(mut buf: *mut byte,
                                   mut offset: *mut libc::c_int) -> ulong {
    let mut val: ulong =
        (*buf.offset(*offset as isize) as ulong) << 24 as libc::c_int |
            (*buf.offset((*offset + 1 as libc::c_int) as isize) as ulong) <<
                16 as libc::c_int |
            (*buf.offset((*offset + 2 as libc::c_int) as isize) as ulong) <<
                8 as libc::c_int |
            *buf.offset((*offset + 3 as libc::c_int) as isize) as ulong;
    *offset += 4 as libc::c_int;
    return val;
}
unsafe extern "C" fn bit_read_short(mut buf: *mut byte,
                                    mut offset: *mut libc::c_int) -> word {
    let mut val: word =
        ((*buf.offset(*offset as isize) as word as libc::c_int) <<
             8 as libc::c_int |
             *buf.offset((*offset + 1 as libc::c_int) as isize) as word as
                 libc::c_int) as word;
    *offset += 2 as libc::c_int;
    return val;
}
unsafe extern "C" fn check_lame_tag(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut lame_offset: libc::c_int =
        if (*fr).stereo == 2 as libc::c_int {
            if (*fr).lsf != 0 { 17 as libc::c_int } else { 32 as libc::c_int }
        } else if (*fr).lsf != 0 {
            9 as libc::c_int
        } else { 17 as libc::c_int };
    let mut xing_flags: ulong = 0;
    let mut long_tmp: ulong = 0;
    let mut i: libc::c_int = 0;
    // going to look for Xing or Info at some position after the header
	//	                                   MPEG 1  MPEG 2/2.5 (LSF)
	//	Stereo, Joint Stereo, Dual Channel  32      17
	//	Mono                                17       9
    if !((*fr).p.flags &
             MPG123_IGNORE_INFOFRAME as libc::c_int as libc::c_long != 0) {
        // note: CRC or not, that does not matter here.
	// but, there is any combination of Xing flags in the wild. There are headers
	// without the search index table! I cannot assume a reasonable minimal size
	// for the actual data, have to check if each byte of information is present.
	// but: 4 B Info/Xing + 4 B flags is bare minimum.
        if !((*fr).framesize < lame_offset + 8 as libc::c_int) {
            // only search for tag when all zero before it (apart from checksum)
            i = 2 as libc::c_int;
            loop  {
                if !(i < lame_offset) {
                    current_block = 8515828400728868193;
                    break ;
                }
                if *(*fr).bsbuf.offset(i as isize) as libc::c_int !=
                       0 as libc::c_int {
                    current_block = 848226666458327087;
                    break ;
                }
                i += 1
            }
            match current_block {
                848226666458327087 => { }
                _ => {
                    if *(*fr).bsbuf.offset(lame_offset as isize) as
                           libc::c_int == 'I' as i32 &&
                           *(*fr).bsbuf.offset((lame_offset +
                                                    1 as libc::c_int) as
                                                   isize) as libc::c_int ==
                               'n' as i32 &&
                           *(*fr).bsbuf.offset((lame_offset +
                                                    2 as libc::c_int) as
                                                   isize) as libc::c_int ==
                               'f' as i32 &&
                           *(*fr).bsbuf.offset((lame_offset +
                                                    3 as libc::c_int) as
                                                   isize) as libc::c_int ==
                               'o' as i32 {
                        current_block = 12800627514080957624;
                    } else if *(*fr).bsbuf.offset(lame_offset as isize) as
                                  libc::c_int == 'X' as i32 &&
                                  *(*fr).bsbuf.offset((lame_offset +
                                                           1 as libc::c_int)
                                                          as isize) as
                                      libc::c_int == 'i' as i32 &&
                                  *(*fr).bsbuf.offset((lame_offset +
                                                           2 as libc::c_int)
                                                          as isize) as
                                      libc::c_int == 'n' as i32 &&
                                  *(*fr).bsbuf.offset((lame_offset +
                                                           3 as libc::c_int)
                                                          as isize) as
                                      libc::c_int == 'g' as i32 {
                        // Xing header means always VBR
                        (*fr).vbr = MPG123_VBR as libc::c_int;
                        current_block = 12800627514080957624;
                    } else { current_block = 848226666458327087; }
                    match current_block {
                        848226666458327087 => { }
                        _ =>
                        // we still have to see what there is
                        {
                            lame_offset += 4 as libc::c_int;
                            xing_flags =
                                bit_read_long((*fr).bsbuf, &mut lame_offset);
                            // from now on, I have to carefully check if the announced data is actually
	// there! I'm always returning 'yes', though.
                            if xing_flags & 1 as libc::c_int as libc::c_ulong
                                   != 0 {
                                // total bitstream frames
                                if (*fr).framesize <
                                       lame_offset + 4 as libc::c_int {
                                    current_block = 1216676218631520498;
                                } else {
                                    long_tmp =
                                        bit_read_long((*fr).bsbuf,
                                                      &mut lame_offset);
                                    if !((*fr).p.flags &
                                             MPG123_IGNORE_STREAMLENGTH as
                                                 libc::c_int as libc::c_long
                                             != 0) {
                                        // check for endless stream, but: TRACK_MAX_FRAMES sensible at all?
                                        (*fr).track_frames =
                                            if long_tmp >
                                                   (9223372036854775807 as
                                                        libc::c_long as
                                                        libc::c_ulong).wrapping_mul(2
                                                                                        as
                                                                                        libc::c_ulong).wrapping_add(1
                                                                                                                        as
                                                                                                                        libc::c_ulong).wrapping_div(4
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_ulong).wrapping_div(1152
                                                                                                                                                                                        as
                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                        as
                                                                                                                                                                                        libc::c_ulong)
                                               {
                                                0 as libc::c_int as
                                                    libc::c_long
                                            } else { long_tmp as mpg_off_t };
                                        // all or nothing: Only if encoder delay/padding is known, we'll cut
			// samples for gapless.
                                        if (*fr).p.flags &
                                               MPG123_GAPLESS as libc::c_int
                                                   as libc::c_long != 0 {
                                            frame_gapless_init(fr,
                                                               (*fr).track_frames,
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   mpg_off_t,
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   mpg_off_t);
                                        }
                                    }
                                    current_block = 10043043949733653460;
                                }
                            } else { current_block = 10043043949733653460; }
                            match current_block {
                                10043043949733653460 => {
                                    if xing_flags &
                                           0x2 as libc::c_int as libc::c_ulong
                                           != 0 {
                                        // total bitstream bytes
                                        if (*fr).framesize <
                                               lame_offset + 4 as libc::c_int
                                           {
                                            current_block =
                                                1216676218631520498;
                                        } else {
                                            long_tmp =
                                                bit_read_long((*fr).bsbuf,
                                                              &mut lame_offset);
                                            if !((*fr).p.flags &
                                                     MPG123_IGNORE_STREAMLENGTH
                                                         as libc::c_int as
                                                         libc::c_long != 0) {
                                                // the Xing bitstream length, at least as interpreted by the Lame
			// encoder, encompasses all data from the Xing header frame on,
			// ignoring leading ID3v2 data. Trailing tags (ID3v1) seem to be
			// included, though.
                                                if (*fr).rdat.filelen <
                                                       1 as libc::c_int as
                                                           libc::c_long {
                                                    (*fr).rdat.filelen =
                                                        long_tmp as mpg_off_t
                                                            +
                                                            (*fr).audio_start
                                                    // Overflow?
                                                }
                                            }
                                            current_block =
                                                14136749492126903395;
                                        }
                                    } else {
                                        current_block = 14136749492126903395;
                                    }
                                    match current_block {
                                        1216676218631520498 => { }
                                        _ => {
                                            if xing_flags &
                                                   0x4 as libc::c_int as
                                                       libc::c_ulong != 0 {
                                                // TOC
                                                if (*fr).framesize <
                                                       lame_offset +
                                                           100 as libc::c_int
                                                   {
                                                    current_block =
                                                        1216676218631520498;
                                                } else {
                                                    frame_fill_toc(fr,
                                                                   (*fr).bsbuf.offset(lame_offset
                                                                                          as
                                                                                          isize));
                                                    lame_offset +=
                                                        100 as libc::c_int;
                                                    current_block =
                                                        4090602189656566074;
                                                }
                                            } else {
                                                current_block =
                                                    4090602189656566074;
                                            }
                                            match current_block {
                                                1216676218631520498 => { }
                                                _ =>
                                                // VBR quality
                                                {
                                                    if xing_flags &
                                                           0x8 as libc::c_int
                                                               as
                                                               libc::c_ulong
                                                           != 0 {
                                                        if (*fr).framesize <
                                                               lame_offset +
                                                                   4 as
                                                                       libc::c_int
                                                           {
                                                            current_block =
                                                                1216676218631520498;
                                                        } else {
                                                            long_tmp =
                                                                bit_read_long((*fr).bsbuf,
                                                                              &mut lame_offset);
                                                            current_block =
                                                                18435049525520518667;
                                                        }
                                                    } else {
                                                        current_block =
                                                            18435049525520518667;
                                                    }
                                                    match current_block {
                                                        1216676218631520498 =>
                                                        {
                                                        }
                                                        _ =>
                                                        // either zeros/nothing, or:
	// 0-8: LAME3.90a
	// 9: revision/VBR method
	// 10: lowpass
	// 11-18: ReplayGain
	// 19: encoder flags
	// 20: ABR
	// 21-23: encoder delays
                                                        {
                                                            if !((*fr).framesize
                                                                     <
                                                                     lame_offset
                                                                         +
                                                                         24 as
                                                                             libc::c_int)
                                                               {
                                                                if *(*fr).bsbuf.offset(lame_offset
                                                                                           as
                                                                                           isize)
                                                                       as
                                                                       libc::c_int
                                                                       !=
                                                                       0 as
                                                                           libc::c_int
                                                                   {
                                                                    let mut lame_vbr:
                                                                            byte =
                                                                        0;
                                                                    let mut replay_gain:
                                                                            [libc::c_float; 2] =
                                                                        [0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_float,
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_float];
                                                                    let mut peak:
                                                                            libc::c_float =
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float; // I'm interested in 24 B of extra info.
                                                                    // final: 24 B LAME data
                                                                    let mut gain_offset:
                                                                            libc::c_float =
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float; // going to be +6 for old lame that used 83dB
                                                                    let mut nb:
                                                                            [libc::c_char; 10] =
                                                                        [0;
                                                                            10];
                                                                    let mut pad_in:
                                                                            mpg_off_t =
                                                                        0;
                                                                    let mut pad_out:
                                                                            mpg_off_t =
                                                                        0;
                                                                    memcpy(nb.as_mut_ptr()
                                                                               as
                                                                               *mut libc::c_void,
                                                                           (*fr).bsbuf.offset(lame_offset
                                                                                                  as
                                                                                                  isize)
                                                                               as
                                                                               *const libc::c_void,
                                                                           9
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong);
                                                                    nb[9 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                                        =
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_char;
                                                                    if strncmp(b"LAME\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               nb.as_mut_ptr(),
                                                                               4
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
                                                                           ==
                                                                           0 {
                                                                        let mut major:
                                                                                uint =
                                                                            0;
                                                                        let mut minor:
                                                                                uint =
                                                                            0;
                                                                        let mut rest:
                                                                                [libc::c_char; 6] =
                                                                            [0;
                                                                                6];
                                                                        rest[0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                                                            =
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_char;
                                                                        // Lame versions before 3.95.1 used 83 dB reference level, later
			// versions 89 dB. We stick with 89 dB as being "normal", adding 6 dB.
                                                                        if sscanf(nb.as_mut_ptr().offset(4
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             isize),
                                                                                  b"%u.%u%s\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char,
                                                                                  &mut major
                                                                                      as
                                                                                      *mut uint,
                                                                                  &mut minor
                                                                                      as
                                                                                      *mut uint,
                                                                                  rest.as_mut_ptr())
                                                                               >=
                                                                               2
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            // We cannot detect LAME 3.95 reliably (same version string as
				// 3.95.1), so this is a blind spot. Everything < 3.95 is safe, though.
                                                                            if major
                                                                                   <
                                                                                   3
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint
                                                                                   ||
                                                                                   major
                                                                                       ==
                                                                                       3
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint
                                                                                       &&
                                                                                       minor
                                                                                           <
                                                                                           95
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint
                                                                               {
                                                                                gain_offset
                                                                                    =
                                                                                    6
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_float
                                                                            }
                                                                        }
                                                                    } // 9 in
                                                                    lame_offset
                                                                        +=
                                                                        9 as
                                                                            libc::c_int;
                                                                    lame_vbr =
                                                                        (*(*fr).bsbuf.offset(lame_offset
                                                                                                 as
                                                                                                 isize)
                                                                             as
                                                                             libc::c_int
                                                                             &
                                                                             15
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            byte;
                                                                    lame_offset
                                                                        +=
                                                                        1 as
                                                                            libc::c_int;
                                                                    match lame_vbr
                                                                              as
                                                                              libc::c_int
                                                                        {
                                                                        1 | 8
                                                                        => {
                                                                            (*fr).vbr
                                                                                =
                                                                                MPG123_CBR
                                                                                    as
                                                                                    libc::c_int
                                                                            // the 4 big bits are tag revision, the small bits vbr method.
                                                                            // 10 in
                                                                            // from rev1 proposal... not sure if all good in practice
                                                                            // 00 ==unknown is taken as VBR
                                                                        }
                                                                        2 | 9
                                                                        => {
                                                                            (*fr).vbr
                                                                                =
                                                                                MPG123_ABR
                                                                                    as
                                                                                    libc::c_int
                                                                        }
                                                                        _ => {
                                                                            (*fr).vbr
                                                                                =
                                                                                MPG123_VBR
                                                                                    as
                                                                                    libc::c_int
                                                                        }
                                                                    } // 11 in, skipping lowpass filter value
                                                                    lame_offset
                                                                        +=
                                                                        1 as
                                                                            libc::c_int; // until better times arrived
                                                                    peak =
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float; // 15 in
                                                                    lame_offset
                                                                        +=
                                                                        4 as
                                                                            libc::c_int;
                                                                    i =
                                                                        0 as
                                                                            libc::c_int;
                                                                    while i <
                                                                              2
                                                                                  as
                                                                                  libc::c_int
                                                                          {
                                                                        let mut gt:
                                                                                byte =
                                                                            (*(*fr).bsbuf.offset(lame_offset
                                                                                                     as
                                                                                                     isize)
                                                                                 as
                                                                                 libc::c_int
                                                                                 >>
                                                                                 5
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                byte;
                                                                        let mut origin:
                                                                                byte =
                                                                            (*(*fr).bsbuf.offset(lame_offset
                                                                                                     as
                                                                                                     isize)
                                                                                 as
                                                                                 libc::c_int
                                                                                 >>
                                                                                 2
                                                                                     as
                                                                                     libc::c_int
                                                                                 &
                                                                                 0x7
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                byte;
                                                                        let mut factor:
                                                                                libc::c_float =
                                                                            if *(*fr).bsbuf.offset(lame_offset
                                                                                                       as
                                                                                                       isize)
                                                                                   as
                                                                                   libc::c_int
                                                                                   &
                                                                                   0x2
                                                                                       as
                                                                                       libc::c_int
                                                                                   !=
                                                                                   0
                                                                               {
                                                                                -0.1f32
                                                                            } else {
                                                                                0.1f32
                                                                            };
                                                                        // ReplayGain values - lame only writes radio mode gain...
		// 16bit gain, 3 bits name, 3 bits originator, sign (1=-, 0=+),
		// dB value * 10 in 9 bits (fixed point) ignore the setting if name or
		// originator == 000!
		// radio      0 0 1 0 1 1 1 0 0 1 1 1 1 1 0 1
		// audiophile 0 1 0 0 1 0 0 0 0 0 0 1 0 1 0 0
                                                                        let mut gain:
                                                                                word =
                                                                            (bit_read_short((*fr).bsbuf,
                                                                                            &mut lame_offset)
                                                                                 as
                                                                                 libc::c_int
                                                                                 &
                                                                                 0x1ff
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                word; // 19 in (2 cycles)
                                                                        if !(origin
                                                                                 as
                                                                                 libc::c_int
                                                                                 ==
                                                                                 0
                                                                                     as
                                                                                     libc::c_int
                                                                                 ||
                                                                                 (gt
                                                                                      as
                                                                                      libc::c_int)
                                                                                     <
                                                                                     1
                                                                                         as
                                                                                         libc::c_int
                                                                                 ||
                                                                                 gt
                                                                                     as
                                                                                     libc::c_int
                                                                                     >
                                                                                     2
                                                                                         as
                                                                                         libc::c_int)
                                                                           {
                                                                            gt
                                                                                =
                                                                                gt.wrapping_sub(1);
                                                                            replay_gain[gt
                                                                                            as
                                                                                            usize]
                                                                                =
                                                                                factor
                                                                                    *
                                                                                    gain
                                                                                        as
                                                                                        libc::c_float;
                                                                            // apply gain offset for automatic origin.
                                                                            if origin
                                                                                   as
                                                                                   libc::c_int
                                                                                   ==
                                                                                   3
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                replay_gain[gt
                                                                                                as
                                                                                                usize]
                                                                                    +=
                                                                                    gain_offset
                                                                            }
                                                                        } // TODO: use parsed peak?
                                                                        i += 1
                                                                    } // 20 in, skipping encoding flags byte
                                                                    i =
                                                                        0 as
                                                                            libc::c_int;
                                                                    while i <
                                                                              2
                                                                                  as
                                                                                  libc::c_int
                                                                          {
                                                                        if (*fr).rva.level[i
                                                                                               as
                                                                                               usize]
                                                                               <=
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            (*fr).rva.peak[i
                                                                                               as
                                                                                               usize]
                                                                                =
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_float;
                                                                            (*fr).rva.gain[i
                                                                                               as
                                                                                               usize]
                                                                                =
                                                                                replay_gain[i
                                                                                                as
                                                                                                usize];
                                                                            (*fr).rva.level[i
                                                                                                as
                                                                                                usize]
                                                                                =
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                        }
                                                                        i += 1
                                                                    }
                                                                    lame_offset
                                                                        +=
                                                                        1 as
                                                                            libc::c_int;
                                                                    if (*fr).vbr
                                                                           ==
                                                                           MPG123_ABR
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        (*fr).abr_rate
                                                                            =
                                                                            *(*fr).bsbuf.offset(lame_offset
                                                                                                    as
                                                                                                    isize)
                                                                                as
                                                                                libc::c_int
                                                                    }
                                                                    lame_offset
                                                                        +=
                                                                        1 as
                                                                            libc::c_int;
                                                                    pad_in =
                                                                        ((*(*fr).bsbuf.offset(lame_offset
                                                                                                  as
                                                                                                  isize)
                                                                              as
                                                                              libc::c_int)
                                                                             <<
                                                                             4
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             *(*fr).bsbuf.offset((lame_offset
                                                                                                      +
                                                                                                      1
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                     as
                                                                                                     isize)
                                                                                 as
                                                                                 libc::c_int
                                                                                 >>
                                                                                 4
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            mpg_off_t;
                                                                    pad_out =
                                                                        (((*(*fr).bsbuf.offset((lame_offset
                                                                                                    +
                                                                                                    1
                                                                                                        as
                                                                                                        libc::c_int)
                                                                                                   as
                                                                                                   isize)
                                                                               as
                                                                               libc::c_int)
                                                                              <<
                                                                              8
                                                                                  as
                                                                                  libc::c_int
                                                                              |
                                                                              *(*fr).bsbuf.offset((lame_offset
                                                                                                       +
                                                                                                       2
                                                                                                           as
                                                                                                           libc::c_int)
                                                                                                      as
                                                                                                      isize)
                                                                                  as
                                                                                  libc::c_int)
                                                                             &
                                                                             0xfff
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            mpg_off_t;
                                                                    lame_offset
                                                                        +=
                                                                        3 as
                                                                            libc::c_int;
                                                                    if (*fr).p.flags
                                                                           &
                                                                           MPG123_GAPLESS
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_long
                                                                           !=
                                                                           0 {
                                                                        frame_gapless_init(fr,
                                                                                           (*fr).track_frames,
                                                                                           pad_in,
                                                                                           pad_out);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => { }
                            }
                            // ABR rate
                            // 21 in
                            // Encoder delay and padding, two 12 bit values
		// ... lame does write them from int.
                            // 24 in
                            // switch buffer back ...
                            (*fr).bsbuf =
                                (*fr).bsspace[(*fr).bsnum as
                                                  usize].as_mut_ptr().offset(512
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize);
                            (*fr).bsnum =
                                (*fr).bsnum + 1 as libc::c_int &
                                    1 as libc::c_int;
                            return 1 as libc::c_int
                        }
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
// first attempt of read ahead check to find the real first header; cannot believe what junk is out there!
unsafe extern "C" fn do_readahead(mut fr: *mut mpg123_handle_t,
                                  mut newhead: ulong) -> libc::c_int {
    let mut nexthead: ulong = 0 as libc::c_int as ulong;
    let mut hd: libc::c_int = 0 as libc::c_int;
    let mut start: mpg_off_t = 0;
    let mut oret: mpg_off_t = 0;
    let mut ret: libc::c_int = 0;
    if !((*fr).firsthead == 0 &&
             (*fr).rdat.flags & (0x4 as libc::c_int | 0x8 as libc::c_int) !=
                 0) {
        return PARSE_GOOD as libc::c_int
    }
    start = (*(*fr).rd).tell.expect("non-null function pointer")(fr);
    // step framesize bytes forward and read next possible header
    oret =
        (*(*fr).rd).skip_bytes.expect("non-null function pointer")(fr,
                                                                   (*fr).framesize
                                                                       as
                                                                       mpg_off_t);
    if oret < 0 as libc::c_int as libc::c_long {
        return if oret == MPG123_NEED_MORE as libc::c_int as libc::c_long {
                   PARSE_MORE as libc::c_int
               } else { PARSE_ERR as libc::c_int }
    }
    // read header, seek back.
    hd =
        (*(*fr).rd).head_read.expect("non-null function pointer")(fr,
                                                                  &mut nexthead); // start over
    if (*(*fr).rd).back_bytes.expect("non-null function pointer")(fr,
                                                                  (*(*fr).rd).tell.expect("non-null function pointer")(fr)
                                                                      - start)
           < 0 as libc::c_int {
        return PARSE_ERR as libc::c_int
    }
    if hd == MPG123_NEED_MORE as libc::c_int {
        return PARSE_MORE as libc::c_int
    }
    if hd == 0 { return PARSE_END as libc::c_int }
    if head_check(nexthead) == 0 || head_compatible(newhead, nexthead) == 0 {
        (*fr).oldhead = 0 as libc::c_int as ulong;
        // try next byte for valid header
        ret =
            (*(*fr).rd).back_bytes.expect("non-null function pointer")(fr,
                                                                       3 as
                                                                           libc::c_int
                                                                           as
                                                                           mpg_off_t);
        if ret < 0 as libc::c_int { return PARSE_ERR as libc::c_int }
        return PARSE_AGAIN as libc::c_int
    }
    return PARSE_GOOD as libc::c_int;
}
unsafe extern "C" fn halfspeed_prepare(mut fr: *mut mpg123_handle_t) {
    if (*fr).p.halfspeed != 0 && (*fr).lay == 3 as libc::c_int {
        memcpy((*fr).ssave.as_mut_ptr() as *mut libc::c_void,
               (*fr).bsbuf as *const libc::c_void,
               (*fr).ssize as libc::c_ulong);
    };
}
// if this returns 1, the next frame is the repetition.
unsafe extern "C" fn halfspeed_do(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    // speed-down hack: Play it again, Sam (the frame, I mean).
    if (*fr).p.halfspeed != 0 {
        if (*fr).halfphase != 0 {
            // repeat last frame
            (*fr).to_ignore =
                (0 as libc::c_int == 0) as libc::c_int; // skip crc
            (*fr).to_decode = (*fr).to_ignore;
            (*fr).halfphase -= 1;
            (*fr).bitindex = 0 as libc::c_int;
            (*fr).wordpointer = (*fr).bsbuf;
            if (*fr).lay == 3 as libc::c_int {
                memcpy((*fr).bsbuf as *mut libc::c_void,
                       (*fr).ssave.as_mut_ptr() as *const libc::c_void,
                       (*fr).ssize as libc::c_ulong);
            }
            if (*fr).error_protection != 0 {
                (*fr).crc = getbits(fr, 16 as libc::c_int)
            }
            return 1 as libc::c_int
        } else {
            (*fr).halfphase =
                ((*fr).p.halfspeed - 1 as libc::c_int as libc::c_long) as
                    libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// read ahead and find the next MPEG header, to guess framesize
// return value: success code
// PARSE_GOOD: found a valid frame size (stored in the handle).
// < 0: error codes, possibly from feeder buffer (NEED_MORE)
// PARSE_BAD: cannot get the framesize for some reason and shall silentry try the next possible header (if this is no free format stream after all...)
unsafe extern "C" fn guess_freeformat_framesize(mut fr: *mut mpg123_handle_t,
                                                mut oldhead: ulong)
 -> libc::c_int {
    let mut head: ulong = 0;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_long = 0;
    if (*fr).rdat.flags & (0x4 as libc::c_int | 0x8 as libc::c_int) == 0 {
        return PARSE_BAD as libc::c_int
    }
    ret =
        (*(*fr).rd).head_read.expect("non-null function pointer")(fr,
                                                                  &mut head);
    if ret <= 0 as libc::c_int { return ret }
    // we are already 4 bytes into it
    i = 4 as libc::c_int as libc::c_long;
    while i < (3456 as libc::c_int + 4 as libc::c_int) as libc::c_long {
        ret =
            (*(*fr).rd).head_shift.expect("non-null function pointer")(fr,
                                                                       &mut head);
        if ret <= 0 as libc::c_int { return ret }
        // no head_check needed, the mask contains all relevant bits.
        if head &
               (0xffe00000 as libc::c_uint |
                    0x180000 as libc::c_int as libc::c_uint |
                    0x60000 as libc::c_int as libc::c_uint |
                    0xf000 as libc::c_int as libc::c_uint |
                    0xc00 as libc::c_int as libc::c_uint |
                    0xc0 as libc::c_int as libc::c_uint |
                    0x30 as libc::c_int as libc::c_uint) as libc::c_ulong ==
               oldhead &
                   (0xffe00000 as libc::c_uint |
                        0x180000 as libc::c_int as libc::c_uint |
                        0x60000 as libc::c_int as libc::c_uint |
                        0xf000 as libc::c_int as libc::c_uint |
                        0xc00 as libc::c_int as libc::c_uint |
                        0xc0 as libc::c_int as libc::c_uint |
                        0x30 as libc::c_int as libc::c_uint) as libc::c_ulong
           {
            (*(*fr).rd).back_bytes.expect("non-null function pointer")(fr,
                                                                       i +
                                                                           1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_long);
            (*fr).framesize =
                (i - 3 as libc::c_int as libc::c_long) as libc::c_int;
            return PARSE_GOOD as libc::c_int
            // success!
        }
        i += 1
    }
    (*(*fr).rd).back_bytes.expect("non-null function pointer")(fr, i);
    return PARSE_BAD as libc::c_int;
}
// decode a header and write the information
// into the frame structure
// return values are compatible with those of read_frame, namely:
//  1: success
//  0: no valid header
// <0: some error
// you are required to do a head_check() before calling!
unsafe extern "C" fn decode_header(mut fr: *mut mpg123_handle_t,
                                   mut newhead: ulong,
                                   mut freeformat_count: *mut libc::c_int)
 -> libc::c_int {
    // for some reason, the layer and sampling freq settings used to be wrapped
	// in a weird conditional including MPG123_NO_RESYNC. what was I thinking?
	// this information has to be consistent.
    (*fr).lay =
        (4 as libc::c_int as
             libc::c_ulong).wrapping_sub((newhead &
                                              0x60000 as libc::c_int as
                                                  libc::c_ulong) >>
                                             17 as libc::c_int) as
            libc::c_int;
    if (newhead & 0x180000 as libc::c_int as libc::c_ulong) >>
           19 as libc::c_int & 0x2 as libc::c_int as libc::c_ulong != 0 {
        (*fr).lsf =
            if (newhead & 0x180000 as libc::c_int as libc::c_ulong) >>
                   19 as libc::c_int & 0x1 as libc::c_int as libc::c_ulong !=
                   0 {
                0 as libc::c_int
            } else { 1 as libc::c_int };
        (*fr).sampling_frequency =
            ((newhead & 0xc00 as libc::c_int as libc::c_ulong) >>
                 10 as
                     libc::c_int).wrapping_add(((*fr).lsf * 3 as libc::c_int)
                                                   as libc::c_ulong) as
                libc::c_int;
        (*fr).mpeg25 = 0 as libc::c_int
    } else {
        (*fr).sampling_frequency =
            (6 as libc::c_int as
                 libc::c_ulong).wrapping_add((newhead &
                                                  0xc00 as libc::c_int as
                                                      libc::c_ulong) >>
                                                 10 as libc::c_int) as
                libc::c_int;
        (*fr).mpeg25 = 1 as libc::c_int;
        (*fr).lsf = 1 as libc::c_int
    }
    (*fr).error_protection =
        ((newhead & 0x10000 as libc::c_int as libc::c_ulong) >>
             16 as libc::c_int ^ 0x1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    (*fr).bitrate_index =
        ((newhead & 0xf000 as libc::c_int as libc::c_ulong) >>
             12 as libc::c_int) as libc::c_int;
    (*fr).padding =
        ((newhead & 0x200 as libc::c_int as libc::c_ulong) >>
             9 as libc::c_int) as libc::c_int;
    (*fr).extension =
        ((newhead & 0x100 as libc::c_int as libc::c_ulong) >>
             8 as libc::c_int) as libc::c_int;
    (*fr).mode =
        ((newhead & 0xc0 as libc::c_int as libc::c_ulong) >> 6 as libc::c_int)
            as libc::c_int;
    (*fr).mode_ext =
        ((newhead & 0x30 as libc::c_int as libc::c_ulong) >> 4 as libc::c_int)
            as libc::c_int;
    (*fr).copyright =
        ((newhead & 0x8 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int)
            as libc::c_int;
    (*fr).original =
        ((newhead & 0x4 as libc::c_int as libc::c_ulong) >> 2 as libc::c_int)
            as libc::c_int;
    (*fr).emphasis =
        ((newhead & 0x3 as libc::c_int as libc::c_ulong) >> 0 as libc::c_int)
            as libc::c_int;
    (*fr).freeformat =
        (newhead & 0xf000 as libc::c_int as libc::c_ulong == 0) as
            libc::c_int;
    (*fr).stereo =
        if (*fr).mode == 3 as libc::c_int {
            1 as libc::c_int
        } else { 2 as libc::c_int };
    // we can't use tabsel_123 for freeformat, so trying to guess framesize...
    if (*fr).freeformat != 0 {
        // when we first encounter the frame with freeformat, guess framesize
        if (*fr).freeformat_framesize < 0 as libc::c_int as libc::c_long {
            let mut ret: libc::c_int = 0;
            *freeformat_count += 1 as libc::c_int;
            if *freeformat_count > 5 as libc::c_int {
                return PARSE_BAD as libc::c_int
            }
            ret = guess_freeformat_framesize(fr, newhead);
            if ret == PARSE_GOOD as libc::c_int {
                (*fr).freeformat_framesize =
                    ((*fr).framesize - (*fr).padding) as libc::c_long
            } else { return ret }
        } else {
            // freeformat should be CBR, so the same framesize can be used at the 2nd reading or later
            (*fr).framesize =
                ((*fr).freeformat_framesize + (*fr).padding as libc::c_long)
                    as libc::c_int
        }
    } /* MPEG 2.5 implies LSF.*/
    match (*fr).lay {
        3 => {
            (*fr).spf =
                if (*fr).lsf != 0 {
                    576 as libc::c_int
                } else { 1152 as libc::c_int } as libc::c_long;
            (*fr).do_layer =
                Some(do_layer3 as
                         unsafe extern "C" fn(_: *mut mpg123_handle_t)
                             -> libc::c_int);
            if (*fr).lsf != 0 {
                (*fr).ssize =
                    if (*fr).stereo == 1 as libc::c_int {
                        9 as libc::c_int
                    } else { 17 as libc::c_int }
            } else {
                (*fr).ssize =
                    if (*fr).stereo == 1 as libc::c_int {
                        17 as libc::c_int
                    } else { 32 as libc::c_int }
            }
            if (*fr).error_protection != 0 { (*fr).ssize += 2 as libc::c_int }
            if (*fr).freeformat == 0 {
                (*fr).framesize =
                    (tabsel_123[(*fr).lsf as
                                    usize][2 as libc::c_int as
                                               usize][(*fr).bitrate_index as
                                                          usize] as
                         libc::c_long * 144000 as libc::c_int as libc::c_long)
                        as libc::c_int;
                (*fr).framesize =
                    ((*fr).framesize as libc::c_long /
                         (freqs[(*fr).sampling_frequency as usize] <<
                              (*fr).lsf)) as libc::c_int;
                (*fr).framesize =
                    (*fr).framesize + (*fr).padding - 4 as libc::c_int
            }
        }
        _ => { return PARSE_BAD as libc::c_int }
    }
    if (*fr).framesize > 3456 as libc::c_int {
        return PARSE_BAD as libc::c_int
    }
    return PARSE_GOOD as libc::c_int;
}
// advance a byte in stream to get next possible header and forget
// buffered data if possible (for feed reader).
unsafe extern "C" fn forget_head_shift(mut fr: *mut mpg123_handle_t,
                                       mut newheadp: *mut ulong,
                                       mut forget: libc::c_int)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret =
        (*(*fr).rd).head_shift.expect("non-null function pointer")(fr,
                                                                   newheadp);
    if ret <= 0 as libc::c_int { return ret }
    // try to forget buffered data as early as possible to speed up parsing where
	// new data needs to be added for resync (and things would be re-parsed again
	// and again because of the start from beginning after hitting end).
    if forget != 0 && (*(*fr).rd).forget.is_some() {
        // ensure that the last 4 bytes stay in buffers for reading the header anew.
        if (*(*fr).rd).back_bytes.expect("non-null function pointer")(fr,
                                                                      4 as
                                                                          libc::c_int
                                                                          as
                                                                          mpg_off_t)
               == 0 {
            (*(*fr).rd).forget.expect("non-null function pointer")(fr);
            (*(*fr).rd).back_bytes.expect("non-null function pointer")(fr,
                                                                       -(4 as
                                                                             libc::c_int)
                                                                           as
                                                                           mpg_off_t);
        }
    }
    return ret;
    // No surprise here, error already triggered early return.
}
// trying to parse ID3v2.3 and ID3v2.4 tags...
// returns:  0: bad or just unparseable tag
//           1: good, (possibly) new tag info
//          <0: reader error (may need more data feed, try again)
#[no_mangle]
pub unsafe extern "C" fn parse_new_id3(mut fr: *mut mpg123_handle_t,
                                       mut first4bytes: ulong)
 -> libc::c_int {
    let mut buf: [byte; 6] = [0; 6];
    let mut length: ulong = 0 as libc::c_int as ulong;
    let mut flags: byte = 0 as libc::c_int as byte;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut ret2: libc::c_int = 0;
    let mut major: byte =
        (first4bytes & 0xff as libc::c_int as libc::c_ulong) as byte;
    if major as libc::c_int == 0xff as libc::c_int { return 0 as libc::c_int }
    ret2 =
        (*(*fr).rd).read_frame_body.expect("non-null function pointer")(fr,
                                                                        buf.as_mut_ptr(),
                                                                        6 as
                                                                            libc::c_int);
    if ret2 < 0 as libc::c_int {
        // read more header information
        return ret2
    } // revision, will never be 0xff.
    if buf[0 as libc::c_int as usize] as libc::c_int == 0xff as libc::c_int {
        return 0 as libc::c_int
    }
    // second new byte are some nice flags, if these are invalid skip the whole thing
    flags = buf[1 as libc::c_int as usize];
    // length-10 or length-20 (footer present); 4 synchsafe integers == 28 bit number
	// we have already read 10 bytes, so left are length or length+10 bytes belonging to tag
    if if (*buf.as_mut_ptr().offset(2 as libc::c_int as
                                        isize).offset(0 as libc::c_int as
                                                          isize) as
               libc::c_int |
               *buf.as_mut_ptr().offset(2 as libc::c_int as
                                            isize).offset(1 as libc::c_int as
                                                              isize) as
                   libc::c_int |
               *buf.as_mut_ptr().offset(2 as libc::c_int as
                                            isize).offset(2 as libc::c_int as
                                                              isize) as
                   libc::c_int |
               *buf.as_mut_ptr().offset(2 as libc::c_int as
                                            isize).offset(3 as libc::c_int as
                                                              isize) as
                   libc::c_int) & 0x80 as libc::c_int != 0 {
           0 as libc::c_int
       } else {
           length =
               (*buf.as_mut_ptr().offset(2 as libc::c_int as
                                             isize).offset(0 as libc::c_int as
                                                               isize) as
                    ulong) << 21 as libc::c_int |
                   (*buf.as_mut_ptr().offset(2 as libc::c_int as
                                                 isize).offset(1 as
                                                                   libc::c_int
                                                                   as isize)
                        as ulong) << 14 as libc::c_int |
                   (*buf.as_mut_ptr().offset(2 as libc::c_int as
                                                 isize).offset(2 as
                                                                   libc::c_int
                                                                   as isize)
                        as ulong) << 7 as libc::c_int |
                   *buf.as_mut_ptr().offset(2 as libc::c_int as
                                                isize).offset(3 as libc::c_int
                                                                  as isize) as
                       ulong;
           1 as libc::c_int
       } == 0 {
        return 0 as libc::c_int
    }
    ret2 =
        (*(*fr).rd).skip_bytes.expect("non-null function pointer")(fr,
                                                                   length as
                                                                       mpg_off_t)
            as libc::c_int;
    if ret2 < 0 as libc::c_int {
        // will not store data in backbuff!
        ret = ret2
    }
    // skip footer if present
    if ret > 0 as libc::c_int && flags as libc::c_int & 16 as libc::c_int != 0
           &&
           {
               ret2 =
                   (*(*fr).rd).skip_bytes.expect("non-null function pointer")(fr,
                                                                              length
                                                                                  as
                                                                                  mpg_off_t)
                       as
                       libc::c_int; // think about that. Used to be present only for skipping of junk, not resync-style wetwork.
               (ret2) < 0 as libc::c_int
           } {
        ret = ret2
    }
    return ret;
}
unsafe extern "C" fn handle_id3v2(mut fr: *mut mpg123_handle_t,
                                  mut newhead: ulong) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    (*fr).oldhead = 0 as libc::c_int as ulong;
    ret = parse_new_id3(fr, newhead);
    if ret < 0 as libc::c_int { return ret }
    return PARSE_AGAIN as libc::c_int;
}
// watch out for junk/tags on beginning of stream by invalid header
unsafe extern "C" fn skip_junk(mut fr: *mut mpg123_handle_t,
                               mut newheadp: *mut ulong,
                               mut headcount: *mut libc::c_long)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut freeformat_count: libc::c_int = 0 as libc::c_int;
    let mut newhead: ulong = *newheadp;
    let mut forgetcount: uint = 0 as libc::c_int as uint;
    let mut limit: libc::c_long = 65536 as libc::c_int as libc::c_long;
    // check for id3v2; first three bytes (of 4) are "ID3"
    if newhead & 0xffffff00 as libc::c_uint as ulong ==
           0x49443300 as libc::c_int as ulong {
        return handle_id3v2(fr, newhead)
    }
    // I even saw RIFF headers at the beginning of MPEG streams ;(
    if newhead ==
           ((('R' as i32) << 24 as libc::c_int) +
                (('I' as i32) << 16 as libc::c_int) +
                (('F' as i32) << 8 as libc::c_int) + 'F' as i32) as
               libc::c_ulong {
        ret =
            (*(*fr).rd).head_read.expect("non-null function pointer")(fr,
                                                                      &mut newhead);
        if ret <= 0 as libc::c_int { return ret }
        while newhead !=
                  ((('d' as i32) << 24 as libc::c_int) +
                       (('a' as i32) << 16 as libc::c_int) +
                       (('t' as i32) << 8 as libc::c_int) + 'a' as i32) as
                      libc::c_ulong {
            forgetcount = forgetcount.wrapping_add(1);
            if forgetcount > 1024 as libc::c_int as libc::c_uint {
                forgetcount = 0 as libc::c_int as uint
            }
            ret =
                forget_head_shift(fr, &mut newhead,
                                  (forgetcount == 0) as libc::c_int);
            if ret <= 0 as libc::c_int { return ret }
        }
        ret =
            (*(*fr).rd).head_read.expect("non-null function pointer")(fr,
                                                                      &mut newhead);
        if ret <= 0 as libc::c_int { return ret }
        (*fr).oldhead = 0 as libc::c_int as ulong;
        *newheadp = newhead;
        return PARSE_AGAIN as libc::c_int
    }
    // unhandled junk... just continue search for a header, stepping in single bytes through next 64K.
	// this is rather identical to the resync loop.
    *newheadp = 0 as libc::c_int as ulong; // invalidate the external value.
    ret = 0 as libc::c_int; // we will check the value after the loop.
    // we prepare for at least the 64K bytes as usual, unless
	// user explicitly wanted more (even infinity). Never less.
    if (*fr).p.resync_limit < 0 as libc::c_int as libc::c_long ||
           (*fr).p.resync_limit > limit {
        limit = (*fr).p.resync_limit
    }
    loop  {
        *headcount += 1;
        if limit >= 0 as libc::c_int as libc::c_long && *headcount >= limit {
            break ;
        }
        forgetcount = forgetcount.wrapping_add(1);
        if forgetcount > 1024 as libc::c_int as libc::c_uint {
            forgetcount = 0 as libc::c_int as uint
        }
        ret =
            forget_head_shift(fr, &mut newhead,
                              (forgetcount == 0) as libc::c_int);
        if ret <= 0 as libc::c_int { return ret }
        if head_check(newhead) != 0 &&
               {
                   ret = decode_header(fr, newhead, &mut freeformat_count);
                   (ret) != 0
               } {
            break ;
        }
    }
    if ret < 0 as libc::c_int { return ret }
    if limit >= 0 as libc::c_int as libc::c_long && *headcount >= limit {
        return PARSE_END as libc::c_int
    }
    // If the new header ist good, it is already decoded.
    *newheadp = newhead;
    return PARSE_GOOD as libc::c_int;
}
// the newhead is bad, so let's check if it is something special, otherwise just resync.
unsafe extern "C" fn wetwork(mut fr: *mut mpg123_handle_t,
                             mut newheadp: *mut ulong) -> libc::c_int {
    let mut ret: libc::c_int = PARSE_ERR as libc::c_int;
    let mut newhead: ulong = *newheadp;
    *newheadp = 0 as libc::c_int as ulong;
    // classic ID3 tags. Read, then start parsing again.
    if newhead & 0xffffff00 as libc::c_uint as libc::c_ulong ==
           ((('T' as i32) << 24 as libc::c_int) +
                (('A' as i32) << 16 as libc::c_int) +
                (('G' as i32) << 8 as libc::c_int)) as libc::c_ulong {
        (*fr).id3buf[0 as libc::c_int as usize] =
            (newhead >> 24 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as
                byte; // that marks id3v1
        (*fr).id3buf[1 as libc::c_int as usize] =
            (newhead >> 16 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as byte;
        (*fr).id3buf[2 as libc::c_int as usize] =
            (newhead >> 8 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as byte;
        (*fr).id3buf[3 as libc::c_int as usize] =
            (newhead & 0xff as libc::c_int as libc::c_ulong) as byte;
        ret =
            (*(*fr).rd).fullread.expect("non-null function pointer")(fr,
                                                                     (*fr).id3buf.as_mut_ptr().offset(4
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize),
                                                                     124 as
                                                                         libc::c_int
                                                                         as
                                                                         mpg_ssize_t)
                as libc::c_int;
        if ret < 0 as libc::c_int { return ret }
        (*fr).metaflags |= 0x1 as libc::c_int | 0x3 as libc::c_int;
        (*fr).rdat.flags |= 0x2 as libc::c_int;
        return PARSE_AGAIN as libc::c_int
    }
    // this is similar to initial junk skipping code...
	// check for id3v2; first three bytes (of 4) are "ID3"
    if newhead & 0xffffff00 as libc::c_uint as ulong ==
           0x49443300 as libc::c_int as ulong {
        return handle_id3v2(fr, newhead)
    }
    // now we got something bad at hand, try to recover.
    if (*fr).p.flags & MPG123_NO_RESYNC as libc::c_int as libc::c_long == 0 {
        let mut try_0: libc::c_long = 0 as libc::c_int as libc::c_long;
        let mut limit: libc::c_long = (*fr).p.resync_limit;
        let mut forgetcount: uint = 0 as libc::c_int as uint;
        // if a resync is needed the bitreservoir of previous frames is no longer valid
        (*fr).bitreservoir = 0 as libc::c_int as uint;
        loop 
             // ... shift the header with additional single bytes until be found something that could be a header.
             {
            try_0 += 1;
            if limit >= 0 as libc::c_int as libc::c_long && try_0 >= limit {
                break ;
            }
            forgetcount = forgetcount.wrapping_add(1);
            if forgetcount > 1024 as libc::c_int as libc::c_uint {
                forgetcount = 0 as libc::c_int as uint
            }
            ret =
                forget_head_shift(fr, &mut newhead,
                                  (forgetcount == 0) as libc::c_int);
            if ret <= 0 as libc::c_int {
                *newheadp = newhead;
                return if ret != 0 { ret } else { PARSE_END as libc::c_int }
            }
            if !(head_check(newhead) == 0) { break ; }
        }
        *newheadp = newhead;
        // now we either got something that could be a header, or we gave up.
        if limit >= 0 as libc::c_int as libc::c_long && try_0 >= limit {
            (*fr).err = MPG123_RESYNC_FAIL as libc::c_int;
            return PARSE_ERR as libc::c_int
        } else {
            (*fr).oldhead = 0 as libc::c_int as ulong;
            return PARSE_RESYNC as libc::c_int
        }
    } else {
        (*fr).err = MPG123_OUT_OF_SYNC as libc::c_int;
        return PARSE_ERR as libc::c_int
    };
}
// that's a big one: read the next frame. 1 is success, <= 0 is some error
// special error READER_MORE means: Please feed more data and try again.
#[no_mangle]
pub unsafe extern "C" fn read_frame(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut current_block: u64;
    // TODO: rework this thing
    let mut freeformat_count: libc::c_int = 0 as libc::c_int; // for Layer3
    let mut oldsize: libc::c_int = (*fr).framesize;
    let mut oldphase: libc::c_int = (*fr).halfphase;
    let mut headcount: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut newbuf: *mut byte = 0 as *mut byte;
    let mut newhead: ulong = 0;
    let mut framepos: mpg_off_t = 0;
    let mut ret: libc::c_int = 0;
    (*fr).fsizeold = (*fr).framesize;
    if halfspeed_do(fr) == 1 as libc::c_int { return 1 as libc::c_int }
    'c_3350:
        loop  {
            // in case we are looping to find a valid frame, discard any buffered data before the current position.
	// this is essential to prevent endless looping, always going back to the beginning when feeder buffer is exhausted.
            if (*(*fr).rd).forget.is_some() {
                (*(*fr).rd).forget.expect("non-null function pointer")(fr);
            }
            ret =
                (*(*fr).rd).head_read.expect("non-null function pointer")(fr,
                                                                          &mut newhead);
            if ret <= 0 as libc::c_int {
                current_block = 2206747307331469876;
                break ;
            }
            loop  {
                if (*fr).firsthead == 0 && head_check(newhead) == 0 {
                    ret = skip_junk(fr, &mut newhead, &mut headcount);
                    if ret < 0 as libc::c_int {
                        current_block = 2206747307331469876;
                        break 'c_3350 ;
                    }
                    if ret == PARSE_AGAIN as libc::c_int {
                        continue 'c_3350 ;
                    }
                    if ret == PARSE_RESYNC as libc::c_int { continue ; }
                    if ret == PARSE_END as libc::c_int {
                        ret = 0 as libc::c_int;
                        current_block = 2206747307331469876;
                        break 'c_3350 ;
                    }
                }
                ret = head_check(newhead);
                if ret != 0 {
                    ret = decode_header(fr, newhead, &mut freeformat_count)
                }
                if ret < 0 as libc::c_int {
                    current_block = 2206747307331469876;
                    break 'c_3350 ;
                }
                if ret == PARSE_AGAIN as libc::c_int { continue 'c_3350 ; }
                if ret == PARSE_RESYNC as libc::c_int { continue ; }
                if ret == PARSE_END as libc::c_int {
                    ret = 0 as libc::c_int;
                    current_block = 2206747307331469876;
                    break 'c_3350 ;
                } else {
                    if ret == PARSE_BAD as libc::c_int {
                        // header was not good.
                        ret =
                            wetwork(fr,
                                    &mut newhead); // Messy stuff, handle junk, resync ...
                        if ret < 0 as libc::c_int {
                            current_block = 2206747307331469876;
                            break 'c_3350 ;
                        }
                        if ret == PARSE_AGAIN as libc::c_int {
                            continue 'c_3350 ;
                        }
                        if ret == PARSE_RESYNC as libc::c_int { continue ; }
                        if ret == PARSE_END as libc::c_int {
                            ret = 0 as libc::c_int;
                            current_block = 2206747307331469876;
                            break 'c_3350 ;
                        } else if ret != PARSE_GOOD as libc::c_int
                         // normally, we jumped already.
		// if for some reason everything's fine to continue, do continue.
                         {
                            current_block = 2206747307331469876;
                            break 'c_3350 ;
                        }
                    }
                    if !((*fr).firsthead == 0) {
                        current_block = 2989495919056355252;
                        break ;
                    }
                    ret = do_readahead(fr, newhead);
                    // readahead can fail mit NEED_MORE, in which case we must also make
		// the just read header available again for next go
                    if ret < 0 as libc::c_int {
                        (*(*fr).rd).back_bytes.expect("non-null function pointer")(fr,
                                                                                   4
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       mpg_off_t);
                    }
                    if ret < 0 as libc::c_int {
                        current_block = 2206747307331469876;
                        break 'c_3350 ;
                    }
                    if ret == PARSE_AGAIN as libc::c_int {
                        continue 'c_3350 ;
                    }
                    if ret == PARSE_RESYNC as libc::c_int { continue ; }
                    if ret == PARSE_END as libc::c_int {
                        current_block = 1622411330066726685;
                        break ;
                    } else { current_block = 2989495919056355252; break ; }
                }
            }
            match current_block {
                1622411330066726685 => {
                    ret = 0 as libc::c_int;
                    current_block = 2206747307331469876;
                    break ;
                }
                _ => {
                    // now we should have our valid header and proceed to reading the frame.
                    // if filepos is invalid, so is framepos
                    framepos =
                        (*(*fr).rd).tell.expect("non-null function pointer")(fr)
                            - 4 as libc::c_int as libc::c_long;
                    // flip/init buffer for Layer 3
                    newbuf =
                        (*fr).bsspace[(*fr).bsnum as
                                          usize].as_mut_ptr().offset(512 as
                                                                         libc::c_int
                                                                         as
                                                                         isize);
                    // read main data into memory
                    ret =
                        (*(*fr).rd).read_frame_body.expect("non-null function pointer")(fr,
                                                                                        newbuf,
                                                                                        (*fr).framesize); // _now_ it's time to store it... the first real header */
                    if ret < 0 as libc::c_int {
                        current_block = 2206747307331469876;
                        break ;
                    }
                    (*fr).bsbufold = (*fr).bsbuf;
                    (*fr).bsbuf = newbuf;
                    (*fr).bsnum =
                        (*fr).bsnum + 1 as libc::c_int & 1 as libc::c_int;
                    if !((*fr).firsthead == 0) {
                        current_block = 13125627826496529465;
                        break ;
                    }
                    (*fr).firsthead = newhead;
                    // this is the first header of our current stream segment.
		// it is only the actual first header of the whole stream when fr->num is still below zero!
		// think of resyncs where firsthead has been reset for format flexibility.
                    if !((*fr).num < 0 as libc::c_int as libc::c_long) {
                        current_block = 13125627826496529465;
                        break ;
                    }
                    (*fr).audio_start = framepos;
                    // only check for LAME tag at beginning of whole stream
			// ... when there indeed is one in between, it's the user's problem.
                    if (*fr).lay == 3 as libc::c_int &&
                           check_lame_tag(fr) == 1 as libc::c_int {
                        // ...in practice, Xing/LAME tags are layer 3 only.
                        if (*(*fr).rd).forget.is_some() {
                            (*(*fr).rd).forget.expect("non-null function pointer")(fr);
                        }
                        (*fr).oldhead = 0 as libc::c_int as ulong
                    } else {
                        // now adjust volume
                        do_rva(fr);
                        current_block = 13125627826496529465;
                        break ;
                    }
                }
            }
        }
    match current_block {
        2206747307331469876 =>
        // if failed: flip back
        // also if we searched for valid data in vein, we can forget skipped data.
	// otherwise, the feeder would hold every dead old byte in memory until the first valid frame!
        {
            if (*(*fr).rd).forget.is_some() {
                (*(*fr).rd).forget.expect("non-null function pointer")(fr);
            }
            (*fr).silent_resync = 0 as libc::c_int as libc::c_char;
            if (*fr).err == MPG123_OK as libc::c_int {
                (*fr).err = MPG123_ERR_READER as libc::c_int
            }
            (*fr).framesize = oldsize;
            (*fr).halfphase = oldphase;
            // that return code might be inherited from some feeder action, or reader error.
            return ret
        }
        _ => {
            (*fr).bitindex = 0 as libc::c_int;
            (*fr).wordpointer = (*fr).bsbuf;
            // question: How bad does the floating point value get with repeated recomputation?
	// also, considering that we can play the file or parts of many times.
            (*fr).mean_frames += 1; // 0 for first frame!
            if (*fr).mean_frames != 0 as libc::c_int as libc::c_long {
                (*fr).mean_framesize =
                    (((*fr).mean_frames - 1 as libc::c_int as libc::c_long) as
                         libc::c_double * (*fr).mean_framesize +
                         compute_bpf(fr)) /
                        (*fr).mean_frames as libc::c_double
            }
            (*fr).num += 1;
            if (*fr).state_flags & FRAME_FRANKENSTEIN as libc::c_int == 0 &&
                   ((*fr).track_frames > 0 as libc::c_int as libc::c_long &&
                        (*fr).num >= (*fr).track_frames ||
                        (*fr).gapless_frames >
                            0 as libc::c_int as libc::c_long &&
                            (*fr).num >= (*fr).gapless_frames) {
                (*fr).state_flags |= FRAME_FRANKENSTEIN as libc::c_int
            }
            halfspeed_prepare(fr);
            // index the position
            (*fr).input_offset = framepos;
            // keep track of true frame positions in our frame index.
	// but only do so when we are sure that the frame number is accurate...
            if (*fr).state_flags & FRAME_ACCURATE as libc::c_int != 0 &&
                   ((*fr).index.size != 0 && (*fr).num == (*fr).index.next) {
                fi_add(&mut (*fr).index, framepos); // skip crc
            }
            if (*fr).silent_resync as libc::c_int > 0 as libc::c_int {
                (*fr).silent_resync -= 1
            }
            if (*(*fr).rd).forget.is_some() {
                (*(*fr).rd).forget.expect("non-null function pointer")(fr);
            }
            (*fr).to_ignore = (0 as libc::c_int == 0) as libc::c_int;
            (*fr).to_decode = (*fr).to_ignore;
            if (*fr).error_protection != 0 {
                (*fr).crc = getbits(fr, 16 as libc::c_int)
            }
            // let's check for header change after deciding that the new one is good
	// and actually having read a frame.
	// header_change > 1: decoder structure has to be updated
	// preserve header_change value from previous runs if it is serious.
	// If we still have a big change pending, it should be dealt with outside,
	// fr->header_change set to zero afterwards.
            if (*fr).header_change < 2 as libc::c_int {
                (*fr).header_change =
                    2 as libc::c_int; // output format change is possible...
                if (*fr).oldhead != 0 {
                    // check a following header for change
                    if (*fr).oldhead == newhead {
                        (*fr).header_change = 0 as libc::c_int
                    } else if head_compatible((*fr).oldhead, newhead) != 0 {
                        // headers that match in this test behave the same for the outside world.
				// namely: same decoding routines, same amount of decoded data.
                        (*fr).header_change = 1 as libc::c_int
                    } else {
                        (*fr).state_flags |= FRAME_FRANKENSTEIN as libc::c_int
                    }
                } else if (*fr).firsthead != 0 &&
                              head_compatible((*fr).firsthead, newhead) == 0 {
                    (*fr).state_flags |= FRAME_FRANKENSTEIN as libc::c_int
                }
            }
            (*fr).oldhead = newhead;
            return 1 as libc::c_int
        }
    };
}
