#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn set_pointer(fr: *mut mpg123_handle_t, backstep: libc::c_long);
    #[no_mangle]
    fn dct12(in_0: *mut libc::c_float, rawout1: *mut libc::c_float,
             rawout2: *mut libc::c_float, wi: *mut libc::c_float,
             ts: *mut libc::c_float);
    #[no_mangle]
    fn dct36(inbuf: *mut libc::c_float, o1: *mut libc::c_float,
             o2: *mut libc::c_float, wintab: *mut libc::c_float,
             tsbuf: *mut libc::c_float);
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn tan(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type word = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bandInfoStruct {
    pub longIdx: [word; 23],
    pub longDiff: [byte; 22],
    pub shortIdx: [word; 14],
    pub shortDiff: [byte; 13],
}
// MPEG 2.0 slen for intensity stereo
// Decoder state data, living on the stack of do_layer3.
pub type gr_info_t = gr_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gr_info_s {
    pub scfsi: libc::c_int,
    pub part2_3_length: uint,
    pub big_values: uint,
    pub scalefac_compress: uint,
    pub block_type: uint,
    pub mixed_block_flag: uint,
    pub table_select: [uint; 3],
    pub maxband: [libc::c_int; 3],
    pub maxbandl: libc::c_int,
    pub maxb: uint,
    pub region1start: uint,
    pub region2start: uint,
    pub preflag: uint,
    pub scalefac_scale: uint,
    pub count1table_select: uint,
    pub full_gain: [*mut libc::c_float; 3],
    pub pow2gain: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub gr: [gr_info_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct III_sideinfo {
    pub main_data_begin: uint,
    pub private_bits: uint,
    pub ch: [C2RustUnnamed_1; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct newhuff {
    pub linbits: uint,
    pub table: *const libc::c_short,
}
static mut tab0: [libc::c_short; 1] = [0 as libc::c_int as libc::c_short];
static mut tab1: [libc::c_short; 7] =
    [-(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 17 as libc::c_int as libc::c_short,
     1 as libc::c_int as libc::c_short, 16 as libc::c_int as libc::c_short,
     0 as libc::c_int as libc::c_short];
static mut tab2: [libc::c_short; 17] =
    [-(15 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 34 as libc::c_int as libc::c_short,
     2 as libc::c_int as libc::c_short, 18 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 33 as libc::c_int as libc::c_short,
     32 as libc::c_int as libc::c_short, 17 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 1 as libc::c_int as libc::c_short,
     16 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short];
static mut tab3: [libc::c_short; 17] =
    [-(13 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 34 as libc::c_int as libc::c_short,
     2 as libc::c_int as libc::c_short, 18 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 33 as libc::c_int as libc::c_short,
     32 as libc::c_int as libc::c_short, 16 as libc::c_int as libc::c_short,
     17 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     1 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short];
static mut tab5: [libc::c_short; 31] =
    [-(29 as libc::c_int) as libc::c_short,
     -(25 as libc::c_int) as libc::c_short,
     -(23 as libc::c_int) as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 51 as libc::c_int as libc::c_short,
     35 as libc::c_int as libc::c_short, 50 as libc::c_int as libc::c_short,
     49 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 19 as libc::c_int as libc::c_short,
     3 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     48 as libc::c_int as libc::c_short, 34 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 18 as libc::c_int as libc::c_short,
     33 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     2 as libc::c_int as libc::c_short, 32 as libc::c_int as libc::c_short,
     17 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     1 as libc::c_int as libc::c_short, 16 as libc::c_int as libc::c_short,
     0 as libc::c_int as libc::c_short];
static mut tab6: [libc::c_short; 31] =
    [-(25 as libc::c_int) as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 51 as libc::c_int as libc::c_short,
     3 as libc::c_int as libc::c_short, 35 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 50 as libc::c_int as libc::c_short,
     48 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     19 as libc::c_int as libc::c_short, 49 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 34 as libc::c_int as libc::c_short,
     2 as libc::c_int as libc::c_short, 18 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 33 as libc::c_int as libc::c_short,
     32 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 17 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 16 as libc::c_int as libc::c_short,
     0 as libc::c_int as libc::c_short];
static mut tab7: [libc::c_short; 71] =
    [-(69 as libc::c_int) as libc::c_short,
     -(65 as libc::c_int) as libc::c_short,
     -(57 as libc::c_int) as libc::c_short,
     -(39 as libc::c_int) as libc::c_short,
     -(29 as libc::c_int) as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 85 as libc::c_int as libc::c_short,
     69 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     84 as libc::c_int as libc::c_short, 83 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 53 as libc::c_int as libc::c_short,
     68 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 37 as libc::c_int as libc::c_short,
     82 as libc::c_int as libc::c_short, 21 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 81 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 5 as libc::c_int as libc::c_short,
     52 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     80 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     67 as libc::c_int as libc::c_short, 51 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 36 as libc::c_int as libc::c_short,
     66 as libc::c_int as libc::c_short, 20 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 65 as libc::c_int as libc::c_short,
     64 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 4 as libc::c_int as libc::c_short,
     35 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     50 as libc::c_int as libc::c_short, 3 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 19 as libc::c_int as libc::c_short,
     49 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 48 as libc::c_int as libc::c_short,
     34 as libc::c_int as libc::c_short, 18 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 33 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 2 as libc::c_int as libc::c_short,
     32 as libc::c_int as libc::c_short, 17 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 1 as libc::c_int as libc::c_short,
     16 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short];
static mut tab8: [libc::c_short; 71] =
    [-(65 as libc::c_int) as libc::c_short,
     -(63 as libc::c_int) as libc::c_short,
     -(59 as libc::c_int) as libc::c_short,
     -(45 as libc::c_int) as libc::c_short,
     -(31 as libc::c_int) as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 85 as libc::c_int as libc::c_short,
     84 as libc::c_int as libc::c_short, 69 as libc::c_int as libc::c_short,
     83 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 53 as libc::c_int as libc::c_short,
     68 as libc::c_int as libc::c_short, 37 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 82 as libc::c_int as libc::c_short,
     5 as libc::c_int as libc::c_short, 21 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 81 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 52 as libc::c_int as libc::c_short,
     67 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 80 as libc::c_int as libc::c_short,
     51 as libc::c_int as libc::c_short, 36 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 66 as libc::c_int as libc::c_short,
     20 as libc::c_int as libc::c_short, 65 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 4 as libc::c_int as libc::c_short,
     64 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     35 as libc::c_int as libc::c_short, 50 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 19 as libc::c_int as libc::c_short,
     49 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     3 as libc::c_int as libc::c_short, 48 as libc::c_int as libc::c_short,
     34 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     2 as libc::c_int as libc::c_short, 32 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 18 as libc::c_int as libc::c_short,
     33 as libc::c_int as libc::c_short, 17 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 1 as libc::c_int as libc::c_short,
     16 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short];
static mut tab9: [libc::c_short; 71] =
    [-(63 as libc::c_int) as libc::c_short,
     -(53 as libc::c_int) as libc::c_short,
     -(41 as libc::c_int) as libc::c_short,
     -(29 as libc::c_int) as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 85 as libc::c_int as libc::c_short,
     69 as libc::c_int as libc::c_short, 53 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 83 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 84 as libc::c_int as libc::c_short,
     5 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 68 as libc::c_int as libc::c_short,
     37 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     82 as libc::c_int as libc::c_short, 21 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 81 as libc::c_int as libc::c_short,
     52 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     67 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     80 as libc::c_int as libc::c_short, 4 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 36 as libc::c_int as libc::c_short,
     66 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     51 as libc::c_int as libc::c_short, 64 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 20 as libc::c_int as libc::c_short,
     65 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 35 as libc::c_int as libc::c_short,
     50 as libc::c_int as libc::c_short, 19 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 49 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 3 as libc::c_int as libc::c_short,
     48 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 34 as libc::c_int as libc::c_short,
     2 as libc::c_int as libc::c_short, 18 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 33 as libc::c_int as libc::c_short,
     32 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 17 as libc::c_int as libc::c_short,
     1 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     16 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short];
static mut tab10: [libc::c_short; 127] =
    [-(125 as libc::c_int) as libc::c_short,
     -(121 as libc::c_int) as libc::c_short,
     -(111 as libc::c_int) as libc::c_short,
     -(83 as libc::c_int) as libc::c_short,
     -(55 as libc::c_int) as libc::c_short,
     -(35 as libc::c_int) as libc::c_short,
     -(21 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     119 as libc::c_int as libc::c_short, 103 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     118 as libc::c_int as libc::c_short, 87 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     117 as libc::c_int as libc::c_short, 102 as libc::c_int as libc::c_short,
     71 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     116 as libc::c_int as libc::c_short, 86 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     101 as libc::c_int as libc::c_short, 55 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     115 as libc::c_int as libc::c_short, 70 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 85 as libc::c_int as libc::c_short,
     84 as libc::c_int as libc::c_short, 99 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 39 as libc::c_int as libc::c_short,
     114 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     100 as libc::c_int as libc::c_short, 7 as libc::c_int as libc::c_short,
     112 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 98 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 69 as libc::c_int as libc::c_short,
     53 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 6 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 83 as libc::c_int as libc::c_short,
     68 as libc::c_int as libc::c_short, 23 as libc::c_int as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     113 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 54 as libc::c_int as libc::c_short,
     38 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 37 as libc::c_int as libc::c_short,
     82 as libc::c_int as libc::c_short, 21 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 81 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 52 as libc::c_int as libc::c_short,
     67 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 22 as libc::c_int as libc::c_short,
     97 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     96 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     5 as libc::c_int as libc::c_short, 80 as libc::c_int as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 36 as libc::c_int as libc::c_short,
     66 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     51 as libc::c_int as libc::c_short, 4 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 20 as libc::c_int as libc::c_short,
     65 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 64 as libc::c_int as libc::c_short,
     35 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     50 as libc::c_int as libc::c_short, 3 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 19 as libc::c_int as libc::c_short,
     49 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     48 as libc::c_int as libc::c_short, 34 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 18 as libc::c_int as libc::c_short,
     33 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     2 as libc::c_int as libc::c_short, 32 as libc::c_int as libc::c_short,
     17 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     1 as libc::c_int as libc::c_short, 16 as libc::c_int as libc::c_short,
     0 as libc::c_int as libc::c_short];
static mut tab11: [libc::c_short; 127] =
    [-(121 as libc::c_int) as libc::c_short,
     -(113 as libc::c_int) as libc::c_short,
     -(89 as libc::c_int) as libc::c_short,
     -(59 as libc::c_int) as libc::c_short,
     -(43 as libc::c_int) as libc::c_short,
     -(27 as libc::c_int) as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     119 as libc::c_int as libc::c_short, 103 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     118 as libc::c_int as libc::c_short, 117 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     102 as libc::c_int as libc::c_short, 71 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     116 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 87 as libc::c_int as libc::c_short,
     85 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 86 as libc::c_int as libc::c_short,
     101 as libc::c_int as libc::c_short, 55 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     115 as libc::c_int as libc::c_short, 70 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 69 as libc::c_int as libc::c_short,
     84 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     53 as libc::c_int as libc::c_short, 83 as libc::c_int as libc::c_short,
     39 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     114 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     100 as libc::c_int as libc::c_short, 7 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     113 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 23 as libc::c_int as libc::c_short,
     112 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 54 as libc::c_int as libc::c_short,
     99 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     96 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     68 as libc::c_int as libc::c_short, 37 as libc::c_int as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 82 as libc::c_int as libc::c_short,
     5 as libc::c_int as libc::c_short, 21 as libc::c_int as libc::c_short,
     98 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 38 as libc::c_int as libc::c_short,
     6 as libc::c_int as libc::c_short, 22 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 97 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 81 as libc::c_int as libc::c_short,
     52 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 80 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 67 as libc::c_int as libc::c_short,
     51 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     36 as libc::c_int as libc::c_short, 66 as libc::c_int as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 20 as libc::c_int as libc::c_short,
     65 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     4 as libc::c_int as libc::c_short, 64 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 35 as libc::c_int as libc::c_short,
     50 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     19 as libc::c_int as libc::c_short, 49 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 3 as libc::c_int as libc::c_short,
     48 as libc::c_int as libc::c_short, 34 as libc::c_int as libc::c_short,
     33 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 18 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 2 as libc::c_int as libc::c_short,
     32 as libc::c_int as libc::c_short, 17 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 1 as libc::c_int as libc::c_short,
     16 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short];
static mut tab12: [libc::c_short; 127] =
    [-(115 as libc::c_int) as libc::c_short,
     -(99 as libc::c_int) as libc::c_short,
     -(73 as libc::c_int) as libc::c_short,
     -(45 as libc::c_int) as libc::c_short,
     -(27 as libc::c_int) as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     119 as libc::c_int as libc::c_short, 103 as libc::c_int as libc::c_short,
     118 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 87 as libc::c_int as libc::c_short,
     117 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     102 as libc::c_int as libc::c_short, 71 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     116 as libc::c_int as libc::c_short, 101 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 86 as libc::c_int as libc::c_short,
     55 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     115 as libc::c_int as libc::c_short, 85 as libc::c_int as libc::c_short,
     39 as libc::c_int as libc::c_short, -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     114 as libc::c_int as libc::c_short, 70 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     100 as libc::c_int as libc::c_short, 23 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     113 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 7 as libc::c_int as libc::c_short,
     112 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 54 as libc::c_int as libc::c_short,
     99 as libc::c_int as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 69 as libc::c_int as libc::c_short,
     84 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     68 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     6 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 38 as libc::c_int as libc::c_short,
     98 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 97 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 22 as libc::c_int as libc::c_short,
     96 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 53 as libc::c_int as libc::c_short,
     83 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     37 as libc::c_int as libc::c_short, 82 as libc::c_int as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 21 as libc::c_int as libc::c_short,
     81 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     52 as libc::c_int as libc::c_short, 67 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 80 as libc::c_int as libc::c_short,
     4 as libc::c_int as libc::c_short, 36 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 66 as libc::c_int as libc::c_short,
     20 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 51 as libc::c_int as libc::c_short,
     65 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     35 as libc::c_int as libc::c_short, 50 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 64 as libc::c_int as libc::c_short,
     3 as libc::c_int as libc::c_short, 48 as libc::c_int as libc::c_short,
     19 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     49 as libc::c_int as libc::c_short, 34 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 18 as libc::c_int as libc::c_short,
     33 as libc::c_int as libc::c_short, -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 2 as libc::c_int as libc::c_short,
     32 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short,
     17 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     1 as libc::c_int as libc::c_short, 16 as libc::c_int as libc::c_short];
static mut tab13: [libc::c_short; 511] =
    [-(509 as libc::c_int) as libc::c_short,
     -(503 as libc::c_int) as libc::c_short,
     -(475 as libc::c_int) as libc::c_short,
     -(405 as libc::c_int) as libc::c_short,
     -(333 as libc::c_int) as libc::c_short,
     -(265 as libc::c_int) as libc::c_short,
     -(205 as libc::c_int) as libc::c_short,
     -(153 as libc::c_int) as libc::c_short,
     -(115 as libc::c_int) as libc::c_short,
     -(83 as libc::c_int) as libc::c_short,
     -(53 as libc::c_int) as libc::c_short,
     -(35 as libc::c_int) as libc::c_short,
     -(21 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     254 as libc::c_int as libc::c_short, 252 as libc::c_int as libc::c_short,
     253 as libc::c_int as libc::c_short, 237 as libc::c_int as libc::c_short,
     255 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     239 as libc::c_int as libc::c_short, 223 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     238 as libc::c_int as libc::c_short, 207 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     222 as libc::c_int as libc::c_short, 191 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     251 as libc::c_int as libc::c_short, 206 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     220 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     175 as libc::c_int as libc::c_short, 233 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     236 as libc::c_int as libc::c_short, 221 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     250 as libc::c_int as libc::c_short, 205 as libc::c_int as libc::c_short,
     190 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     235 as libc::c_int as libc::c_short, 159 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     249 as libc::c_int as libc::c_short, 234 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     189 as libc::c_int as libc::c_short, 219 as libc::c_int as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     143 as libc::c_int as libc::c_short, 248 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     204 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     174 as libc::c_int as libc::c_short, 158 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     142 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     127 as libc::c_int as libc::c_short, 126 as libc::c_int as libc::c_short,
     247 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     218 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     173 as libc::c_int as libc::c_short, 188 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     203 as libc::c_int as libc::c_short, 246 as libc::c_int as libc::c_short,
     111 as libc::c_int as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     232 as libc::c_int as libc::c_short, 95 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     157 as libc::c_int as libc::c_short, 217 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     245 as libc::c_int as libc::c_short, 231 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     172 as libc::c_int as libc::c_short, 187 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 79 as libc::c_int as libc::c_short,
     244 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     202 as libc::c_int as libc::c_short, 230 as libc::c_int as libc::c_short,
     243 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 63 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     141 as libc::c_int as libc::c_short, 216 as libc::c_int as libc::c_short,
     -(21 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 47 as libc::c_int as libc::c_short,
     242 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     110 as libc::c_int as libc::c_short, 156 as libc::c_int as libc::c_short,
     15 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     201 as libc::c_int as libc::c_short, 94 as libc::c_int as libc::c_short,
     171 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     125 as libc::c_int as libc::c_short, 215 as libc::c_int as libc::c_short,
     78 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     200 as libc::c_int as libc::c_short, 214 as libc::c_int as libc::c_short,
     62 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     185 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     155 as libc::c_int as libc::c_short, 170 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 31 as libc::c_int as libc::c_short,
     241 as libc::c_int as libc::c_short,
     -(23 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     240 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     186 as libc::c_int as libc::c_short, 229 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     228 as libc::c_int as libc::c_short, 140 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     109 as libc::c_int as libc::c_short, 227 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     226 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 46 as libc::c_int as libc::c_short,
     14 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     30 as libc::c_int as libc::c_short, 225 as libc::c_int as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     224 as libc::c_int as libc::c_short, 93 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     213 as libc::c_int as libc::c_short, 124 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     199 as libc::c_int as libc::c_short, 77 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     139 as libc::c_int as libc::c_short, 184 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     212 as libc::c_int as libc::c_short, 154 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     169 as libc::c_int as libc::c_short, 108 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     198 as libc::c_int as libc::c_short, 61 as libc::c_int as libc::c_short,
     -(37 as libc::c_int) as libc::c_short,
     -(21 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     211 as libc::c_int as libc::c_short, 123 as libc::c_int as libc::c_short,
     45 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     210 as libc::c_int as libc::c_short, 29 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     183 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 92 as libc::c_int as libc::c_short,
     197 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     153 as libc::c_int as libc::c_short, 122 as libc::c_int as libc::c_short,
     195 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     167 as libc::c_int as libc::c_short, 151 as libc::c_int as libc::c_short,
     75 as libc::c_int as libc::c_short, 209 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 13 as libc::c_int as libc::c_short,
     208 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     138 as libc::c_int as libc::c_short, 168 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 76 as libc::c_int as libc::c_short,
     196 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     107 as libc::c_int as libc::c_short, 182 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 60 as libc::c_int as libc::c_short,
     44 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     194 as libc::c_int as libc::c_short, 91 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     181 as libc::c_int as libc::c_short, 137 as libc::c_int as libc::c_short,
     28 as libc::c_int as libc::c_short,
     -(43 as libc::c_int) as libc::c_short,
     -(23 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     193 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     152 as libc::c_int as libc::c_short, 12 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     192 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     180 as libc::c_int as libc::c_short, 106 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     166 as libc::c_int as libc::c_short, 121 as libc::c_int as libc::c_short,
     59 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     179 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     136 as libc::c_int as libc::c_short, 90 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 43 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     165 as libc::c_int as libc::c_short, 105 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     164 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     120 as libc::c_int as libc::c_short, 135 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     148 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     119 as libc::c_int as libc::c_short, 118 as libc::c_int as libc::c_short,
     178 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 27 as libc::c_int as libc::c_short,
     177 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 11 as libc::c_int as libc::c_short,
     176 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     150 as libc::c_int as libc::c_short, 74 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 58 as libc::c_int as libc::c_short,
     163 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 89 as libc::c_int as libc::c_short,
     149 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 42 as libc::c_int as libc::c_short,
     162 as libc::c_int as libc::c_short,
     -(47 as libc::c_int) as libc::c_short,
     -(23 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 26 as libc::c_int as libc::c_short,
     161 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 10 as libc::c_int as libc::c_short,
     104 as libc::c_int as libc::c_short, 160 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     134 as libc::c_int as libc::c_short, 73 as libc::c_int as libc::c_short,
     147 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 57 as libc::c_int as libc::c_short,
     88 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     133 as libc::c_int as libc::c_short, 103 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 41 as libc::c_int as libc::c_short,
     146 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 87 as libc::c_int as libc::c_short,
     117 as libc::c_int as libc::c_short, 56 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     131 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     102 as libc::c_int as libc::c_short, 71 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     116 as libc::c_int as libc::c_short, 86 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     101 as libc::c_int as libc::c_short, 115 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 25 as libc::c_int as libc::c_short,
     145 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 9 as libc::c_int as libc::c_short,
     144 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 72 as libc::c_int as libc::c_short,
     132 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     114 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 70 as libc::c_int as libc::c_short,
     100 as libc::c_int as libc::c_short, 40 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     130 as libc::c_int as libc::c_short, 24 as libc::c_int as libc::c_short,
     -(41 as libc::c_int) as libc::c_short,
     -(27 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 55 as libc::c_int as libc::c_short,
     39 as libc::c_int as libc::c_short, 23 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     113 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 85 as libc::c_int as libc::c_short,
     7 as libc::c_int as libc::c_short, -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     112 as libc::c_int as libc::c_short, 54 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 99 as libc::c_int as libc::c_short,
     69 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 84 as libc::c_int as libc::c_short,
     38 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     98 as libc::c_int as libc::c_short, 53 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     129 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 8 as libc::c_int as libc::c_short,
     128 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 22 as libc::c_int as libc::c_short,
     97 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     6 as libc::c_int as libc::c_short, 96 as libc::c_int as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 83 as libc::c_int as libc::c_short,
     68 as libc::c_int as libc::c_short, 37 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 82 as libc::c_int as libc::c_short,
     5 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     21 as libc::c_int as libc::c_short, 81 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 52 as libc::c_int as libc::c_short,
     67 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     80 as libc::c_int as libc::c_short, 36 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 66 as libc::c_int as libc::c_short,
     51 as libc::c_int as libc::c_short, 20 as libc::c_int as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 65 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 4 as libc::c_int as libc::c_short,
     64 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 35 as libc::c_int as libc::c_short,
     50 as libc::c_int as libc::c_short, 19 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 49 as libc::c_int as libc::c_short,
     3 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     48 as libc::c_int as libc::c_short, 34 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 18 as libc::c_int as libc::c_short,
     33 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     2 as libc::c_int as libc::c_short, 32 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 17 as libc::c_int as libc::c_short,
     1 as libc::c_int as libc::c_short, 16 as libc::c_int as libc::c_short,
     0 as libc::c_int as libc::c_short];
static mut tab15: [libc::c_short; 511] =
    [-(495 as libc::c_int) as libc::c_short,
     -(445 as libc::c_int) as libc::c_short,
     -(355 as libc::c_int) as libc::c_short,
     -(263 as libc::c_int) as libc::c_short,
     -(183 as libc::c_int) as libc::c_short,
     -(115 as libc::c_int) as libc::c_short,
     -(77 as libc::c_int) as libc::c_short,
     -(43 as libc::c_int) as libc::c_short,
     -(27 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     255 as libc::c_int as libc::c_short, 239 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     254 as libc::c_int as libc::c_short, 223 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     238 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     253 as libc::c_int as libc::c_short, 207 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     252 as libc::c_int as libc::c_short, 222 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     237 as libc::c_int as libc::c_short, 191 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     251 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     206 as libc::c_int as libc::c_short, 236 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     221 as libc::c_int as libc::c_short, 175 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     250 as libc::c_int as libc::c_short, 190 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     235 as libc::c_int as libc::c_short, 205 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     220 as libc::c_int as libc::c_short, 159 as libc::c_int as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     249 as libc::c_int as libc::c_short, 234 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     189 as libc::c_int as libc::c_short, 219 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     143 as libc::c_int as libc::c_short, 248 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     204 as libc::c_int as libc::c_short, 158 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     233 as libc::c_int as libc::c_short, 127 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     247 as libc::c_int as libc::c_short, 173 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     218 as libc::c_int as libc::c_short, 188 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     111 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     174 as libc::c_int as libc::c_short, 15 as libc::c_int as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     203 as libc::c_int as libc::c_short, 246 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     142 as libc::c_int as libc::c_short, 232 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 95 as libc::c_int as libc::c_short,
     157 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     245 as libc::c_int as libc::c_short, 126 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     231 as libc::c_int as libc::c_short, 172 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     202 as libc::c_int as libc::c_short, 187 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     217 as libc::c_int as libc::c_short, 141 as libc::c_int as libc::c_short,
     79 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     244 as libc::c_int as libc::c_short, 63 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     243 as libc::c_int as libc::c_short, 216 as libc::c_int as libc::c_short,
     -(33 as libc::c_int) as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     230 as libc::c_int as libc::c_short, 47 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     242 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     110 as libc::c_int as libc::c_short, 240 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 31 as libc::c_int as libc::c_short,
     241 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     156 as libc::c_int as libc::c_short, 201 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 94 as libc::c_int as libc::c_short,
     171 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     186 as libc::c_int as libc::c_short, 229 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     125 as libc::c_int as libc::c_short, 215 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 78 as libc::c_int as libc::c_short,
     228 as libc::c_int as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     140 as libc::c_int as libc::c_short, 200 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 62 as libc::c_int as libc::c_short,
     109 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     214 as libc::c_int as libc::c_short, 227 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     155 as libc::c_int as libc::c_short, 185 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 46 as libc::c_int as libc::c_short,
     170 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     226 as libc::c_int as libc::c_short, 30 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     225 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 14 as libc::c_int as libc::c_short,
     224 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 93 as libc::c_int as libc::c_short,
     213 as libc::c_int as libc::c_short,
     -(45 as libc::c_int) as libc::c_short,
     -(25 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     124 as libc::c_int as libc::c_short, 199 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 77 as libc::c_int as libc::c_short,
     139 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     212 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     184 as libc::c_int as libc::c_short, 154 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     169 as libc::c_int as libc::c_short, 108 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     198 as libc::c_int as libc::c_short, 61 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     211 as libc::c_int as libc::c_short, 210 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 45 as libc::c_int as libc::c_short,
     13 as libc::c_int as libc::c_short, 29 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     123 as libc::c_int as libc::c_short, 183 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     209 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 92 as libc::c_int as libc::c_short,
     208 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     197 as libc::c_int as libc::c_short, 138 as libc::c_int as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     168 as libc::c_int as libc::c_short, 76 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     196 as libc::c_int as libc::c_short, 107 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     182 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     153 as libc::c_int as libc::c_short, 12 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 60 as libc::c_int as libc::c_short,
     195 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     122 as libc::c_int as libc::c_short, 167 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     166 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     192 as libc::c_int as libc::c_short, 11 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     194 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 44 as libc::c_int as libc::c_short,
     91 as libc::c_int as libc::c_short,
     -(55 as libc::c_int) as libc::c_short,
     -(29 as libc::c_int) as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     181 as libc::c_int as libc::c_short, 28 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     137 as libc::c_int as libc::c_short, 152 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     193 as libc::c_int as libc::c_short, 75 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     180 as libc::c_int as libc::c_short, 106 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 59 as libc::c_int as libc::c_short,
     121 as libc::c_int as libc::c_short, 179 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     151 as libc::c_int as libc::c_short, 136 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 43 as libc::c_int as libc::c_short,
     90 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     178 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     165 as libc::c_int as libc::c_short, 27 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     177 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     176 as libc::c_int as libc::c_short, 105 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     150 as libc::c_int as libc::c_short, 74 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     164 as libc::c_int as libc::c_short, 120 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     135 as libc::c_int as libc::c_short, 58 as libc::c_int as libc::c_short,
     163 as libc::c_int as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 89 as libc::c_int as libc::c_short,
     149 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 42 as libc::c_int as libc::c_short,
     162 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 26 as libc::c_int as libc::c_short,
     161 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 10 as libc::c_int as libc::c_short,
     160 as libc::c_int as libc::c_short, 104 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     134 as libc::c_int as libc::c_short, 73 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     148 as libc::c_int as libc::c_short, 57 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     147 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     119 as libc::c_int as libc::c_short, 9 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 88 as libc::c_int as libc::c_short,
     133 as libc::c_int as libc::c_short,
     -(53 as libc::c_int) as libc::c_short,
     -(29 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 41 as libc::c_int as libc::c_short,
     103 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     118 as libc::c_int as libc::c_short, 146 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     145 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 25 as libc::c_int as libc::c_short,
     144 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 72 as libc::c_int as libc::c_short,
     132 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 87 as libc::c_int as libc::c_short,
     117 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 56 as libc::c_int as libc::c_short,
     131 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     102 as libc::c_int as libc::c_short, 71 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 40 as libc::c_int as libc::c_short,
     130 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 24 as libc::c_int as libc::c_short,
     129 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     116 as libc::c_int as libc::c_short, 8 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     128 as libc::c_int as libc::c_short, 86 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     101 as libc::c_int as libc::c_short, 55 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     115 as libc::c_int as libc::c_short, 70 as libc::c_int as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 39 as libc::c_int as libc::c_short,
     114 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     100 as libc::c_int as libc::c_short, 23 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 85 as libc::c_int as libc::c_short,
     113 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 7 as libc::c_int as libc::c_short,
     112 as libc::c_int as libc::c_short, 54 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 99 as libc::c_int as libc::c_short,
     69 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     84 as libc::c_int as libc::c_short, 38 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 98 as libc::c_int as libc::c_short,
     22 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 6 as libc::c_int as libc::c_short,
     96 as libc::c_int as libc::c_short, 53 as libc::c_int as libc::c_short,
     -(33 as libc::c_int) as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 97 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 83 as libc::c_int as libc::c_short,
     68 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     37 as libc::c_int as libc::c_short, 82 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 21 as libc::c_int as libc::c_short,
     81 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 5 as libc::c_int as libc::c_short,
     80 as libc::c_int as libc::c_short, 52 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 67 as libc::c_int as libc::c_short,
     36 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     66 as libc::c_int as libc::c_short, 51 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 65 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 20 as libc::c_int as libc::c_short,
     4 as libc::c_int as libc::c_short, -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 35 as libc::c_int as libc::c_short,
     50 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 64 as libc::c_int as libc::c_short,
     3 as libc::c_int as libc::c_short, 19 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 49 as libc::c_int as libc::c_short,
     48 as libc::c_int as libc::c_short, 34 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 18 as libc::c_int as libc::c_short,
     33 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     2 as libc::c_int as libc::c_short, 32 as libc::c_int as libc::c_short,
     17 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 1 as libc::c_int as libc::c_short,
     16 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short];
static mut tab16: [libc::c_short; 511] =
    [-(509 as libc::c_int) as libc::c_short,
     -(503 as libc::c_int) as libc::c_short,
     -(461 as libc::c_int) as libc::c_short,
     -(323 as libc::c_int) as libc::c_short,
     -(103 as libc::c_int) as libc::c_short,
     -(37 as libc::c_int) as libc::c_short,
     -(27 as libc::c_int) as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     239 as libc::c_int as libc::c_short, 254 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     223 as libc::c_int as libc::c_short, 253 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     207 as libc::c_int as libc::c_short, 252 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     191 as libc::c_int as libc::c_short, 251 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     175 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     250 as libc::c_int as libc::c_short, 159 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     249 as libc::c_int as libc::c_short, 248 as libc::c_int as libc::c_short,
     143 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     127 as libc::c_int as libc::c_short, 247 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     111 as libc::c_int as libc::c_short, 246 as libc::c_int as libc::c_short,
     255 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 95 as libc::c_int as libc::c_short,
     245 as libc::c_int as libc::c_short, 79 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     244 as libc::c_int as libc::c_short, 243 as libc::c_int as libc::c_short,
     -(53 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     240 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 63 as libc::c_int as libc::c_short,
     -(29 as libc::c_int) as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     206 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     236 as libc::c_int as libc::c_short, 221 as libc::c_int as libc::c_short,
     222 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     233 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     234 as libc::c_int as libc::c_short, 217 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     238 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     237 as libc::c_int as libc::c_short, 235 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     190 as libc::c_int as libc::c_short, 205 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     220 as libc::c_int as libc::c_short, 219 as libc::c_int as libc::c_short,
     174 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     204 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     173 as libc::c_int as libc::c_short, 218 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     126 as libc::c_int as libc::c_short, 172 as libc::c_int as libc::c_short,
     202 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     201 as libc::c_int as libc::c_short, 125 as libc::c_int as libc::c_short,
     94 as libc::c_int as libc::c_short, 189 as libc::c_int as libc::c_short,
     242 as libc::c_int as libc::c_short,
     -(93 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 47 as libc::c_int as libc::c_short,
     15 as libc::c_int as libc::c_short, 31 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     241 as libc::c_int as libc::c_short,
     -(49 as libc::c_int) as libc::c_short,
     -(25 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     158 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     188 as libc::c_int as libc::c_short, 203 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     142 as libc::c_int as libc::c_short, 232 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     157 as libc::c_int as libc::c_short, 231 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     187 as libc::c_int as libc::c_short, 141 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     216 as libc::c_int as libc::c_short, 110 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     230 as libc::c_int as libc::c_short, 156 as libc::c_int as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     171 as libc::c_int as libc::c_short, 186 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     229 as libc::c_int as libc::c_short, 215 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 78 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     228 as libc::c_int as libc::c_short, 140 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     200 as libc::c_int as libc::c_short, 62 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     109 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     214 as libc::c_int as libc::c_short, 155 as libc::c_int as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     185 as libc::c_int as libc::c_short, 170 as libc::c_int as libc::c_short,
     225 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     212 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     184 as libc::c_int as libc::c_short, 169 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     123 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     183 as libc::c_int as libc::c_short, 208 as libc::c_int as libc::c_short,
     227 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 14 as libc::c_int as libc::c_short,
     224 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 93 as libc::c_int as libc::c_short,
     213 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     124 as libc::c_int as libc::c_short, 199 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 77 as libc::c_int as libc::c_short,
     139 as libc::c_int as libc::c_short,
     -(75 as libc::c_int) as libc::c_short,
     -(45 as libc::c_int) as libc::c_short,
     -(27 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     154 as libc::c_int as libc::c_short, 108 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     198 as libc::c_int as libc::c_short, 61 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 92 as libc::c_int as libc::c_short,
     197 as libc::c_int as libc::c_short, 13 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     138 as libc::c_int as libc::c_short, 168 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     153 as libc::c_int as libc::c_short, 76 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     182 as libc::c_int as libc::c_short, 122 as libc::c_int as libc::c_short,
     60 as libc::c_int as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 91 as libc::c_int as libc::c_short,
     137 as libc::c_int as libc::c_short, 28 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     192 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     152 as libc::c_int as libc::c_short, 121 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     226 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 46 as libc::c_int as libc::c_short,
     30 as libc::c_int as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     211 as libc::c_int as libc::c_short, 45 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     210 as libc::c_int as libc::c_short, 209 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 59 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     151 as libc::c_int as libc::c_short, 136 as libc::c_int as libc::c_short,
     29 as libc::c_int as libc::c_short, -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     196 as libc::c_int as libc::c_short, 107 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     195 as libc::c_int as libc::c_short, 167 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 44 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     194 as libc::c_int as libc::c_short, 181 as libc::c_int as libc::c_short,
     -(23 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     193 as libc::c_int as libc::c_short, 12 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 75 as libc::c_int as libc::c_short,
     180 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     106 as libc::c_int as libc::c_short, 166 as libc::c_int as libc::c_short,
     179 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 90 as libc::c_int as libc::c_short,
     165 as libc::c_int as libc::c_short, 43 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     178 as libc::c_int as libc::c_short, 27 as libc::c_int as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     177 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 11 as libc::c_int as libc::c_short,
     176 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     105 as libc::c_int as libc::c_short, 150 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 74 as libc::c_int as libc::c_short,
     164 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     120 as libc::c_int as libc::c_short, 135 as libc::c_int as libc::c_short,
     163 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 58 as libc::c_int as libc::c_short,
     89 as libc::c_int as libc::c_short, 42 as libc::c_int as libc::c_short,
     -(97 as libc::c_int) as libc::c_short,
     -(57 as libc::c_int) as libc::c_short,
     -(33 as libc::c_int) as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     149 as libc::c_int as libc::c_short, 104 as libc::c_int as libc::c_short,
     161 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     134 as libc::c_int as libc::c_short, 119 as libc::c_int as libc::c_short,
     148 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 73 as libc::c_int as libc::c_short,
     87 as libc::c_int as libc::c_short, 103 as libc::c_int as libc::c_short,
     162 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 26 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 10 as libc::c_int as libc::c_short,
     160 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 57 as libc::c_int as libc::c_short,
     147 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 88 as libc::c_int as libc::c_short,
     133 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 41 as libc::c_int as libc::c_short,
     146 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     118 as libc::c_int as libc::c_short, 9 as libc::c_int as libc::c_short,
     25 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     145 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     144 as libc::c_int as libc::c_short, 72 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     132 as libc::c_int as libc::c_short, 117 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 56 as libc::c_int as libc::c_short,
     131 as libc::c_int as libc::c_short,
     -(21 as libc::c_int) as libc::c_short,
     -(11 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     102 as libc::c_int as libc::c_short, 40 as libc::c_int as libc::c_short,
     130 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 71 as libc::c_int as libc::c_short,
     116 as libc::c_int as libc::c_short, 24 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     129 as libc::c_int as libc::c_short, 128 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 8 as libc::c_int as libc::c_short,
     86 as libc::c_int as libc::c_short, 55 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     115 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     101 as libc::c_int as libc::c_short, 70 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 39 as libc::c_int as libc::c_short,
     114 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     100 as libc::c_int as libc::c_short, 85 as libc::c_int as libc::c_short,
     7 as libc::c_int as libc::c_short, 23 as libc::c_int as libc::c_short,
     -(23 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     113 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     112 as libc::c_int as libc::c_short, 54 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 99 as libc::c_int as libc::c_short,
     69 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     84 as libc::c_int as libc::c_short, 38 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 98 as libc::c_int as libc::c_short,
     22 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     97 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     6 as libc::c_int as libc::c_short, 96 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 83 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 53 as libc::c_int as libc::c_short,
     68 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     37 as libc::c_int as libc::c_short, 82 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 81 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 21 as libc::c_int as libc::c_short,
     5 as libc::c_int as libc::c_short, -(33 as libc::c_int) as libc::c_short,
     -(23 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 52 as libc::c_int as libc::c_short,
     67 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     80 as libc::c_int as libc::c_short, 36 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 66 as libc::c_int as libc::c_short,
     51 as libc::c_int as libc::c_short, 20 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 65 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 4 as libc::c_int as libc::c_short,
     64 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     35 as libc::c_int as libc::c_short, 50 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 19 as libc::c_int as libc::c_short,
     49 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 3 as libc::c_int as libc::c_short,
     48 as libc::c_int as libc::c_short, 34 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 18 as libc::c_int as libc::c_short,
     33 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     2 as libc::c_int as libc::c_short, 32 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 17 as libc::c_int as libc::c_short,
     1 as libc::c_int as libc::c_short, 16 as libc::c_int as libc::c_short,
     0 as libc::c_int as libc::c_short];
static mut tab24: [libc::c_short; 511] =
    [-(451 as libc::c_int) as libc::c_short,
     -(117 as libc::c_int) as libc::c_short,
     -(43 as libc::c_int) as libc::c_short,
     -(25 as libc::c_int) as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     239 as libc::c_int as libc::c_short, 254 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     223 as libc::c_int as libc::c_short, 253 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     207 as libc::c_int as libc::c_short, 252 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     191 as libc::c_int as libc::c_short, 251 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     250 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     175 as libc::c_int as libc::c_short, 159 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     249 as libc::c_int as libc::c_short, 248 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     143 as libc::c_int as libc::c_short, 127 as libc::c_int as libc::c_short,
     247 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     111 as libc::c_int as libc::c_short, 246 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 95 as libc::c_int as libc::c_short,
     245 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 79 as libc::c_int as libc::c_short,
     244 as libc::c_int as libc::c_short,
     -(71 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 63 as libc::c_int as libc::c_short,
     243 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 47 as libc::c_int as libc::c_short,
     242 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     241 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 31 as libc::c_int as libc::c_short,
     240 as libc::c_int as libc::c_short,
     -(25 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 15 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     238 as libc::c_int as libc::c_short, 222 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     237 as libc::c_int as libc::c_short, 206 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     236 as libc::c_int as libc::c_short, 221 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     190 as libc::c_int as libc::c_short, 235 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     205 as libc::c_int as libc::c_short, 220 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     174 as libc::c_int as libc::c_short, 234 as libc::c_int as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     189 as libc::c_int as libc::c_short, 219 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     204 as libc::c_int as libc::c_short, 158 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     233 as libc::c_int as libc::c_short, 173 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     218 as libc::c_int as libc::c_short, 188 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     203 as libc::c_int as libc::c_short, 142 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     232 as libc::c_int as libc::c_short, 157 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     217 as libc::c_int as libc::c_short, 126 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     231 as libc::c_int as libc::c_short, 172 as libc::c_int as libc::c_short,
     255 as libc::c_int as libc::c_short,
     -(235 as libc::c_int) as libc::c_short,
     -(143 as libc::c_int) as libc::c_short,
     -(77 as libc::c_int) as libc::c_short,
     -(45 as libc::c_int) as libc::c_short,
     -(25 as libc::c_int) as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     202 as libc::c_int as libc::c_short, 187 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     141 as libc::c_int as libc::c_short, 216 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 14 as libc::c_int as libc::c_short,
     224 as libc::c_int as libc::c_short, 13 as libc::c_int as libc::c_short,
     230 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     110 as libc::c_int as libc::c_short, 156 as libc::c_int as libc::c_short,
     201 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 94 as libc::c_int as libc::c_short,
     186 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     229 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     171 as libc::c_int as libc::c_short, 125 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     215 as libc::c_int as libc::c_short, 228 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     140 as libc::c_int as libc::c_short, 200 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 78 as libc::c_int as libc::c_short,
     46 as libc::c_int as libc::c_short, 62 as libc::c_int as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     109 as libc::c_int as libc::c_short, 214 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     227 as libc::c_int as libc::c_short, 155 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     185 as libc::c_int as libc::c_short, 170 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     226 as libc::c_int as libc::c_short, 30 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     225 as libc::c_int as libc::c_short, 93 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     213 as libc::c_int as libc::c_short, 124 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     199 as libc::c_int as libc::c_short, 77 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     139 as libc::c_int as libc::c_short, 184 as libc::c_int as libc::c_short,
     -(31 as libc::c_int) as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     212 as libc::c_int as libc::c_short, 154 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     169 as libc::c_int as libc::c_short, 108 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     198 as libc::c_int as libc::c_short, 61 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     211 as libc::c_int as libc::c_short, 45 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     210 as libc::c_int as libc::c_short, 29 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     123 as libc::c_int as libc::c_short, 183 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     209 as libc::c_int as libc::c_short, 92 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     197 as libc::c_int as libc::c_short, 138 as libc::c_int as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     168 as libc::c_int as libc::c_short, 153 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 76 as libc::c_int as libc::c_short,
     196 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     107 as libc::c_int as libc::c_short, 182 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     208 as libc::c_int as libc::c_short, 12 as libc::c_int as libc::c_short,
     60 as libc::c_int as libc::c_short, -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     195 as libc::c_int as libc::c_short, 122 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     167 as libc::c_int as libc::c_short, 44 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     194 as libc::c_int as libc::c_short, 91 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     181 as libc::c_int as libc::c_short, 28 as libc::c_int as libc::c_short,
     -(57 as libc::c_int) as libc::c_short,
     -(35 as libc::c_int) as libc::c_short,
     -(19 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     137 as libc::c_int as libc::c_short, 152 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     193 as libc::c_int as libc::c_short, 75 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     192 as libc::c_int as libc::c_short, 11 as libc::c_int as libc::c_short,
     59 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     176 as libc::c_int as libc::c_short, 10 as libc::c_int as libc::c_short,
     26 as libc::c_int as libc::c_short, -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     180 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     106 as libc::c_int as libc::c_short, 166 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     121 as libc::c_int as libc::c_short, 151 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     160 as libc::c_int as libc::c_short, 9 as libc::c_int as libc::c_short,
     144 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     179 as libc::c_int as libc::c_short, 136 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 43 as libc::c_int as libc::c_short,
     90 as libc::c_int as libc::c_short, 178 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     165 as libc::c_int as libc::c_short, 27 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     177 as libc::c_int as libc::c_short, 105 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     150 as libc::c_int as libc::c_short, 164 as libc::c_int as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 74 as libc::c_int as libc::c_short,
     120 as libc::c_int as libc::c_short, 135 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 58 as libc::c_int as libc::c_short,
     163 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 89 as libc::c_int as libc::c_short,
     149 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 42 as libc::c_int as libc::c_short,
     162 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     161 as libc::c_int as libc::c_short, 104 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     134 as libc::c_int as libc::c_short, 119 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 73 as libc::c_int as libc::c_short,
     148 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 57 as libc::c_int as libc::c_short,
     147 as libc::c_int as libc::c_short,
     -(63 as libc::c_int) as libc::c_short,
     -(31 as libc::c_int) as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 88 as libc::c_int as libc::c_short,
     133 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 41 as libc::c_int as libc::c_short,
     103 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     118 as libc::c_int as libc::c_short, 146 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 25 as libc::c_int as libc::c_short,
     145 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 72 as libc::c_int as libc::c_short,
     132 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 87 as libc::c_int as libc::c_short,
     117 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 56 as libc::c_int as libc::c_short,
     131 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     102 as libc::c_int as libc::c_short, 40 as libc::c_int as libc::c_short,
     -(17 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     130 as libc::c_int as libc::c_short, 24 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 71 as libc::c_int as libc::c_short,
     116 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short,
     129 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 8 as libc::c_int as libc::c_short,
     128 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 86 as libc::c_int as libc::c_short,
     101 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 23 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 7 as libc::c_int as libc::c_short,
     112 as libc::c_int as libc::c_short, 115 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 55 as libc::c_int as libc::c_short,
     39 as libc::c_int as libc::c_short, 114 as libc::c_int as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 70 as libc::c_int as libc::c_short,
     100 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 85 as libc::c_int as libc::c_short,
     113 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 54 as libc::c_int as libc::c_short,
     99 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     69 as libc::c_int as libc::c_short, 84 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 38 as libc::c_int as libc::c_short,
     98 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     22 as libc::c_int as libc::c_short, 97 as libc::c_int as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 6 as libc::c_int as libc::c_short,
     96 as libc::c_int as libc::c_short, 53 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 83 as libc::c_int as libc::c_short,
     68 as libc::c_int as libc::c_short,
     -(51 as libc::c_int) as libc::c_short,
     -(37 as libc::c_int) as libc::c_short,
     -(23 as libc::c_int) as libc::c_short,
     -(15 as libc::c_int) as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 37 as libc::c_int as libc::c_short,
     82 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     21 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     5 as libc::c_int as libc::c_short, 80 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 81 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 52 as libc::c_int as libc::c_short,
     67 as libc::c_int as libc::c_short, -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 36 as libc::c_int as libc::c_short,
     66 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     51 as libc::c_int as libc::c_short, 20 as libc::c_int as libc::c_short,
     -(9 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 65 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 4 as libc::c_int as libc::c_short,
     64 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     35 as libc::c_int as libc::c_short, 50 as libc::c_int as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 19 as libc::c_int as libc::c_short,
     49 as libc::c_int as libc::c_short, -(7 as libc::c_int) as libc::c_short,
     -(5 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 3 as libc::c_int as libc::c_short,
     48 as libc::c_int as libc::c_short, 34 as libc::c_int as libc::c_short,
     18 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     33 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     2 as libc::c_int as libc::c_short, 32 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 17 as libc::c_int as libc::c_short,
     1 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     16 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short];
static mut tab_c0: [libc::c_short; 31] =
    [-(29 as libc::c_int) as libc::c_short,
     -(21 as libc::c_int) as libc::c_short,
     -(13 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 11 as libc::c_int as libc::c_short,
     15 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     13 as libc::c_int as libc::c_short, 14 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 7 as libc::c_int as libc::c_short,
     5 as libc::c_int as libc::c_short, 9 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 6 as libc::c_int as libc::c_short,
     3 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     10 as libc::c_int as libc::c_short, 12 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 2 as libc::c_int as libc::c_short,
     1 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     4 as libc::c_int as libc::c_short, 8 as libc::c_int as libc::c_short,
     0 as libc::c_int as libc::c_short];
static mut tab_c1: [libc::c_short; 31] =
    [-(15 as libc::c_int) as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 15 as libc::c_int as libc::c_short,
     14 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     13 as libc::c_int as libc::c_short, 12 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 11 as libc::c_int as libc::c_short,
     10 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     9 as libc::c_int as libc::c_short, 8 as libc::c_int as libc::c_short,
     -(7 as libc::c_int) as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 7 as libc::c_int as libc::c_short,
     6 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     5 as libc::c_int as libc::c_short, 4 as libc::c_int as libc::c_short,
     -(3 as libc::c_int) as libc::c_short,
     -(1 as libc::c_int) as libc::c_short, 3 as libc::c_int as libc::c_short,
     2 as libc::c_int as libc::c_short, -(1 as libc::c_int) as libc::c_short,
     1 as libc::c_int as libc::c_short, 0 as libc::c_int as libc::c_short];
static mut ht: [newhuff; 32] =
    unsafe {
        [{
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab0.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab1.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab2.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab3.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab0.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab5.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab6.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab7.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab8.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab9.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab10.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab11.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab12.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab13.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab0.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab15.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 1 as libc::c_int as uint,
                         table: tab16.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 2 as libc::c_int as uint,
                         table: tab16.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 3 as libc::c_int as uint,
                         table: tab16.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 4 as libc::c_int as uint,
                         table: tab16.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 6 as libc::c_int as uint,
                         table: tab16.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 8 as libc::c_int as uint,
                         table: tab16.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 10 as libc::c_int as uint,
                         table: tab16.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 13 as libc::c_int as uint,
                         table: tab16.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 4 as libc::c_int as uint,
                         table: tab24.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 5 as libc::c_int as uint,
                         table: tab24.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 6 as libc::c_int as uint,
                         table: tab24.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 7 as libc::c_int as uint,
                         table: tab24.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 8 as libc::c_int as uint,
                         table: tab24.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 9 as libc::c_int as uint,
                         table: tab24.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 11 as libc::c_int as uint,
                         table: tab24.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 13 as libc::c_int as uint,
                         table: tab24.as_ptr(),};
             init
         }]
    };
static mut htc: [newhuff; 2] =
    unsafe {
        [{
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab_c0.as_ptr(),};
             init
         },
         {
             let mut init =
                 newhuff{linbits: 0 as libc::c_int as uint,
                         table: tab_c1.as_ptr(),};
             init
         }]
    };
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
// making those two signed int as workaround for open64/pathscale/sun compilers,
	// and also for consistency, since they're worked on together with other signed variables.
/*
layer3.c - compact version of famous library mpg123
Copyright (C) 2017 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// static one-time calculated tables... or so
#[no_mangle]
pub static mut COS6_1: libc::c_float = 0.;
// dct12 wants to use that
#[no_mangle]
pub static mut COS6_2: libc::c_float = 0.;
// dct12 wants to use that
#[no_mangle]
pub static mut cos9: [libc::c_float; 3] = [0.; 3];
// dct36 wants to use that
#[no_mangle]
pub static mut cos18: [libc::c_float; 3] = [0.; 3];
// dct36 wants to use that
#[no_mangle]
pub static mut tfcos12: [libc::c_float; 3] = [0.; 3];
// dct12 wants to use that
#[no_mangle]
pub static mut tfcos36: [libc::c_float; 9] = [0.; 9];
// dct36 wants to use that
static mut ispow: [libc::c_float; 8207] = [0.; 8207];
static mut COS9: [libc::c_float; 9] = [0.; 9];
static mut aa_ca: [libc::c_float; 8] = [0.; 8];
static mut aa_cs: [libc::c_float; 8] = [0.; 8];
static mut win: [[libc::c_float; 36]; 4] = [[0.; 36]; 4];
static mut win1: [[libc::c_float; 36]; 4] = [[0.; 36]; 4];
static mut tan1_1: [libc::c_float; 16] = [0.; 16];
static mut tan2_1: [libc::c_float; 16] = [0.; 16];
static mut tan1_2: [libc::c_float; 16] = [0.; 16];
static mut tan2_2: [libc::c_float; 16] = [0.; 16];
static mut pow1_1: [[libc::c_float; 16]; 2] = [[0.; 16]; 2];
static mut pow2_1: [[libc::c_float; 16]; 2] = [[0.; 16]; 2];
static mut pow1_2: [[libc::c_float; 16]; 2] = [[0.; 16]; 2];
static mut pow2_2: [[libc::c_float; 16]; 2] = [[0.; 16]; 2];
static mut mapbuf0: [[libc::c_int; 152]; 9] = [[0; 152]; 9];
static mut mapbuf1: [[libc::c_int; 156]; 9] = [[0; 156]; 9];
static mut mapbuf2: [[libc::c_int; 44]; 9] = [[0; 44]; 9];
static mut map: [[*mut libc::c_int; 3]; 9] =
    [[0 as *const libc::c_int as *mut libc::c_int; 3]; 9];
static mut mapend: [[*mut libc::c_int; 3]; 9] =
    [[0 as *const libc::c_int as *mut libc::c_int; 3]; 9];
static mut n_slen2: [uint; 512] = [0; 512];
// MPEG 2.0 slen for 'normal' mode
static mut i_slen2: [uint; 256] = [0; 256];
// techy details about our friendly MPEG data. Fairly constant over the years ;-)
static mut bandInfo: [bandInfoStruct; 9] =
    [{
         let mut init =
             bandInfoStruct{longIdx:
                                [0 as libc::c_int as word,
                                 4 as libc::c_int as word,
                                 8 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 16 as libc::c_int as word,
                                 20 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 30 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 44 as libc::c_int as word,
                                 52 as libc::c_int as word,
                                 62 as libc::c_int as word,
                                 74 as libc::c_int as word,
                                 90 as libc::c_int as word,
                                 110 as libc::c_int as word,
                                 134 as libc::c_int as word,
                                 162 as libc::c_int as word,
                                 196 as libc::c_int as word,
                                 238 as libc::c_int as word,
                                 288 as libc::c_int as word,
                                 342 as libc::c_int as word,
                                 418 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            longDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 28 as libc::c_int as byte,
                                 34 as libc::c_int as byte,
                                 42 as libc::c_int as byte,
                                 50 as libc::c_int as byte,
                                 54 as libc::c_int as byte,
                                 76 as libc::c_int as byte,
                                 158 as libc::c_int as byte],
                            shortIdx:
                                [0 as libc::c_int as word,
                                 (4 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (8 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (12 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (16 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (22 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (30 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (40 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (52 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (66 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (84 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (106 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (136 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (192 as libc::c_int * 3 as libc::c_int) as
                                     word],
                            shortDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 18 as libc::c_int as byte,
                                 22 as libc::c_int as byte,
                                 30 as libc::c_int as byte,
                                 56 as libc::c_int as byte],};
         init
     },
     {
         let mut init =
             bandInfoStruct{longIdx:
                                [0 as libc::c_int as word,
                                 4 as libc::c_int as word,
                                 8 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 16 as libc::c_int as word,
                                 20 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 30 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 42 as libc::c_int as word,
                                 50 as libc::c_int as word,
                                 60 as libc::c_int as word,
                                 72 as libc::c_int as word,
                                 88 as libc::c_int as word,
                                 106 as libc::c_int as word,
                                 128 as libc::c_int as word,
                                 156 as libc::c_int as word,
                                 190 as libc::c_int as word,
                                 230 as libc::c_int as word,
                                 276 as libc::c_int as word,
                                 330 as libc::c_int as word,
                                 384 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            longDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 18 as libc::c_int as byte,
                                 22 as libc::c_int as byte,
                                 28 as libc::c_int as byte,
                                 34 as libc::c_int as byte,
                                 40 as libc::c_int as byte,
                                 46 as libc::c_int as byte,
                                 54 as libc::c_int as byte,
                                 54 as libc::c_int as byte,
                                 192 as libc::c_int as byte],
                            shortIdx:
                                [0 as libc::c_int as word,
                                 (4 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (8 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (12 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (16 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (22 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (28 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (38 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (50 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (64 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (80 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (100 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (126 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (192 as libc::c_int * 3 as libc::c_int) as
                                     word],
                            shortDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 26 as libc::c_int as byte,
                                 66 as libc::c_int as byte],};
         init
     },
     {
         let mut init =
             bandInfoStruct{longIdx:
                                [0 as libc::c_int as word,
                                 4 as libc::c_int as word,
                                 8 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 16 as libc::c_int as word,
                                 20 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 30 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 44 as libc::c_int as word,
                                 54 as libc::c_int as word,
                                 66 as libc::c_int as word,
                                 82 as libc::c_int as word,
                                 102 as libc::c_int as word,
                                 126 as libc::c_int as word,
                                 156 as libc::c_int as word,
                                 194 as libc::c_int as word,
                                 240 as libc::c_int as word,
                                 296 as libc::c_int as word,
                                 364 as libc::c_int as word,
                                 448 as libc::c_int as word,
                                 550 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            longDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 30 as libc::c_int as byte,
                                 38 as libc::c_int as byte,
                                 46 as libc::c_int as byte,
                                 56 as libc::c_int as byte,
                                 68 as libc::c_int as byte,
                                 84 as libc::c_int as byte,
                                 102 as libc::c_int as byte,
                                 26 as libc::c_int as byte],
                            shortIdx:
                                [0 as libc::c_int as word,
                                 (4 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (8 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (12 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (16 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (22 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (30 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (42 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (58 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (78 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (104 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (138 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (180 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (192 as libc::c_int * 3 as libc::c_int) as
                                     word],
                            shortDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 26 as libc::c_int as byte,
                                 34 as libc::c_int as byte,
                                 42 as libc::c_int as byte,
                                 12 as libc::c_int as byte],};
         init
     },
     {
         let mut init =
             bandInfoStruct{longIdx:
                                [0 as libc::c_int as word,
                                 6 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 18 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 30 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 44 as libc::c_int as word,
                                 54 as libc::c_int as word,
                                 66 as libc::c_int as word,
                                 80 as libc::c_int as word,
                                 96 as libc::c_int as word,
                                 116 as libc::c_int as word,
                                 140 as libc::c_int as word,
                                 168 as libc::c_int as word,
                                 200 as libc::c_int as word,
                                 238 as libc::c_int as word,
                                 284 as libc::c_int as word,
                                 336 as libc::c_int as word,
                                 396 as libc::c_int as word,
                                 464 as libc::c_int as word,
                                 522 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            longDiff:
                                [6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 28 as libc::c_int as byte,
                                 32 as libc::c_int as byte,
                                 38 as libc::c_int as byte,
                                 46 as libc::c_int as byte,
                                 52 as libc::c_int as byte,
                                 60 as libc::c_int as byte,
                                 68 as libc::c_int as byte,
                                 58 as libc::c_int as byte,
                                 54 as libc::c_int as byte],
                            shortIdx:
                                [0 as libc::c_int as word,
                                 (4 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (8 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (12 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (18 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (24 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (32 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (42 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (56 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (74 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (100 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (132 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (174 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (192 as libc::c_int * 3 as libc::c_int) as
                                     word],
                            shortDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 18 as libc::c_int as byte,
                                 26 as libc::c_int as byte,
                                 32 as libc::c_int as byte,
                                 42 as libc::c_int as byte,
                                 18 as libc::c_int as byte],};
         init
     },
     {
         let mut init =
             bandInfoStruct{longIdx:
                                [0 as libc::c_int as word,
                                 6 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 18 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 30 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 44 as libc::c_int as word,
                                 54 as libc::c_int as word,
                                 66 as libc::c_int as word,
                                 80 as libc::c_int as word,
                                 96 as libc::c_int as word,
                                 114 as libc::c_int as word,
                                 136 as libc::c_int as word,
                                 162 as libc::c_int as word,
                                 194 as libc::c_int as word,
                                 232 as libc::c_int as word,
                                 278 as libc::c_int as word,
                                 332 as libc::c_int as word,
                                 394 as libc::c_int as word,
                                 464 as libc::c_int as word,
                                 540 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            longDiff:
                                [6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 18 as libc::c_int as byte,
                                 22 as libc::c_int as byte,
                                 26 as libc::c_int as byte,
                                 32 as libc::c_int as byte,
                                 38 as libc::c_int as byte,
                                 46 as libc::c_int as byte,
                                 54 as libc::c_int as byte,
                                 62 as libc::c_int as byte,
                                 70 as libc::c_int as byte,
                                 76 as libc::c_int as byte,
                                 36 as libc::c_int as byte],
                            shortIdx:
                                [0 as libc::c_int as word,
                                 (4 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (8 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (12 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (18 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (26 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (36 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (48 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (62 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (80 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (104 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (136 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (180 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (192 as libc::c_int * 3 as libc::c_int) as
                                     word],
                            shortDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 18 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 32 as libc::c_int as byte,
                                 44 as libc::c_int as byte,
                                 12 as libc::c_int as byte],};
         init
     },
     {
         let mut init =
             bandInfoStruct{longIdx:
                                [0 as libc::c_int as word,
                                 6 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 18 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 30 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 44 as libc::c_int as word,
                                 54 as libc::c_int as word,
                                 66 as libc::c_int as word,
                                 80 as libc::c_int as word,
                                 96 as libc::c_int as word,
                                 116 as libc::c_int as word,
                                 140 as libc::c_int as word,
                                 168 as libc::c_int as word,
                                 200 as libc::c_int as word,
                                 238 as libc::c_int as word,
                                 284 as libc::c_int as word,
                                 336 as libc::c_int as word,
                                 396 as libc::c_int as word,
                                 464 as libc::c_int as word,
                                 522 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            longDiff:
                                [6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 28 as libc::c_int as byte,
                                 32 as libc::c_int as byte,
                                 38 as libc::c_int as byte,
                                 46 as libc::c_int as byte,
                                 52 as libc::c_int as byte,
                                 60 as libc::c_int as byte,
                                 68 as libc::c_int as byte,
                                 58 as libc::c_int as byte,
                                 54 as libc::c_int as byte],
                            shortIdx:
                                [0 as libc::c_int as word,
                                 (4 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (8 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (12 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (18 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (26 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (36 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (48 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (62 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (80 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (104 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (134 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (174 as libc::c_int * 3 as libc::c_int) as
                                     word,
                                 (192 as libc::c_int * 3 as libc::c_int) as
                                     word],
                            shortDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 18 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 30 as libc::c_int as byte,
                                 40 as libc::c_int as byte,
                                 18 as libc::c_int as byte],};
         init
     },
     {
         let mut init =
             bandInfoStruct{longIdx:
                                [0 as libc::c_int as word,
                                 6 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 18 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 30 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 44 as libc::c_int as word,
                                 54 as libc::c_int as word,
                                 66 as libc::c_int as word,
                                 80 as libc::c_int as word,
                                 96 as libc::c_int as word,
                                 116 as libc::c_int as word,
                                 140 as libc::c_int as word,
                                 168 as libc::c_int as word,
                                 200 as libc::c_int as word,
                                 238 as libc::c_int as word,
                                 284 as libc::c_int as word,
                                 336 as libc::c_int as word,
                                 396 as libc::c_int as word,
                                 464 as libc::c_int as word,
                                 522 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            longDiff:
                                [6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 28 as libc::c_int as byte,
                                 32 as libc::c_int as byte,
                                 38 as libc::c_int as byte,
                                 46 as libc::c_int as byte,
                                 52 as libc::c_int as byte,
                                 60 as libc::c_int as byte,
                                 68 as libc::c_int as byte,
                                 58 as libc::c_int as byte,
                                 54 as libc::c_int as byte],
                            shortIdx:
                                [0 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 54 as libc::c_int as word,
                                 78 as libc::c_int as word,
                                 108 as libc::c_int as word,
                                 144 as libc::c_int as word,
                                 186 as libc::c_int as word,
                                 240 as libc::c_int as word,
                                 312 as libc::c_int as word,
                                 402 as libc::c_int as word,
                                 522 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            shortDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 18 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 30 as libc::c_int as byte,
                                 40 as libc::c_int as byte,
                                 18 as libc::c_int as byte],};
         init
     },
     {
         let mut init =
             bandInfoStruct{longIdx:
                                [0 as libc::c_int as word,
                                 6 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 18 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 30 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 44 as libc::c_int as word,
                                 54 as libc::c_int as word,
                                 66 as libc::c_int as word,
                                 80 as libc::c_int as word,
                                 96 as libc::c_int as word,
                                 116 as libc::c_int as word,
                                 140 as libc::c_int as word,
                                 168 as libc::c_int as word,
                                 200 as libc::c_int as word,
                                 238 as libc::c_int as word,
                                 284 as libc::c_int as word,
                                 336 as libc::c_int as word,
                                 396 as libc::c_int as word,
                                 464 as libc::c_int as word,
                                 522 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            longDiff:
                                [6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 28 as libc::c_int as byte,
                                 32 as libc::c_int as byte,
                                 38 as libc::c_int as byte,
                                 46 as libc::c_int as byte,
                                 52 as libc::c_int as byte,
                                 60 as libc::c_int as byte,
                                 68 as libc::c_int as byte,
                                 58 as libc::c_int as byte,
                                 54 as libc::c_int as byte],
                            shortIdx:
                                [0 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 54 as libc::c_int as word,
                                 78 as libc::c_int as word,
                                 108 as libc::c_int as word,
                                 144 as libc::c_int as word,
                                 186 as libc::c_int as word,
                                 240 as libc::c_int as word,
                                 312 as libc::c_int as word,
                                 402 as libc::c_int as word,
                                 522 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            shortDiff:
                                [4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 4 as libc::c_int as byte,
                                 6 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 10 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 14 as libc::c_int as byte,
                                 18 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 30 as libc::c_int as byte,
                                 40 as libc::c_int as byte,
                                 18 as libc::c_int as byte],};
         init
     },
     {
         let mut init =
             bandInfoStruct{longIdx:
                                [0 as libc::c_int as word,
                                 12 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 36 as libc::c_int as word,
                                 48 as libc::c_int as word,
                                 60 as libc::c_int as word,
                                 72 as libc::c_int as word,
                                 88 as libc::c_int as word,
                                 108 as libc::c_int as word,
                                 132 as libc::c_int as word,
                                 160 as libc::c_int as word,
                                 192 as libc::c_int as word,
                                 232 as libc::c_int as word,
                                 280 as libc::c_int as word,
                                 336 as libc::c_int as word,
                                 400 as libc::c_int as word,
                                 476 as libc::c_int as word,
                                 566 as libc::c_int as word,
                                 568 as libc::c_int as word,
                                 570 as libc::c_int as word,
                                 572 as libc::c_int as word,
                                 574 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            longDiff:
                                [12 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 28 as libc::c_int as byte,
                                 32 as libc::c_int as byte,
                                 40 as libc::c_int as byte,
                                 48 as libc::c_int as byte,
                                 56 as libc::c_int as byte,
                                 64 as libc::c_int as byte,
                                 76 as libc::c_int as byte,
                                 90 as libc::c_int as byte,
                                 2 as libc::c_int as byte,
                                 2 as libc::c_int as byte,
                                 2 as libc::c_int as byte,
                                 2 as libc::c_int as byte,
                                 2 as libc::c_int as byte],
                            shortIdx:
                                [0 as libc::c_int as word,
                                 24 as libc::c_int as word,
                                 48 as libc::c_int as word,
                                 72 as libc::c_int as word,
                                 108 as libc::c_int as word,
                                 156 as libc::c_int as word,
                                 216 as libc::c_int as word,
                                 288 as libc::c_int as word,
                                 372 as libc::c_int as word,
                                 480 as libc::c_int as word,
                                 486 as libc::c_int as word,
                                 492 as libc::c_int as word,
                                 498 as libc::c_int as word,
                                 576 as libc::c_int as word],
                            shortDiff:
                                [8 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 8 as libc::c_int as byte,
                                 12 as libc::c_int as byte,
                                 16 as libc::c_int as byte,
                                 20 as libc::c_int as byte,
                                 24 as libc::c_int as byte,
                                 28 as libc::c_int as byte,
                                 36 as libc::c_int as byte,
                                 2 as libc::c_int as byte,
                                 2 as libc::c_int as byte,
                                 2 as libc::c_int as byte,
                                 26 as libc::c_int as byte],};
         init
     }];
static mut pretab_choice: [[byte; 22]; 2] =
    [[0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte],
     [0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 1 as libc::c_int as byte,
      1 as libc::c_int as byte, 1 as libc::c_int as byte,
      1 as libc::c_int as byte, 2 as libc::c_int as byte,
      2 as libc::c_int as byte, 3 as libc::c_int as byte,
      3 as libc::c_int as byte, 3 as libc::c_int as byte,
      2 as libc::c_int as byte, 0 as libc::c_int as byte]];
// init tables for layer-3 ... specific with the downsampling...
#[no_mangle]
pub unsafe extern "C" fn init_layer3() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8207 as libc::c_int {
        ispow[i as usize] =
            pow(i as libc::c_double, 4.0f64 / 3.0f64) as libc::c_float;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let Ci: [libc::c_double; 8] =
            [-0.6f64, -0.535f64, -0.33f64, -0.185f64, -0.095f64, -0.041f64,
             -0.0142f64, -0.0037f64];
        let mut sq: libc::c_double =
            sqrt(1.0f64 + Ci[i as usize] * Ci[i as usize]);
        aa_cs[i as usize] = (1.0f64 / sq) as libc::c_float;
        aa_ca[i as usize] = (Ci[i as usize] / sq) as libc::c_float;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 18 as libc::c_int {
        win[1 as libc::c_int as usize][i as usize] =
            (0.5f64 *
                 sin(3.14159265358979323846f64 / 72.0f64 *
                         (2 as libc::c_int * (i + 0 as libc::c_int) +
                              1 as libc::c_int) as libc::c_double) /
                 cos(3.14159265358979323846f64 *
                         (2 as libc::c_int * (i + 0 as libc::c_int) +
                              19 as libc::c_int) as libc::c_double / 72.0f64))
                as libc::c_float;
        win[0 as libc::c_int as usize][i as usize] =
            win[1 as libc::c_int as usize][i as usize];
        win[3 as libc::c_int as usize][(i + 18 as libc::c_int) as usize] =
            (0.5f64 *
                 sin(3.14159265358979323846f64 / 72.0f64 *
                         (2 as libc::c_int * (i + 18 as libc::c_int) +
                              1 as libc::c_int) as libc::c_double) /
                 cos(3.14159265358979323846f64 *
                         (2 as libc::c_int * (i + 18 as libc::c_int) +
                              19 as libc::c_int) as libc::c_double / 72.0f64))
                as libc::c_float;
        win[0 as libc::c_int as usize][(i + 18 as libc::c_int) as usize] =
            win[3 as libc::c_int as usize][(i + 18 as libc::c_int) as usize];
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        win[1 as libc::c_int as usize][(i + 18 as libc::c_int) as usize] =
            (0.5f64 /
                 cos(3.14159265358979323846f64 *
                         (2 as libc::c_int * (i + 18 as libc::c_int) +
                              19 as libc::c_int) as libc::c_double / 72.0f64))
                as libc::c_float;
        win[3 as libc::c_int as usize][(i + 12 as libc::c_int) as usize] =
            (0.5f64 /
                 cos(3.14159265358979323846f64 *
                         (2 as libc::c_int * (i + 12 as libc::c_int) +
                              19 as libc::c_int) as libc::c_double / 72.0f64))
                as libc::c_float;
        win[1 as libc::c_int as usize][(i + 24 as libc::c_int) as usize] =
            (0.5f64 *
                 sin(3.14159265358979323846f64 / 24.0f64 *
                         (2 as libc::c_int * i + 13 as libc::c_int) as
                             libc::c_double) /
                 cos(3.14159265358979323846f64 *
                         (2 as libc::c_int * (i + 24 as libc::c_int) +
                              19 as libc::c_int) as libc::c_double / 72.0f64))
                as libc::c_float;
        win[3 as libc::c_int as usize][i as usize] = 0.0f64 as libc::c_float;
        win[1 as libc::c_int as usize][(i + 30 as libc::c_int) as usize] =
            win[3 as libc::c_int as usize][i as usize];
        win[3 as libc::c_int as usize][(i + 6 as libc::c_int) as usize] =
            (0.5f64 *
                 sin(3.14159265358979323846f64 / 24.0f64 *
                         (2 as libc::c_int * i + 1 as libc::c_int) as
                             libc::c_double) /
                 cos(3.14159265358979323846f64 *
                         (2 as libc::c_int * (i + 6 as libc::c_int) +
                              19 as libc::c_int) as libc::c_double / 72.0f64))
                as libc::c_float;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        COS9[i as usize] =
            cos(3.14159265358979323846f64 / 18.0f64 * i as libc::c_double) as
                libc::c_float;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        tfcos36[i as usize] =
            (0.5f64 /
                 cos(3.14159265358979323846f64 *
                         (i * 2 as libc::c_int + 1 as libc::c_int) as
                             libc::c_double / 36.0f64)) as libc::c_float;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        tfcos12[i as usize] =
            (0.5f64 /
                 cos(3.14159265358979323846f64 *
                         (i * 2 as libc::c_int + 1 as libc::c_int) as
                             libc::c_double / 12.0f64)) as libc::c_float;
        i += 1
    }
    COS6_1 =
        cos(3.14159265358979323846f64 / 6.0f64 *
                1 as libc::c_int as libc::c_double) as libc::c_float;
    COS6_2 =
        cos(3.14159265358979323846f64 / 6.0f64 *
                2 as libc::c_int as libc::c_double) as libc::c_float;
    cos9[0 as libc::c_int as usize] =
        cos(1.0f64 * 3.14159265358979323846f64 / 9.0f64) as libc::c_float;
    cos9[1 as libc::c_int as usize] =
        cos(5.0f64 * 3.14159265358979323846f64 / 9.0f64) as libc::c_float;
    cos9[2 as libc::c_int as usize] =
        cos(7.0f64 * 3.14159265358979323846f64 / 9.0f64) as libc::c_float;
    cos18[0 as libc::c_int as usize] =
        cos(1.0f64 * 3.14159265358979323846f64 / 18.0f64) as libc::c_float;
    cos18[1 as libc::c_int as usize] =
        cos(11.0f64 * 3.14159265358979323846f64 / 18.0f64) as libc::c_float;
    cos18[2 as libc::c_int as usize] =
        cos(13.0f64 * 3.14159265358979323846f64 / 18.0f64) as libc::c_float;
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        win[2 as libc::c_int as usize][i as usize] =
            (0.5f64 *
                 sin(3.14159265358979323846f64 / 24.0f64 *
                         (2 as libc::c_int * i + 1 as libc::c_int) as
                             libc::c_double) /
                 cos(3.14159265358979323846f64 *
                         (2 as libc::c_int * i + 7 as libc::c_int) as
                             libc::c_double / 24.0f64)) as libc::c_float;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut t: libc::c_double =
            tan(i as libc::c_double * 3.14159265358979323846f64 / 12.0f64);
        tan1_1[i as usize] = (t / (1.0f64 + t)) as libc::c_float;
        tan2_1[i as usize] = (1.0f64 / (1.0f64 + t)) as libc::c_float;
        tan1_2[i as usize] =
            (1.41421356237309504880f64 * t / (1.0f64 + t)) as libc::c_float;
        tan2_2[i as usize] =
            (1.41421356237309504880f64 / (1.0f64 + t)) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            let mut base: libc::c_double =
                pow(2.0f64, -0.25f64 * (j as libc::c_double + 1.0f64));
            let mut p1: libc::c_double = 1.0f64;
            let mut p2: libc::c_double = 1.0f64;
            if i > 0 as libc::c_int {
                if i & 1 as libc::c_int != 0 {
                    p1 = pow(base, (i as libc::c_double + 1.0f64) * 0.5f64)
                } else { p2 = pow(base, i as libc::c_double * 0.5f64) }
            }
            pow1_1[j as usize][i as usize] = p1 as libc::c_float;
            pow2_1[j as usize][i as usize] = p2 as libc::c_float;
            pow1_2[j as usize][i as usize] =
                (1.41421356237309504880f64 * p1) as libc::c_float;
            pow2_2[j as usize][i as usize] =
                (1.41421356237309504880f64 * p2) as libc::c_float;
            j += 1
        }
        i += 1
    }
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        let len: [libc::c_int; 4] =
            [36 as libc::c_int, 36 as libc::c_int, 12 as libc::c_int,
             36 as libc::c_int];
        i = 0 as libc::c_int;
        while i < len[j as usize] {
            win1[j as usize][i as usize] = win[j as usize][i as usize];
            i += 2 as libc::c_int
        }
        i = 1 as libc::c_int;
        while i < len[j as usize] {
            win1[j as usize][i as usize] = -win[j as usize][i as usize];
            i += 2 as libc::c_int
        }
        j += 1
    }
    j = 0 as libc::c_int;
    while j < 9 as libc::c_int {
        let mut bi: *const bandInfoStruct =
            &*bandInfo.as_ptr().offset(j as isize) as *const bandInfoStruct;
        let mut cb: libc::c_int = 0;
        let mut lwin: libc::c_int = 0;
        let mut bdf: *const byte = 0 as *const byte;
        let mut mp: *mut libc::c_int = 0 as *mut libc::c_int;
        map[j as usize][0 as libc::c_int as usize] =
            mapbuf0[j as usize].as_mut_ptr();
        mp = map[j as usize][0 as libc::c_int as usize];
        bdf = (*bi).longDiff.as_ptr();
        i = 0 as libc::c_int;
        cb = 0 as libc::c_int;
        while cb < 8 as libc::c_int {
            let fresh0 = mp;
            mp = mp.offset(1);
            *fresh0 = *bdf as libc::c_int >> 1 as libc::c_int;
            let fresh1 = mp;
            mp = mp.offset(1);
            *fresh1 = i;
            let fresh2 = mp;
            mp = mp.offset(1);
            *fresh2 = 3 as libc::c_int;
            let fresh3 = mp;
            mp = mp.offset(1);
            *fresh3 = cb;
            cb += 1;
            let fresh4 = bdf;
            bdf = bdf.offset(1);
            i += *fresh4 as libc::c_int
        }
        bdf = (*bi).shortDiff.as_ptr().offset(3 as libc::c_int as isize);
        cb = 3 as libc::c_int;
        while cb < 13 as libc::c_int {
            let fresh5 = bdf;
            bdf = bdf.offset(1);
            let mut l_0: libc::c_int =
                *fresh5 as libc::c_int >> 1 as libc::c_int;
            lwin = 0 as libc::c_int;
            while lwin < 3 as libc::c_int {
                let fresh6 = mp;
                mp = mp.offset(1);
                *fresh6 = l_0;
                let fresh7 = mp;
                mp = mp.offset(1);
                *fresh7 = i + lwin;
                let fresh8 = mp;
                mp = mp.offset(1);
                *fresh8 = lwin;
                let fresh9 = mp;
                mp = mp.offset(1);
                *fresh9 = cb;
                lwin += 1
            }
            i += 6 as libc::c_int * l_0;
            cb += 1
        }
        mapend[j as usize][0 as libc::c_int as usize] = mp;
        map[j as usize][1 as libc::c_int as usize] =
            mapbuf1[j as usize].as_mut_ptr();
        mp = map[j as usize][1 as libc::c_int as usize];
        bdf = (*bi).shortDiff.as_ptr().offset(0 as libc::c_int as isize);
        i = 0 as libc::c_int;
        cb = 0 as libc::c_int;
        while cb < 13 as libc::c_int {
            let fresh10 = bdf;
            bdf = bdf.offset(1);
            let mut l_1: libc::c_int =
                *fresh10 as libc::c_int >> 1 as libc::c_int;
            lwin = 0 as libc::c_int;
            while lwin < 3 as libc::c_int {
                let fresh11 = mp;
                mp = mp.offset(1);
                *fresh11 = l_1;
                let fresh12 = mp;
                mp = mp.offset(1);
                *fresh12 = i + lwin;
                let fresh13 = mp;
                mp = mp.offset(1);
                *fresh13 = lwin;
                let fresh14 = mp;
                mp = mp.offset(1);
                *fresh14 = cb;
                lwin += 1
            }
            i += 6 as libc::c_int * l_1;
            cb += 1
        }
        mapend[j as usize][1 as libc::c_int as usize] = mp;
        map[j as usize][2 as libc::c_int as usize] =
            mapbuf2[j as usize].as_mut_ptr();
        mp = map[j as usize][2 as libc::c_int as usize];
        bdf = (*bi).longDiff.as_ptr();
        cb = 0 as libc::c_int;
        while cb < 22 as libc::c_int {
            let fresh15 = bdf;
            bdf = bdf.offset(1);
            let fresh16 = mp;
            mp = mp.offset(1);
            *fresh16 = *fresh15 as libc::c_int >> 1 as libc::c_int;
            let fresh17 = mp;
            mp = mp.offset(1);
            *fresh17 = cb;
            cb += 1
        }
        mapend[j as usize][2 as libc::c_int as usize] = mp;
        j += 1
    }
    // now for some serious loopings!
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                let mut n: libc::c_int =
                    k + j * 6 as libc::c_int + i * 36 as libc::c_int;
                i_slen2[n as usize] =
                    (i | j << 3 as libc::c_int | k << 6 as libc::c_int |
                         (3 as libc::c_int) << 12 as libc::c_int) as uint;
                k += 1
            }
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                let mut n_0: libc::c_int =
                    k + j * 4 as libc::c_int + i * 16 as libc::c_int;
                i_slen2[(n_0 + 180 as libc::c_int) as usize] =
                    (i | j << 3 as libc::c_int | k << 6 as libc::c_int |
                         (4 as libc::c_int) << 12 as libc::c_int) as uint;
                k += 1
            }
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            let mut n_1: libc::c_int = j + i * 3 as libc::c_int;
            i_slen2[(n_1 + 244 as libc::c_int) as usize] =
                (i | j << 3 as libc::c_int |
                     (5 as libc::c_int) << 12 as libc::c_int) as uint;
            n_slen2[(n_1 + 500 as libc::c_int) as usize] =
                (i | j << 3 as libc::c_int |
                     (2 as libc::c_int) << 12 as libc::c_int |
                     (1 as libc::c_int) << 15 as libc::c_int) as uint;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                l = 0 as libc::c_int;
                while l < 4 as libc::c_int {
                    let mut n_2: libc::c_int =
                        l + k * 4 as libc::c_int + j * 16 as libc::c_int +
                            i * 80 as libc::c_int;
                    n_slen2[n_2 as usize] =
                        (i | j << 3 as libc::c_int | k << 6 as libc::c_int |
                             l << 9 as libc::c_int |
                             (0 as libc::c_int) << 12 as libc::c_int) as uint;
                    l += 1
                }
                k += 1
            }
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                let mut n_3: libc::c_int =
                    k + j * 4 as libc::c_int + i * 20 as libc::c_int;
                n_slen2[(n_3 + 400 as libc::c_int) as usize] =
                    (i | j << 3 as libc::c_int | k << 6 as libc::c_int |
                         (1 as libc::c_int) << 12 as libc::c_int) as uint;
                k += 1
            }
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_layer3_stuff(mut fr: *mut mpg123_handle_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = -(256 as libc::c_int);
    while i < 118 as libc::c_int + 4 as libc::c_int {
        (*fr).gainpow2[(i + 256 as libc::c_int) as usize] =
            pow(2.0f64, -0.25f64 * (i + 210 as libc::c_int) as libc::c_double)
                as libc::c_float;
        i += 1
    }
    j = 0 as libc::c_int;
    while j < 9 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 23 as libc::c_int {
            (*fr).longLimit[j as usize][i as usize] =
                (bandInfo[j as usize].longIdx[i as usize] as libc::c_int -
                     1 as libc::c_int + 8 as libc::c_int) / 18 as libc::c_int
                    + 1 as libc::c_int;
            if (*fr).longLimit[j as usize][i as usize] >
                   (*fr).down_sample_sblimit {
                (*fr).longLimit[j as usize][i as usize] =
                    (*fr).down_sample_sblimit
            }
            i += 1
        }
        i = 0 as libc::c_int;
        while i < 14 as libc::c_int {
            (*fr).shortLimit[j as usize][i as usize] =
                (bandInfo[j as usize].shortIdx[i as usize] as libc::c_int -
                     1 as libc::c_int) / 18 as libc::c_int + 1 as libc::c_int;
            if (*fr).shortLimit[j as usize][i as usize] >
                   (*fr).down_sample_sblimit {
                (*fr).shortLimit[j as usize][i as usize] =
                    (*fr).down_sample_sblimit
            }
            i += 1
        }
        j += 1
    };
}
// read additional side information (for MPEG 1 and MPEG 2)
unsafe extern "C" fn III_get_side_info(mut fr: *mut mpg123_handle_t,
                                       mut si: *mut III_sideinfo,
                                       mut stereo: libc::c_int,
                                       mut ms_stereo: libc::c_int,
                                       mut sfreq: libc::c_long,
                                       mut single: libc::c_int)
 -> libc::c_int {
    let mut powdiff: libc::c_int =
        if single == 3 as libc::c_int {
            4 as libc::c_int
        } else { 0 as libc::c_int };
    let tabs: [[libc::c_int; 5]; 2] =
        [[2 as libc::c_int, 9 as libc::c_int, 5 as libc::c_int,
          3 as libc::c_int, 4 as libc::c_int],
         [1 as libc::c_int, 8 as libc::c_int, 1 as libc::c_int,
          2 as libc::c_int, 9 as libc::c_int]];
    let mut tab: *const libc::c_int = tabs[(*fr).lsf as usize].as_ptr();
    let mut ch: libc::c_int = 0;
    let mut gr: libc::c_int = 0;
    (*si).main_data_begin =
        getbits(fr, *tab.offset(1 as libc::c_int as isize));
    if (*si).main_data_begin > (*fr).bitreservoir {
        //  overwrite main_data_begin for the floatly available bit reservoir
        (*fr).bitindex -= *tab.offset(1 as libc::c_int as isize);
        (*fr).wordpointer =
            (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int) as
                                         isize);
        (*fr).bitindex &= 0x7 as libc::c_int;
        if (*fr).lsf == 0 as libc::c_int {
            *(*fr).wordpointer.offset(0 as libc::c_int as isize) =
                ((*fr).bitreservoir >> 1 as libc::c_int) as byte;
            *(*fr).wordpointer.offset(1 as libc::c_int as isize) =
                (((*fr).bitreservoir & 1 as libc::c_int as libc::c_uint) <<
                     7 as libc::c_int) as byte
        } else {
            *(*fr).wordpointer.offset(0 as libc::c_int as isize) =
                (*fr).bitreservoir as byte
        }
        // zero "side-info" data for a silence-frame
		// without touching audio data used as bit reservoir for following frame
        memset((*fr).wordpointer.offset(2 as libc::c_int as isize) as
                   *mut libc::c_void, 0 as libc::c_int,
               ((*fr).ssize - 2 as libc::c_int) as libc::c_ulong);
        // reread the new bit reservoir offset
        (*si).main_data_begin =
            getbits(fr, *tab.offset(1 as libc::c_int as isize))
    }
    // keep track of the available data bytes for the bit reservoir.
	// think: Substract the 2 crc bytes in parser already?
    (*fr).bitreservoir =
        (*fr).bitreservoir.wrapping_add((*fr).framesize as
                                            libc::c_uint).wrapping_sub((*fr).ssize
                                                                           as
                                                                           libc::c_uint).wrapping_sub((if (*fr).error_protection
                                                                                                              !=
                                                                                                              0
                                                                                                          {
                                                                                                           2
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                       } else {
                                                                                                           0
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                       })
                                                                                                          as
                                                                                                          libc::c_uint);
    // limit the reservoir to the max for MPEG 1.0 or 2.x.
    if (*fr).bitreservoir >
           (if (*fr).lsf == 0 as libc::c_int {
                511 as libc::c_int
            } else { 255 as libc::c_int }) as uint {
        (*fr).bitreservoir =
            if (*fr).lsf == 0 as libc::c_int {
                511 as libc::c_int
            } else { 255 as libc::c_int } as uint
    }
    // now back into less commented territory. It's code. It works.
    if stereo == 1 as libc::c_int {
        (*fr).ultmp =
            ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                  libc::c_int) << (*fr).bitindex) as byte as ulong;
        (*fr).ultmp |=
            (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as ulong) <<
                (*fr).bitindex >> 8 as libc::c_int;
        (*fr).ultmp <<= *tab.offset(2 as libc::c_int as isize);
        (*fr).ultmp >>= 8 as libc::c_int;
        (*fr).bitindex += *tab.offset(2 as libc::c_int as isize);
        (*fr).wordpointer =
            (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int) as
                                         isize);
        (*fr).bitindex &= 7 as libc::c_int;
        (*si).private_bits = (*fr).ultmp as uint
    } else {
        (*fr).ultmp =
            ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                  libc::c_int) << (*fr).bitindex) as byte as ulong;
        (*fr).ultmp |=
            (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as ulong) <<
                (*fr).bitindex >> 8 as libc::c_int;
        (*fr).ultmp <<= *tab.offset(3 as libc::c_int as isize);
        (*fr).ultmp >>= 8 as libc::c_int;
        (*fr).bitindex += *tab.offset(3 as libc::c_int as isize);
        (*fr).wordpointer =
            (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int) as
                                         isize);
        (*fr).bitindex &= 7 as libc::c_int;
        (*si).private_bits = (*fr).ultmp as uint
    }
    if (*fr).lsf == 0 {
        ch = 0 as libc::c_int;
        while ch < stereo {
            (*si).ch[ch as usize].gr[0 as libc::c_int as usize].scfsi =
                -(1 as libc::c_int);
            (*fr).ultmp =
                ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                      libc::c_int) << (*fr).bitindex) as byte as ulong;
            (*fr).ultmp |=
                (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                     ulong) << (*fr).bitindex >> 8 as libc::c_int;
            (*fr).ultmp <<= 4 as libc::c_int;
            (*fr).ultmp >>= 8 as libc::c_int;
            (*fr).bitindex += 4 as libc::c_int;
            (*fr).wordpointer =
                (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int)
                                             as isize);
            (*fr).bitindex &= 7 as libc::c_int;
            (*si).ch[ch as usize].gr[1 as libc::c_int as usize].scfsi =
                (*fr).ultmp as libc::c_int;
            ch += 1
        }
    }
    gr = 0 as libc::c_int;
    while gr < *tab.offset(0 as libc::c_int as isize) {
        ch = 0 as libc::c_int;
        while ch < stereo {
            let mut gr_info: *mut gr_info_t =
                &mut *(*(*si).ch.as_mut_ptr().offset(ch as
                                                         isize)).gr.as_mut_ptr().offset(gr
                                                                                            as
                                                                                            isize)
                    as *mut gr_info_t;
            (*gr_info).part2_3_length = getbits(fr, 12 as libc::c_int);
            (*gr_info).big_values = getbits(fr, 9 as libc::c_int);
            if (*gr_info).big_values > 288 as libc::c_int as libc::c_uint {
                (*gr_info).big_values = 288 as libc::c_int as uint
            }
            (*fr).ultmp =
                ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                      libc::c_int) << (*fr).bitindex) as byte as ulong;
            (*fr).ultmp |=
                (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                     ulong) << (*fr).bitindex >> 8 as libc::c_int;
            (*fr).ultmp <<= 8 as libc::c_int;
            (*fr).ultmp >>= 8 as libc::c_int;
            (*fr).bitindex += 8 as libc::c_int;
            (*fr).wordpointer =
                (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int)
                                             as isize);
            (*fr).bitindex &= 7 as libc::c_int;
            (*gr_info).pow2gain =
                (*fr).gainpow2.as_mut_ptr().offset(256 as libc::c_int as
                                                       isize).offset(-((*fr).ultmp
                                                                           as
                                                                           isize)).offset(powdiff
                                                                                              as
                                                                                              isize);
            if ms_stereo != 0 {
                (*gr_info).pow2gain =
                    (*gr_info).pow2gain.offset(2 as libc::c_int as isize)
            }
            (*gr_info).scalefac_compress =
                getbits(fr, *tab.offset(4 as libc::c_int as isize));
            (*fr).uctmp =
                ((*(*fr).wordpointer as libc::c_int) << (*fr).bitindex) as
                    byte;
            (*fr).bitindex += 1;
            (*fr).wordpointer =
                (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int)
                                             as isize);
            (*fr).bitindex &= 7 as libc::c_int;
            if (*fr).uctmp as libc::c_int >> 7 as libc::c_int != 0 {
                let mut i: libc::c_int = 0;
                // window switch flag
                (*fr).ultmp =
                    ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                          libc::c_int) << (*fr).bitindex) as byte as ulong;
                (*fr).ultmp |=
                    (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                         ulong) << (*fr).bitindex >> 8 as libc::c_int;
                (*fr).ultmp <<= 2 as libc::c_int;
                (*fr).ultmp >>= 8 as libc::c_int;
                (*fr).bitindex += 2 as libc::c_int;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                (*gr_info).block_type = (*fr).ultmp as uint;
                (*fr).uctmp =
                    ((*(*fr).wordpointer as libc::c_int) << (*fr).bitindex) as
                        byte;
                (*fr).bitindex += 1;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                (*gr_info).mixed_block_flag =
                    ((*fr).uctmp as libc::c_int >> 7 as libc::c_int) as uint;
                (*fr).ultmp =
                    ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                          libc::c_int) << (*fr).bitindex) as byte as ulong;
                (*fr).ultmp |=
                    (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                         ulong) << (*fr).bitindex >> 8 as libc::c_int;
                (*fr).ultmp <<= 5 as libc::c_int;
                (*fr).ultmp >>= 8 as libc::c_int;
                (*fr).bitindex += 5 as libc::c_int;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                (*gr_info).table_select[0 as libc::c_int as usize] =
                    (*fr).ultmp as uint;
                (*fr).ultmp =
                    ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                          libc::c_int) << (*fr).bitindex) as byte as ulong;
                (*fr).ultmp |=
                    (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                         ulong) << (*fr).bitindex >> 8 as libc::c_int;
                (*fr).ultmp <<= 5 as libc::c_int;
                (*fr).ultmp >>= 8 as libc::c_int;
                (*fr).bitindex += 5 as libc::c_int;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                (*gr_info).table_select[1 as libc::c_int as usize] =
                    (*fr).ultmp as uint;
                // table_select[2] not needed, because there is no region2,
				// but to satisfy some verification tools we set it either.
                (*gr_info).table_select[2 as libc::c_int as usize] =
                    0 as libc::c_int as uint;
                i = 0 as libc::c_int;
                while i < 3 as libc::c_int {
                    (*fr).ultmp =
                        ((*(*fr).wordpointer.offset(0 as libc::c_int as isize)
                              as libc::c_int) << (*fr).bitindex) as byte as
                            ulong;
                    (*fr).ultmp |=
                        (*(*fr).wordpointer.offset(1 as libc::c_int as isize)
                             as ulong) << (*fr).bitindex >> 8 as libc::c_int;
                    (*fr).ultmp <<= 3 as libc::c_int;
                    (*fr).ultmp >>= 8 as libc::c_int;
                    (*fr).bitindex += 3 as libc::c_int;
                    (*fr).wordpointer =
                        (*fr).wordpointer.offset(((*fr).bitindex >>
                                                      3 as libc::c_int) as
                                                     isize);
                    (*fr).bitindex &= 7 as libc::c_int;
                    (*gr_info).full_gain[i as usize] =
                        (*gr_info).pow2gain.offset(((*fr).ultmp <<
                                                        3 as libc::c_int) as
                                                       isize);
                    i += 1
                }
                if (*gr_info).block_type == 0 as libc::c_int as libc::c_uint {
                    return 1 as libc::c_int
                }
                // region_count/start parameters are implicit in this case.
                if ((*fr).lsf == 0 ||
                        (*gr_info).block_type ==
                            2 as libc::c_int as libc::c_uint) &&
                       (*fr).mpeg25 == 0 {
                    (*gr_info).region1start =
                        (36 as libc::c_int >> 1 as libc::c_int) as uint;
                    (*gr_info).region2start =
                        (576 as libc::c_int >> 1 as libc::c_int) as uint
                } else if (*fr).mpeg25 != 0 {
                    let mut r0c: libc::c_int = 0;
                    let mut r1c: libc::c_int = 0;
                    if (*gr_info).block_type ==
                           2 as libc::c_int as libc::c_uint &&
                           (*gr_info).mixed_block_flag == 0 {
                        r0c = 5 as libc::c_int
                    } else { r0c = 7 as libc::c_int }
                    // r0c + 1 + r1c + 1 == 22, always.
                    r1c = 20 as libc::c_int - r0c; // 0 .. 15
                    (*gr_info).region1start =
                        (bandInfo[sfreq as
                                      usize].longIdx[(r0c + 1 as libc::c_int)
                                                         as usize] as
                             libc::c_int >> 1 as libc::c_int) as
                            uint; // 0 .. 7
                    (*gr_info).region2start =
                        (bandInfo[sfreq as
                                      usize].longIdx[(r0c + 1 as libc::c_int +
                                                          r1c +
                                                          1 as libc::c_int) as
                                                         usize] as libc::c_int
                             >> 1 as libc::c_int) as uint
                } else {
                    (*gr_info).region1start =
                        (54 as libc::c_int >> 1 as libc::c_int) as uint;
                    (*gr_info).region2start =
                        (576 as libc::c_int >> 1 as libc::c_int) as uint
                }
            } else {
                let mut i_0: libc::c_int = 0;
                let mut r0c_0: libc::c_int = 0;
                let mut r1c_0: libc::c_int = 0;
                i_0 = 0 as libc::c_int;
                while i_0 < 3 as libc::c_int {
                    (*fr).ultmp =
                        ((*(*fr).wordpointer.offset(0 as libc::c_int as isize)
                              as libc::c_int) << (*fr).bitindex) as byte as
                            ulong;
                    (*fr).ultmp |=
                        (*(*fr).wordpointer.offset(1 as libc::c_int as isize)
                             as ulong) << (*fr).bitindex >> 8 as libc::c_int;
                    (*fr).ultmp <<= 5 as libc::c_int;
                    (*fr).ultmp >>= 8 as libc::c_int;
                    (*fr).bitindex += 5 as libc::c_int;
                    (*fr).wordpointer =
                        (*fr).wordpointer.offset(((*fr).bitindex >>
                                                      3 as libc::c_int) as
                                                     isize);
                    (*fr).bitindex &= 7 as libc::c_int;
                    (*gr_info).table_select[i_0 as usize] =
                        (*fr).ultmp as uint;
                    i_0 += 1
                }
                (*fr).ultmp =
                    ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                          libc::c_int) << (*fr).bitindex) as byte as ulong;
                (*fr).ultmp |=
                    (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                         ulong) << (*fr).bitindex >> 8 as libc::c_int;
                (*fr).ultmp <<= 4 as libc::c_int;
                (*fr).ultmp >>= 8 as libc::c_int;
                (*fr).bitindex += 4 as libc::c_int;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                r0c_0 = (*fr).ultmp as libc::c_int;
                (*fr).ultmp =
                    ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                          libc::c_int) << (*fr).bitindex) as byte as ulong;
                (*fr).ultmp |=
                    (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                         ulong) << (*fr).bitindex >> 8 as libc::c_int;
                (*fr).ultmp <<= 3 as libc::c_int;
                (*fr).ultmp >>= 8 as libc::c_int;
                (*fr).bitindex += 3 as libc::c_int;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                r1c_0 = (*fr).ultmp as libc::c_int;
                (*gr_info).region1start =
                    (bandInfo[sfreq as
                                  usize].longIdx[(r0c_0 + 1 as libc::c_int) as
                                                     usize] as libc::c_int >>
                         1 as libc::c_int) as uint;
                // max( r0c + r1c + 2 ) = 15 + 7 + 2 = 24
                if r0c_0 + 1 as libc::c_int + r1c_0 + 1 as libc::c_int >
                       22 as libc::c_int {
                    (*gr_info).region2start =
                        (576 as libc::c_int >> 1 as libc::c_int) as uint
                } else {
                    (*gr_info).region2start =
                        (bandInfo[sfreq as
                                      usize].longIdx[(r0c_0 + 1 as libc::c_int
                                                          + r1c_0 +
                                                          1 as libc::c_int) as
                                                         usize] as libc::c_int
                             >> 1 as libc::c_int) as uint
                }
                (*gr_info).block_type = 0 as libc::c_int as uint;
                (*gr_info).mixed_block_flag = 0 as libc::c_int as uint
            }
            if (*fr).lsf == 0 {
                (*fr).uctmp =
                    ((*(*fr).wordpointer as libc::c_int) << (*fr).bitindex) as
                        byte;
                (*fr).bitindex += 1;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                (*gr_info).preflag =
                    ((*fr).uctmp as libc::c_int >> 7 as libc::c_int) as uint
            }
            (*fr).uctmp =
                ((*(*fr).wordpointer as libc::c_int) << (*fr).bitindex) as
                    byte;
            (*fr).bitindex += 1;
            (*fr).wordpointer =
                (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int)
                                             as isize);
            (*fr).bitindex &= 7 as libc::c_int;
            (*gr_info).scalefac_scale =
                ((*fr).uctmp as libc::c_int >> 7 as libc::c_int) as uint;
            (*fr).uctmp =
                ((*(*fr).wordpointer as libc::c_int) << (*fr).bitindex) as
                    byte;
            (*fr).bitindex += 1;
            (*fr).wordpointer =
                (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int)
                                             as isize);
            (*fr).bitindex &= 7 as libc::c_int;
            (*gr_info).count1table_select =
                ((*fr).uctmp as libc::c_int >> 7 as libc::c_int) as uint;
            ch += 1
        }
        gr += 1
    }
    return 0 as libc::c_int;
}
// read scalefactors
unsafe extern "C" fn III_get_scale_factors_1(mut fr: *mut mpg123_handle_t,
                                             mut scf: *mut libc::c_int,
                                             mut gr_info: *mut gr_info_t)
 -> libc::c_int {
    let slen: [[byte; 16]; 2] =
        [[0 as libc::c_int as byte, 0 as libc::c_int as byte,
          0 as libc::c_int as byte, 0 as libc::c_int as byte,
          3 as libc::c_int as byte, 1 as libc::c_int as byte,
          1 as libc::c_int as byte, 1 as libc::c_int as byte,
          2 as libc::c_int as byte, 2 as libc::c_int as byte,
          2 as libc::c_int as byte, 3 as libc::c_int as byte,
          3 as libc::c_int as byte, 3 as libc::c_int as byte,
          4 as libc::c_int as byte, 4 as libc::c_int as byte],
         [0 as libc::c_int as byte, 1 as libc::c_int as byte,
          2 as libc::c_int as byte, 3 as libc::c_int as byte,
          0 as libc::c_int as byte, 1 as libc::c_int as byte,
          2 as libc::c_int as byte, 3 as libc::c_int as byte,
          1 as libc::c_int as byte, 2 as libc::c_int as byte,
          3 as libc::c_int as byte, 1 as libc::c_int as byte,
          2 as libc::c_int as byte, 3 as libc::c_int as byte,
          2 as libc::c_int as byte, 3 as libc::c_int as byte]];
    let mut num0: libc::c_int =
        slen[0 as libc::c_int as usize][(*gr_info).scalefac_compress as usize]
            as libc::c_int;
    let mut num1: libc::c_int =
        slen[1 as libc::c_int as usize][(*gr_info).scalefac_compress as usize]
            as libc::c_int;
    let mut numbits: libc::c_int = 0;
    if (*gr_info).block_type == 2 as libc::c_int as libc::c_uint {
        let mut i: libc::c_int = 18 as libc::c_int;
        numbits = (num0 + num1) * 18 as libc::c_int;
        if (*gr_info).mixed_block_flag != 0 {
            i = 8 as libc::c_int;
            while i != 0 {
                (*fr).ultmp =
                    ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                          libc::c_int) << (*fr).bitindex) as byte as ulong;
                (*fr).ultmp |=
                    (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                         ulong) << (*fr).bitindex >> 8 as libc::c_int;
                (*fr).ultmp <<= num0;
                (*fr).ultmp >>= 8 as libc::c_int;
                (*fr).bitindex += num0;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                let fresh18 = scf;
                scf = scf.offset(1);
                *fresh18 = (*fr).ultmp as libc::c_int;
                i -= 1
            }
            i = 9 as libc::c_int;
            numbits -= num0
            // num0 * 17 + num1 * 18
        }
        while i != 0 {
            (*fr).ultmp =
                ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                      libc::c_int) << (*fr).bitindex) as byte as ulong;
            (*fr).ultmp |=
                (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                     ulong) << (*fr).bitindex >> 8 as libc::c_int;
            (*fr).ultmp <<= num0;
            (*fr).ultmp >>= 8 as libc::c_int;
            (*fr).bitindex += num0;
            (*fr).wordpointer =
                (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int)
                                             as isize);
            (*fr).bitindex &= 7 as libc::c_int;
            let fresh19 = scf;
            scf = scf.offset(1);
            *fresh19 = (*fr).ultmp as libc::c_int;
            i -= 1
        }
        i = 18 as libc::c_int;
        while i != 0 {
            (*fr).ultmp =
                ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                      libc::c_int) << (*fr).bitindex) as byte as ulong;
            (*fr).ultmp |=
                (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                     ulong) << (*fr).bitindex >> 8 as libc::c_int;
            (*fr).ultmp <<= num1;
            (*fr).ultmp >>= 8 as libc::c_int;
            (*fr).bitindex += num1;
            (*fr).wordpointer =
                (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int)
                                             as isize);
            (*fr).bitindex &= 7 as libc::c_int;
            let fresh20 = scf;
            scf = scf.offset(1);
            *fresh20 = (*fr).ultmp as libc::c_int;
            i -= 1
        }
        // short[13][0..2] = 0
        let fresh21 = scf;
        scf = scf.offset(1);
        *fresh21 = 0 as libc::c_int;
        let fresh22 = scf;
        scf = scf.offset(1);
        *fresh22 = 0 as libc::c_int;
        let fresh23 = scf;
        scf = scf.offset(1);
        *fresh23 = 0 as libc::c_int
    } else {
        let mut i_0: libc::c_int = 0;
        let mut scfsi: libc::c_int = (*gr_info).scfsi;
        if scfsi < 0 as libc::c_int {
            // scfsi < 0 => granule == 0
            i_0 = 11 as libc::c_int;
            while i_0 != 0 {
                (*fr).ultmp =
                    ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                          libc::c_int) << (*fr).bitindex) as byte as ulong;
                (*fr).ultmp |=
                    (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                         ulong) << (*fr).bitindex >> 8 as libc::c_int;
                (*fr).ultmp <<= num0;
                (*fr).ultmp >>= 8 as libc::c_int;
                (*fr).bitindex += num0;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                let fresh24 = scf;
                scf = scf.offset(1);
                *fresh24 = (*fr).ultmp as libc::c_int;
                i_0 -= 1
            }
            i_0 = 10 as libc::c_int;
            while i_0 != 0 {
                (*fr).ultmp =
                    ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                          libc::c_int) << (*fr).bitindex) as byte as ulong;
                (*fr).ultmp |=
                    (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                         ulong) << (*fr).bitindex >> 8 as libc::c_int;
                (*fr).ultmp <<= num1;
                (*fr).ultmp >>= 8 as libc::c_int;
                (*fr).bitindex += num1;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                let fresh25 = scf;
                scf = scf.offset(1);
                *fresh25 = (*fr).ultmp as libc::c_int;
                i_0 -= 1
            }
            numbits = (num0 + num1) * 10 as libc::c_int + num0;
            let fresh26 = scf;
            scf = scf.offset(1);
            *fresh26 = 0 as libc::c_int
        } else {
            numbits = 0 as libc::c_int;
            if scfsi & 0x8 as libc::c_int == 0 {
                i_0 = 0 as libc::c_int;
                while i_0 < 6 as libc::c_int {
                    (*fr).ultmp =
                        ((*(*fr).wordpointer.offset(0 as libc::c_int as isize)
                              as libc::c_int) << (*fr).bitindex) as byte as
                            ulong;
                    (*fr).ultmp |=
                        (*(*fr).wordpointer.offset(1 as libc::c_int as isize)
                             as ulong) << (*fr).bitindex >> 8 as libc::c_int;
                    (*fr).ultmp <<= num0;
                    (*fr).ultmp >>= 8 as libc::c_int;
                    (*fr).bitindex += num0;
                    (*fr).wordpointer =
                        (*fr).wordpointer.offset(((*fr).bitindex >>
                                                      3 as libc::c_int) as
                                                     isize);
                    (*fr).bitindex &= 7 as libc::c_int;
                    let fresh27 = scf;
                    scf = scf.offset(1);
                    *fresh27 = (*fr).ultmp as libc::c_int;
                    i_0 += 1
                }
                numbits += num0 * 6 as libc::c_int
            } else { scf = scf.offset(6 as libc::c_int as isize) }
            if scfsi & 0x4 as libc::c_int == 0 {
                i_0 = 0 as libc::c_int;
                while i_0 < 5 as libc::c_int {
                    (*fr).ultmp =
                        ((*(*fr).wordpointer.offset(0 as libc::c_int as isize)
                              as libc::c_int) << (*fr).bitindex) as byte as
                            ulong;
                    (*fr).ultmp |=
                        (*(*fr).wordpointer.offset(1 as libc::c_int as isize)
                             as ulong) << (*fr).bitindex >> 8 as libc::c_int;
                    (*fr).ultmp <<= num0;
                    (*fr).ultmp >>= 8 as libc::c_int;
                    (*fr).bitindex += num0;
                    (*fr).wordpointer =
                        (*fr).wordpointer.offset(((*fr).bitindex >>
                                                      3 as libc::c_int) as
                                                     isize);
                    (*fr).bitindex &= 7 as libc::c_int;
                    let fresh28 = scf;
                    scf = scf.offset(1);
                    *fresh28 = (*fr).ultmp as libc::c_int;
                    i_0 += 1
                }
                numbits += num0 * 5 as libc::c_int
            } else { scf = scf.offset(5 as libc::c_int as isize) }
            if scfsi & 0x2 as libc::c_int == 0 {
                i_0 = 0 as libc::c_int;
                while i_0 < 5 as libc::c_int {
                    (*fr).ultmp =
                        ((*(*fr).wordpointer.offset(0 as libc::c_int as isize)
                              as libc::c_int) << (*fr).bitindex) as byte as
                            ulong;
                    (*fr).ultmp |=
                        (*(*fr).wordpointer.offset(1 as libc::c_int as isize)
                             as ulong) << (*fr).bitindex >> 8 as libc::c_int;
                    (*fr).ultmp <<= num1;
                    (*fr).ultmp >>= 8 as libc::c_int;
                    (*fr).bitindex += num1;
                    (*fr).wordpointer =
                        (*fr).wordpointer.offset(((*fr).bitindex >>
                                                      3 as libc::c_int) as
                                                     isize);
                    (*fr).bitindex &= 7 as libc::c_int;
                    let fresh29 = scf;
                    scf = scf.offset(1);
                    *fresh29 = (*fr).ultmp as libc::c_int;
                    i_0 += 1
                }
                numbits += num1 * 5 as libc::c_int
            } else { scf = scf.offset(5 as libc::c_int as isize) }
            if scfsi & 0x1 as libc::c_int == 0 {
                i_0 = 0 as libc::c_int;
                while i_0 < 5 as libc::c_int {
                    (*fr).ultmp =
                        ((*(*fr).wordpointer.offset(0 as libc::c_int as isize)
                              as libc::c_int) << (*fr).bitindex) as byte as
                            ulong;
                    (*fr).ultmp |=
                        (*(*fr).wordpointer.offset(1 as libc::c_int as isize)
                             as ulong) << (*fr).bitindex >> 8 as libc::c_int;
                    (*fr).ultmp <<= num1;
                    (*fr).ultmp >>= 8 as libc::c_int;
                    (*fr).bitindex += num1;
                    (*fr).wordpointer =
                        (*fr).wordpointer.offset(((*fr).bitindex >>
                                                      3 as libc::c_int) as
                                                     isize);
                    (*fr).bitindex &= 7 as libc::c_int;
                    let fresh30 = scf;
                    scf = scf.offset(1);
                    *fresh30 = (*fr).ultmp as libc::c_int;
                    i_0 += 1
                }
                numbits += num1 * 5 as libc::c_int
            } else { scf = scf.offset(5 as libc::c_int as isize) }
            // no l[21] in original sources
            let fresh31 = scf;
            scf = scf.offset(1);
            *fresh31 = 0 as libc::c_int
        }
    }
    return numbits;
}
unsafe extern "C" fn III_get_scale_factors_2(mut fr: *mut mpg123_handle_t,
                                             mut scf: *mut libc::c_int,
                                             mut gr_info: *mut gr_info_t,
                                             mut i_stereo: libc::c_int)
 -> libc::c_int {
    let mut pnt: *const byte = 0 as *const byte;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut numbits: libc::c_int = 0 as libc::c_int;
    let mut slen: uint = 0;
    let stab: [[[byte; 4]; 6]; 3] =
        [[[6 as libc::c_int as byte, 5 as libc::c_int as byte,
           5 as libc::c_int as byte, 5 as libc::c_int as byte],
          [6 as libc::c_int as byte, 5 as libc::c_int as byte,
           7 as libc::c_int as byte, 3 as libc::c_int as byte],
          [11 as libc::c_int as byte, 10 as libc::c_int as byte,
           0 as libc::c_int as byte, 0 as libc::c_int as byte],
          [7 as libc::c_int as byte, 7 as libc::c_int as byte,
           7 as libc::c_int as byte, 0 as libc::c_int as byte],
          [6 as libc::c_int as byte, 6 as libc::c_int as byte,
           6 as libc::c_int as byte, 3 as libc::c_int as byte],
          [8 as libc::c_int as byte, 8 as libc::c_int as byte,
           5 as libc::c_int as byte, 0 as libc::c_int as byte]],
         [[9 as libc::c_int as byte, 9 as libc::c_int as byte,
           9 as libc::c_int as byte, 9 as libc::c_int as byte],
          [9 as libc::c_int as byte, 9 as libc::c_int as byte,
           12 as libc::c_int as byte, 6 as libc::c_int as byte],
          [18 as libc::c_int as byte, 18 as libc::c_int as byte,
           0 as libc::c_int as byte, 0 as libc::c_int as byte],
          [12 as libc::c_int as byte, 12 as libc::c_int as byte,
           12 as libc::c_int as byte, 0 as libc::c_int as byte],
          [12 as libc::c_int as byte, 9 as libc::c_int as byte,
           9 as libc::c_int as byte, 6 as libc::c_int as byte],
          [15 as libc::c_int as byte, 12 as libc::c_int as byte,
           9 as libc::c_int as byte, 0 as libc::c_int as byte]],
         [[6 as libc::c_int as byte, 9 as libc::c_int as byte,
           9 as libc::c_int as byte, 9 as libc::c_int as byte],
          [6 as libc::c_int as byte, 9 as libc::c_int as byte,
           12 as libc::c_int as byte, 6 as libc::c_int as byte],
          [15 as libc::c_int as byte, 18 as libc::c_int as byte,
           0 as libc::c_int as byte, 0 as libc::c_int as byte],
          [6 as libc::c_int as byte, 15 as libc::c_int as byte,
           12 as libc::c_int as byte, 0 as libc::c_int as byte],
          [6 as libc::c_int as byte, 12 as libc::c_int as byte,
           9 as libc::c_int as byte, 6 as libc::c_int as byte],
          [6 as libc::c_int as byte, 18 as libc::c_int as byte,
           9 as libc::c_int as byte, 0 as libc::c_int as byte]]];
    // i_stereo AND second channel -> do_layer3() checks this
    if i_stereo != 0 {
        slen =
            i_slen2[((*gr_info).scalefac_compress >> 1 as libc::c_int) as
                        usize]
    } else { slen = n_slen2[(*gr_info).scalefac_compress as usize] }
    (*gr_info).preflag =
        slen >> 15 as libc::c_int & 0x1 as libc::c_int as libc::c_uint;
    n = 0 as libc::c_int;
    if (*gr_info).block_type == 2 as libc::c_int as libc::c_uint {
        if (*gr_info).mixed_block_flag != 0 { n += 1 }
        n += 1
    }
    pnt =
        stab[n as
                 usize][(slen >> 12 as libc::c_int &
                             0x7 as libc::c_int as libc::c_uint) as
                            usize].as_ptr();
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut num: libc::c_int =
            (slen & 0x7 as libc::c_int as libc::c_uint) as libc::c_int;
        slen >>= 3 as libc::c_int;
        if num != 0 {
            j = 0 as libc::c_int;
            while j < *pnt.offset(i as isize) as libc::c_int {
                (*fr).ultmp =
                    ((*(*fr).wordpointer.offset(0 as libc::c_int as isize) as
                          libc::c_int) << (*fr).bitindex) as byte as ulong;
                (*fr).ultmp |=
                    (*(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                         ulong) << (*fr).bitindex >> 8 as libc::c_int;
                (*fr).ultmp <<= num;
                (*fr).ultmp >>= 8 as libc::c_int;
                (*fr).bitindex += num;
                (*fr).wordpointer =
                    (*fr).wordpointer.offset(((*fr).bitindex >>
                                                  3 as libc::c_int) as isize);
                (*fr).bitindex &= 7 as libc::c_int;
                let fresh32 = scf;
                scf = scf.offset(1);
                *fresh32 = (*fr).ultmp as libc::c_int;
                j += 1
            }
            numbits += *pnt.offset(i as isize) as libc::c_int * num
        } else {
            j = 0 as libc::c_int;
            while j < *pnt.offset(i as isize) as libc::c_int {
                let fresh33 = scf;
                scf = scf.offset(1);
                *fresh33 = 0 as libc::c_int;
                j += 1
            }
        }
        i += 1
    }
    n = (n << 1 as libc::c_int) + 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let fresh34 = scf;
        scf = scf.offset(1);
        *fresh34 = 0 as libc::c_int;
        i += 1
    }
    return numbits;
}
unsafe extern "C" fn III_dequantize_sample(mut fr: *mut mpg123_handle_t,
                                           mut xr: *mut [libc::c_float; 18],
                                           mut scf: *mut libc::c_int,
                                           mut gr_info: *mut gr_info_t,
                                           mut sfreq: libc::c_int,
                                           mut part2bits: libc::c_int)
 -> libc::c_int {
    let mut shift: libc::c_int =
        (1 as libc::c_int as
             libc::c_uint).wrapping_add((*gr_info).scalefac_scale) as
            libc::c_int;
    let mut part2remain: libc::c_int =
        (*gr_info).part2_3_length.wrapping_sub(part2bits as libc::c_uint) as
            libc::c_int;
    let mut region1: libc::c_int = (*gr_info).region1start as libc::c_int;
    let mut region2: libc::c_int = (*gr_info).region2start as libc::c_int;
    let mut bv: libc::c_int = (*gr_info).big_values as libc::c_int;
    let mut num: libc::c_int = -(*fr).bitindex & 0x7 as libc::c_int;
    let mut xrpnt: *mut libc::c_float = xr as *mut libc::c_float;
    let mut l: [libc::c_int; 3] = [0; 3];
    let mut l3: libc::c_int = 0;
    let mut mask: uint32_t = 0;
    let mut me: *mut libc::c_int = 0 as *mut libc::c_int;
    // we must split this, because for num == 0 the shift is undefined if you do it in one step.
    mask =
        getbits(fr, num) <<
            (::std::mem::size_of::<uint32_t>() as
                 libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong).wrapping_mul(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong);
    mask <<= 8 as libc::c_int - num;
    part2remain -= num;
    l3 = (576 as libc::c_int >> 1 as libc::c_int) - bv >> 1 as libc::c_int;
    // we may lose the 'odd' bit here !! check this later again
    if bv <= region1 {
        l[0 as libc::c_int as usize] = bv;
        l[1 as libc::c_int as usize] = 0 as libc::c_int;
        l[2 as libc::c_int as usize] = 0 as libc::c_int
    } else {
        l[0 as libc::c_int as usize] = region1;
        if bv <= region2 {
            l[1 as libc::c_int as usize] = bv - l[0 as libc::c_int as usize];
            l[2 as libc::c_int as usize] = 0 as libc::c_int
        } else {
            l[1 as libc::c_int as usize] =
                region2 - l[0 as libc::c_int as usize];
            l[2 as libc::c_int as usize] = bv - region2
        }
    }
    if (*gr_info).block_type == 2 as libc::c_int as libc::c_uint {
        let mut i: libc::c_int = 0;
        let mut max: [libc::c_int; 4] = [0; 4];
        let mut step: libc::c_int = 0 as libc::c_int;
        let mut lwin: libc::c_int = 3 as libc::c_int;
        let mut v: libc::c_float = 0.0f32;
        let mut cb: libc::c_int = 0 as libc::c_int;
        let mut m: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut mc: libc::c_int = 0;
        let mut rmax: libc::c_int = 0;
        // decoding with short or mixed mode BandIndex table
        if (*gr_info).mixed_block_flag != 0 {
            max[3 as libc::c_int as usize] = -(1 as libc::c_int);
            max[2 as libc::c_int as usize] = 2 as libc::c_int;
            max[1 as libc::c_int as usize] = max[2 as libc::c_int as usize];
            max[0 as libc::c_int as usize] = max[1 as libc::c_int as usize];
            m = map[sfreq as usize][0 as libc::c_int as usize];
            me = mapend[sfreq as usize][0 as libc::c_int as usize]
        } else {
            max[3 as libc::c_int as usize] = -(1 as libc::c_int);
            max[2 as libc::c_int as usize] = max[3 as libc::c_int as usize];
            max[1 as libc::c_int as usize] = max[2 as libc::c_int as usize];
            max[0 as libc::c_int as usize] = max[1 as libc::c_int as usize];
            // max[3] not floatly needed in this case
            m = map[sfreq as usize][1 as libc::c_int as usize];
            me = mapend[sfreq as usize][1 as libc::c_int as usize]
        }
        mc = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            let mut h: *const newhuff =
                ht.as_ptr().offset((*gr_info).table_select[i as usize] as
                                       isize);
            let mut lp: libc::c_int = l[i as usize];
            while lp != 0 {
                let mut x: int32_t = 0;
                let mut y: int32_t = 0;
                if mc == 0 {
                    let fresh35 = m;
                    m = m.offset(1);
                    mc = *fresh35;
                    let fresh36 = m;
                    m = m.offset(1);
                    xrpnt =
                        (xr as *mut libc::c_float).offset(*fresh36 as isize);
                    let fresh37 = m;
                    m = m.offset(1);
                    lwin = *fresh37;
                    let fresh38 = m;
                    m = m.offset(1);
                    cb = *fresh38;
                    if lwin == 3 as libc::c_int {
                        let fresh39 = scf;
                        scf = scf.offset(1);
                        v =
                            *(*gr_info).pow2gain.offset((*fresh39 << shift) as
                                                            isize);
                        step = 1 as libc::c_int
                    } else {
                        let fresh40 = scf;
                        scf = scf.offset(1);
                        v =
                            *(*gr_info).full_gain[lwin as
                                                      usize].offset((*fresh40
                                                                         <<
                                                                         shift)
                                                                        as
                                                                        isize);
                        step = 3 as libc::c_int
                    }
                }
                let mut val: *const libc::c_short = (*h).table;
                while (num as libc::c_ulong) <
                          (::std::mem::size_of::<uint32_t>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong).wrapping_mul(8
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                      {
                    let fresh41 = (*fr).wordpointer;
                    (*fr).wordpointer = (*fr).wordpointer.offset(1);
                    mask |=
                        (*fresh41 as uint32_t) <<
                            (::std::mem::size_of::<uint32_t>() as
                                 libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_sub(num
                                                                                                                                 as
                                                                                                                                 libc::c_ulong);
                    num += 8 as libc::c_int;
                    part2remain -= 8 as libc::c_int
                }
                loop  {
                    let fresh42 = val;
                    val = val.offset(1);
                    y = *fresh42 as int32_t;
                    if !(y < 0 as libc::c_int) { break ; }
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        val = val.offset(-(y as isize))
                    }
                    num -= 1;
                    mask <<= 1 as libc::c_int
                }
                x = y >> 4 as libc::c_int;
                y &= 0xf as libc::c_int;
                if x == 15 as libc::c_int && (*h).linbits != 0 {
                    max[lwin as usize] = cb;
                    while (num as libc::c_ulong) <
                              (::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                          {
                        let fresh43 = (*fr).wordpointer;
                        (*fr).wordpointer = (*fr).wordpointer.offset(1);
                        mask |=
                            (*fresh43 as uint32_t) <<
                                (::std::mem::size_of::<uint32_t>() as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(num
                                                                                                                                     as
                                                                                                                                     libc::c_ulong);
                        num += 8 as libc::c_int;
                        part2remain -= 8 as libc::c_int
                    }
                    x =
                        (x as
                             libc::c_uint).wrapping_add(mask >>
                                                            (::std::mem::size_of::<uint32_t>()
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_ulong).wrapping_add(8
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_ulong).wrapping_sub((*h).linbits
                                                                                                                                                                                                 as
                                                                                                                                                                                                 libc::c_ulong))
                            as int32_t as int32_t;
                    num =
                        (num as
                             libc::c_uint).wrapping_sub((*h).linbits.wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint))
                            as libc::c_int as libc::c_int;
                    mask <<= (*h).linbits;
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        *xrpnt = -ispow[x as usize] * v
                    } else { *xrpnt = ispow[x as usize] * v }
                    mask <<= 1 as libc::c_int
                } else if x != 0 {
                    max[lwin as usize] = cb;
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        *xrpnt = -ispow[x as usize] * v
                    } else { *xrpnt = ispow[x as usize] * v }
                    num -= 1;
                    mask <<= 1 as libc::c_int
                } else { *xrpnt = 0.0f64 as libc::c_float }
                xrpnt = xrpnt.offset(step as isize);
                if y == 15 as libc::c_int && (*h).linbits != 0 {
                    max[lwin as usize] = cb;
                    while (num as libc::c_ulong) <
                              (::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                          {
                        let fresh44 = (*fr).wordpointer;
                        (*fr).wordpointer = (*fr).wordpointer.offset(1);
                        mask |=
                            (*fresh44 as uint32_t) <<
                                (::std::mem::size_of::<uint32_t>() as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(num
                                                                                                                                     as
                                                                                                                                     libc::c_ulong);
                        num += 8 as libc::c_int;
                        part2remain -= 8 as libc::c_int
                    }
                    y =
                        (y as
                             libc::c_uint).wrapping_add(mask >>
                                                            (::std::mem::size_of::<uint32_t>()
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_ulong).wrapping_add(8
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_ulong).wrapping_sub((*h).linbits
                                                                                                                                                                                                 as
                                                                                                                                                                                                 libc::c_ulong))
                            as int32_t as int32_t;
                    num =
                        (num as
                             libc::c_uint).wrapping_sub((*h).linbits.wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint))
                            as libc::c_int as libc::c_int;
                    mask <<= (*h).linbits;
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        *xrpnt = -ispow[y as usize] * v
                    } else { *xrpnt = ispow[y as usize] * v }
                    mask <<= 1 as libc::c_int
                } else if y != 0 {
                    max[lwin as usize] = cb;
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        *xrpnt = -ispow[y as usize] * v
                    } else { *xrpnt = ispow[y as usize] * v }
                    num -= 1;
                    mask <<= 1 as libc::c_int
                } else { *xrpnt = 0.0f64 as libc::c_float }
                xrpnt = xrpnt.offset(step as isize);
                lp -= 1;
                mc -= 1
            }
            i += 1
        }
        while l3 != 0 && part2remain + num > 0 as libc::c_int {
            let mut h_0: *const newhuff = 0 as *const newhuff;
            let mut val_0: *const libc::c_short = 0 as *const libc::c_short;
            let mut a: libc::c_short = 0;
            // this is only a humble hack to prevent a special segfault.
			// more insight into the float workings is still needed.
			// especially why there are (valid?) files that make xrpnt exceed the array with 4 bytes without segfaulting
			// more seems to be floatly bad, though.
            if !(xrpnt <
                     (&mut *(*xr.offset(32 as libc::c_int as
                                            isize)).as_mut_ptr().offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                          as
                          *mut libc::c_float).offset(5 as libc::c_int as
                                                         isize)) {
                return 2 as libc::c_int
            }
            h_0 = htc.as_ptr().offset((*gr_info).count1table_select as isize);
            val_0 = (*h_0).table;
            while (num as libc::c_ulong) <
                      (::std::mem::size_of::<uint32_t>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong).wrapping_mul(8
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong)
                  {
                let fresh45 = (*fr).wordpointer;
                (*fr).wordpointer = (*fr).wordpointer.offset(1);
                mask |=
                    (*fresh45 as uint32_t) <<
                        (::std::mem::size_of::<uint32_t>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong).wrapping_sub(num
                                                                                                                             as
                                                                                                                             libc::c_ulong);
                num += 8 as libc::c_int;
                part2remain -= 8 as libc::c_int
            }
            loop  {
                let fresh46 = val_0;
                val_0 = val_0.offset(1);
                a = *fresh46;
                if !((a as libc::c_int) < 0 as libc::c_int) { break ; }
                if mask &
                       (1 as libc::c_int as uint32_t) <<
                           (::std::mem::size_of::<uint32_t>() as
                                libc::c_ulong).wrapping_mul(8 as libc::c_int
                                                                as
                                                                libc::c_ulong).wrapping_sub(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong)
                       != 0 {
                    val_0 = val_0.offset(-(a as libc::c_int as isize))
                }
                num -= 1;
                mask <<= 1 as libc::c_int
            }
            if part2remain + num <= 0 as libc::c_int {
                num -= part2remain + num;
                break ;
            } else {
                i = 0 as libc::c_int;
                while i < 4 as libc::c_int {
                    if i & 1 as libc::c_int == 0 {
                        if mc == 0 {
                            let fresh47 = m;
                            m = m.offset(1);
                            mc = *fresh47;
                            let fresh48 = m;
                            m = m.offset(1);
                            xrpnt =
                                (xr as
                                     *mut libc::c_float).offset(*fresh48 as
                                                                    isize);
                            let fresh49 = m;
                            m = m.offset(1);
                            lwin = *fresh49;
                            let fresh50 = m;
                            m = m.offset(1);
                            cb = *fresh50;
                            if lwin == 3 as libc::c_int {
                                let fresh51 = scf;
                                scf = scf.offset(1);
                                v =
                                    *(*gr_info).pow2gain.offset((*fresh51 <<
                                                                     shift) as
                                                                    isize);
                                step = 1 as libc::c_int
                            } else {
                                let fresh52 = scf;
                                scf = scf.offset(1);
                                v =
                                    *(*gr_info).full_gain[lwin as
                                                              usize].offset((*fresh52
                                                                                 <<
                                                                                 shift)
                                                                                as
                                                                                isize);
                                step = 3 as libc::c_int
                            }
                        }
                        mc -= 1
                    }
                    if a as libc::c_int & 0x8 as libc::c_int >> i != 0 {
                        max[lwin as usize] = cb;
                        if part2remain + num <= 0 as libc::c_int { break ; }
                        if mask &
                               (1 as libc::c_int as uint32_t) <<
                                   (::std::mem::size_of::<uint32_t>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong).wrapping_sub(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong)
                               != 0 {
                            *xrpnt = -v
                        } else { *xrpnt = v }
                        num -= 1;
                        mask <<= 1 as libc::c_int
                    } else { *xrpnt = 0.0f64 as libc::c_float }
                    xrpnt = xrpnt.offset(step as isize);
                    i += 1
                }
                l3 -= 1
            }
        }
        if lwin < 3 as libc::c_int {
            loop 
                 // short band?
                 {
                while mc > 0 as libc::c_int {
                    *xrpnt =
                        0.0f64 as libc::c_float; // short band -> step = 3
                    xrpnt =
                        xrpnt.offset(3 as libc::c_int as
                                         isize); // optimize: field will be set to zero at the end of the function
                    *xrpnt = 0.0f64 as libc::c_float;
                    xrpnt = xrpnt.offset(3 as libc::c_int as isize);
                    mc -= 1
                }
                if m >= me { break ; }
                let fresh53 = m;
                m = m.offset(1);
                mc = *fresh53;
                let fresh54 = m;
                m = m.offset(1);
                xrpnt = (xr as *mut libc::c_float).offset(*fresh54 as isize);
                let fresh55 = m;
                m = m.offset(1);
                if *fresh55 == 0 as libc::c_int { break ; }
                m = m.offset(1)
            }
        }
        (*gr_info).maxband[0 as libc::c_int as usize] =
            max[0 as libc::c_int as usize] + 1 as libc::c_int;
        (*gr_info).maxband[1 as libc::c_int as usize] =
            max[1 as libc::c_int as usize] + 1 as libc::c_int;
        (*gr_info).maxband[2 as libc::c_int as usize] =
            max[2 as libc::c_int as usize] + 1 as libc::c_int;
        (*gr_info).maxbandl =
            max[3 as libc::c_int as usize] + 1 as libc::c_int;
        rmax =
            if max[0 as libc::c_int as usize] > max[1 as libc::c_int as usize]
               {
                max[0 as libc::c_int as usize]
            } else { max[1 as libc::c_int as usize] };
        rmax =
            (if rmax > max[2 as libc::c_int as usize] {
                 rmax
             } else { max[2 as libc::c_int as usize] }) + 1 as libc::c_int;
        (*gr_info).maxb =
            if rmax != 0 {
                (*fr).shortLimit[sfreq as usize][rmax as usize]
            } else {
                (*fr).longLimit[sfreq as
                                    usize][(max[3 as libc::c_int as usize] +
                                                1 as libc::c_int) as usize]
            } as uint
    } else {
        // decoding with 'long' BandIndex table (block_type != 2)
        let mut pretab: *const byte =
            pretab_choice[(*gr_info).preflag as usize].as_mut_ptr();
        let mut m_0: *mut libc::c_int =
            map[sfreq as usize][2 as libc::c_int as usize];
        let mut i_0: libc::c_int = 0;
        let mut max_0: libc::c_int = -(1 as libc::c_int);
        let mut cb_0: libc::c_int = 0 as libc::c_int;
        let mut v_0: libc::c_float = 0.0f64 as libc::c_float;
        let mut mc_0: libc::c_int = 0 as libc::c_int;
        // long hash table values
        i_0 = 0 as libc::c_int;
        while i_0 < 3 as libc::c_int {
            let mut h_1: *const newhuff =
                ht.as_ptr().offset((*gr_info).table_select[i_0 as usize] as
                                       isize);
            let mut lp_0: libc::c_int = l[i_0 as usize];
            while lp_0 != 0 {
                let mut x_0: int32_t = 0;
                let mut y_0: int32_t = 0;
                if mc_0 == 0 {
                    let fresh56 = m_0;
                    m_0 = m_0.offset(1);
                    mc_0 = *fresh56;
                    let fresh57 = m_0;
                    m_0 = m_0.offset(1);
                    cb_0 = *fresh57;
                    let fresh58 = scf;
                    scf = scf.offset(1);
                    let fresh59 = pretab;
                    pretab = pretab.offset(1);
                    v_0 =
                        *(*gr_info).pow2gain.offset(((*fresh58 +
                                                          *fresh59 as
                                                              libc::c_int) <<
                                                         shift) as isize)
                }
                let mut val_1: *const libc::c_short = (*h_1).table;
                while (num as libc::c_ulong) <
                          (::std::mem::size_of::<uint32_t>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong).wrapping_mul(8
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                      {
                    let fresh60 = (*fr).wordpointer;
                    (*fr).wordpointer = (*fr).wordpointer.offset(1);
                    mask |=
                        (*fresh60 as uint32_t) <<
                            (::std::mem::size_of::<uint32_t>() as
                                 libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_sub(num
                                                                                                                                 as
                                                                                                                                 libc::c_ulong);
                    num += 8 as libc::c_int;
                    part2remain -= 8 as libc::c_int
                }
                loop  {
                    let fresh61 = val_1;
                    val_1 = val_1.offset(1);
                    y_0 = *fresh61 as int32_t;
                    if !(y_0 < 0 as libc::c_int) { break ; }
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        val_1 = val_1.offset(-(y_0 as isize))
                    }
                    num -= 1;
                    mask <<= 1 as libc::c_int
                }
                x_0 = y_0 >> 4 as libc::c_int;
                y_0 &= 0xf as libc::c_int;
                if x_0 == 15 as libc::c_int && (*h_1).linbits != 0 {
                    max_0 = cb_0;
                    while (num as libc::c_ulong) <
                              (::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                          {
                        let fresh62 = (*fr).wordpointer;
                        (*fr).wordpointer = (*fr).wordpointer.offset(1);
                        mask |=
                            (*fresh62 as uint32_t) <<
                                (::std::mem::size_of::<uint32_t>() as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(num
                                                                                                                                     as
                                                                                                                                     libc::c_ulong);
                        num += 8 as libc::c_int;
                        part2remain -= 8 as libc::c_int
                    }
                    x_0 =
                        (x_0 as
                             libc::c_uint).wrapping_add(mask >>
                                                            (::std::mem::size_of::<uint32_t>()
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_ulong).wrapping_add(8
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_ulong).wrapping_sub((*h_1).linbits
                                                                                                                                                                                                 as
                                                                                                                                                                                                 libc::c_ulong))
                            as int32_t as int32_t;
                    num =
                        (num as
                             libc::c_uint).wrapping_sub((*h_1).linbits.wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint))
                            as libc::c_int as libc::c_int;
                    mask <<= (*h_1).linbits;
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        let fresh63 = xrpnt;
                        xrpnt = xrpnt.offset(1);
                        *fresh63 = -ispow[x_0 as usize] * v_0
                    } else {
                        let fresh64 = xrpnt;
                        xrpnt = xrpnt.offset(1);
                        *fresh64 = ispow[x_0 as usize] * v_0
                    }
                    mask <<= 1 as libc::c_int
                } else if x_0 != 0 {
                    max_0 = cb_0;
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        let fresh65 = xrpnt;
                        xrpnt = xrpnt.offset(1);
                        *fresh65 = -ispow[x_0 as usize] * v_0
                    } else {
                        let fresh66 = xrpnt;
                        xrpnt = xrpnt.offset(1);
                        *fresh66 = ispow[x_0 as usize] * v_0
                    }
                    num -= 1;
                    mask <<= 1 as libc::c_int
                } else {
                    let fresh67 = xrpnt;
                    xrpnt = xrpnt.offset(1);
                    *fresh67 = 0.0f64 as libc::c_float
                }
                if y_0 == 15 as libc::c_int && (*h_1).linbits != 0 {
                    max_0 = cb_0;
                    while (num as libc::c_ulong) <
                              (::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                          {
                        let fresh68 = (*fr).wordpointer;
                        (*fr).wordpointer = (*fr).wordpointer.offset(1);
                        mask |=
                            (*fresh68 as uint32_t) <<
                                (::std::mem::size_of::<uint32_t>() as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(num
                                                                                                                                     as
                                                                                                                                     libc::c_ulong);
                        num += 8 as libc::c_int;
                        part2remain -= 8 as libc::c_int
                    }
                    y_0 =
                        (y_0 as
                             libc::c_uint).wrapping_add(mask >>
                                                            (::std::mem::size_of::<uint32_t>()
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_ulong).wrapping_add(8
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_ulong).wrapping_sub((*h_1).linbits
                                                                                                                                                                                                 as
                                                                                                                                                                                                 libc::c_ulong))
                            as int32_t as int32_t;
                    num =
                        (num as
                             libc::c_uint).wrapping_sub((*h_1).linbits.wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint))
                            as libc::c_int as libc::c_int;
                    mask <<= (*h_1).linbits;
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        let fresh69 = xrpnt;
                        xrpnt = xrpnt.offset(1);
                        *fresh69 = -ispow[y_0 as usize] * v_0
                    } else {
                        let fresh70 = xrpnt;
                        xrpnt = xrpnt.offset(1);
                        *fresh70 = ispow[y_0 as usize] * v_0
                    }
                    mask <<= 1 as libc::c_int
                } else if y_0 != 0 {
                    max_0 = cb_0;
                    if mask &
                           (1 as libc::c_int as uint32_t) <<
                               (::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                           != 0 {
                        let fresh71 = xrpnt;
                        xrpnt = xrpnt.offset(1);
                        *fresh71 = -ispow[y_0 as usize] * v_0
                    } else {
                        let fresh72 = xrpnt;
                        xrpnt = xrpnt.offset(1);
                        *fresh72 = ispow[y_0 as usize] * v_0
                    }
                    num -= 1;
                    mask <<= 1 as libc::c_int
                } else {
                    let fresh73 = xrpnt;
                    xrpnt = xrpnt.offset(1);
                    *fresh73 = 0.0f64 as libc::c_float
                }
                lp_0 -= 1;
                mc_0 -= 1
            }
            i_0 += 1
        }
        // short (count1table) values
        while l3 != 0 && part2remain + num > 0 as libc::c_int {
            let mut h_2: *const newhuff =
                htc.as_ptr().offset((*gr_info).count1table_select as
                                        isize); // dismiss stuffing Bits
            let mut val_2: *const libc::c_short = (*h_2).table;
            let mut a_0: libc::c_short = 0;
            while (num as libc::c_ulong) <
                      (::std::mem::size_of::<uint32_t>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong).wrapping_mul(8
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong)
                  {
                let fresh74 = (*fr).wordpointer;
                (*fr).wordpointer = (*fr).wordpointer.offset(1);
                mask |=
                    (*fresh74 as uint32_t) <<
                        (::std::mem::size_of::<uint32_t>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong).wrapping_sub(num
                                                                                                                             as
                                                                                                                             libc::c_ulong);
                num += 8 as libc::c_int;
                part2remain -= 8 as libc::c_int
            }
            loop  {
                let fresh75 = val_2;
                val_2 = val_2.offset(1);
                a_0 = *fresh75;
                if !((a_0 as libc::c_int) < 0 as libc::c_int) { break ; }
                if mask &
                       (1 as libc::c_int as uint32_t) <<
                           (::std::mem::size_of::<uint32_t>() as
                                libc::c_ulong).wrapping_mul(8 as libc::c_int
                                                                as
                                                                libc::c_ulong).wrapping_sub(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong)
                       != 0 {
                    val_2 = val_2.offset(-(a_0 as libc::c_int as isize))
                }
                num -= 1;
                mask <<= 1 as libc::c_int
            }
            if part2remain + num <= 0 as libc::c_int {
                num -= part2remain + num;
                break ;
            } else {
                i_0 = 0 as libc::c_int;
                while i_0 < 4 as libc::c_int {
                    if i_0 & 1 as libc::c_int == 0 {
                        if mc_0 == 0 {
                            let fresh76 = m_0;
                            m_0 = m_0.offset(1);
                            mc_0 = *fresh76;
                            let fresh77 = m_0;
                            m_0 = m_0.offset(1);
                            cb_0 = *fresh77;
                            let fresh78 = scf;
                            scf = scf.offset(1);
                            let fresh79 = pretab;
                            pretab = pretab.offset(1);
                            v_0 =
                                *(*gr_info).pow2gain.offset(((*fresh78 +
                                                                  *fresh79 as
                                                                      libc::c_int)
                                                                 << shift) as
                                                                isize)
                        }
                        mc_0 -= 1
                    }
                    if a_0 as libc::c_int & 0x8 as libc::c_int >> i_0 != 0 {
                        max_0 = cb_0;
                        if part2remain + num <= 0 as libc::c_int { break ; }
                        if mask &
                               (1 as libc::c_int as uint32_t) <<
                                   (::std::mem::size_of::<uint32_t>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong).wrapping_sub(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong)
                               != 0 {
                            let fresh80 = xrpnt;
                            xrpnt = xrpnt.offset(1);
                            *fresh80 = -v_0
                        } else {
                            let fresh81 = xrpnt;
                            xrpnt = xrpnt.offset(1);
                            *fresh81 = v_0
                        }
                        num -= 1;
                        mask <<= 1 as libc::c_int
                    } else {
                        let fresh82 = xrpnt;
                        xrpnt = xrpnt.offset(1);
                        *fresh82 = 0.0f64 as libc::c_float
                    }
                    i_0 += 1
                }
                l3 -= 1
            }
        }
        (*gr_info).maxbandl = max_0 + 1 as libc::c_int;
        (*gr_info).maxb =
            (*fr).longLimit[sfreq as usize][(*gr_info).maxbandl as usize] as
                uint
    }
    part2remain += num;
    (*fr).bitindex -= num;
    (*fr).wordpointer =
        (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int) as
                                     isize);
    (*fr).bitindex &= 0x7 as libc::c_int;
    num = 0 as libc::c_int;
    while xrpnt <
              &mut *(*xr.offset(32 as libc::c_int as
                                    isize)).as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                  as *mut libc::c_float {
        let fresh83 = xrpnt;
        xrpnt = xrpnt.offset(1);
        *fresh83 = 0.0f64 as libc::c_float
    }
    while part2remain > 16 as libc::c_int {
        (*fr).ultmp =
            *(*fr).wordpointer.offset(0 as libc::c_int as isize) as ulong;
        (*fr).ultmp <<= 8 as libc::c_int;
        (*fr).ultmp |=
            *(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                libc::c_ulong;
        (*fr).ultmp <<= 8 as libc::c_int;
        (*fr).ultmp |=
            *(*fr).wordpointer.offset(2 as libc::c_int as isize) as
                libc::c_ulong;
        (*fr).ultmp <<= (*fr).bitindex;
        (*fr).ultmp &= 0xffffff as libc::c_int as libc::c_ulong;
        (*fr).bitindex += 16 as libc::c_int;
        (*fr).ultmp >>= 24 as libc::c_int - 16 as libc::c_int;
        (*fr).wordpointer =
            (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int) as
                                         isize);
        (*fr).bitindex &= 7 as libc::c_int;
        (*fr).ultmp = (*fr).bitindex as ulong;
        part2remain -= 16 as libc::c_int
    }
    if part2remain > 0 as libc::c_int {
        (*fr).ultmp =
            *(*fr).wordpointer.offset(0 as libc::c_int as isize) as ulong;
        (*fr).ultmp <<= 8 as libc::c_int;
        (*fr).ultmp |=
            *(*fr).wordpointer.offset(1 as libc::c_int as isize) as
                libc::c_ulong;
        (*fr).ultmp <<= 8 as libc::c_int;
        (*fr).ultmp |=
            *(*fr).wordpointer.offset(2 as libc::c_int as isize) as
                libc::c_ulong;
        (*fr).ultmp <<= (*fr).bitindex;
        (*fr).ultmp &= 0xffffff as libc::c_int as libc::c_ulong;
        (*fr).bitindex += part2remain;
        (*fr).ultmp >>= 24 as libc::c_int - part2remain;
        (*fr).wordpointer =
            (*fr).wordpointer.offset(((*fr).bitindex >> 3 as libc::c_int) as
                                         isize);
        (*fr).bitindex &= 7 as libc::c_int;
        (*fr).ultmp = (*fr).bitindex as ulong
    } else if part2remain < 0 as libc::c_int {
        // error
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// calculate float channel values for Joint-I-Stereo-mode
unsafe extern "C" fn III_i_stereo(mut xr_buf: *mut [[libc::c_float; 18]; 32],
                                  mut scalefac: *mut libc::c_int,
                                  mut gr_info: *mut gr_info_t,
                                  mut sfreq: libc::c_int,
                                  mut ms_stereo: libc::c_int,
                                  mut lsf: libc::c_int) {
    let mut xr: *mut [libc::c_float; 576] =
        xr_buf as *mut [libc::c_float; 576];
    let mut bi: *const bandInfoStruct =
        &*bandInfo.as_ptr().offset(sfreq as isize) as *const bandInfoStruct;
    let mut tab1_0: *const libc::c_float = 0 as *const libc::c_float;
    let mut tab2_0: *const libc::c_float = 0 as *const libc::c_float;
    let mut tab: libc::c_int = 0;
    // TODO: optimize as static
    let mut tabs: [[[*const libc::c_float; 2]; 2]; 3] =
        [[[tan1_1.as_mut_ptr() as *const libc::c_float,
           tan2_1.as_mut_ptr() as *const libc::c_float],
          [tan1_2.as_mut_ptr() as *const libc::c_float,
           tan2_2.as_mut_ptr() as *const libc::c_float]],
         [[pow1_1[0 as libc::c_int as usize].as_mut_ptr() as
               *const libc::c_float,
           pow2_1[0 as libc::c_int as usize].as_mut_ptr() as
               *const libc::c_float],
          [pow1_2[0 as libc::c_int as usize].as_mut_ptr() as
               *const libc::c_float,
           pow2_2[0 as libc::c_int as usize].as_mut_ptr() as
               *const libc::c_float]],
         [[pow1_1[1 as libc::c_int as usize].as_mut_ptr() as
               *const libc::c_float,
           pow2_1[1 as libc::c_int as usize].as_mut_ptr() as
               *const libc::c_float],
          [pow1_2[1 as libc::c_int as usize].as_mut_ptr() as
               *const libc::c_float,
           pow2_2[1 as libc::c_int as usize].as_mut_ptr() as
               *const libc::c_float]]]; // sfb is minimal 3 for mixed mode
    tab =
        (lsf as
             libc::c_uint).wrapping_add((*gr_info).scalefac_compress &
                                            lsf as libc::c_uint) as
            libc::c_int;
    tab1_0 =
        tabs[tab as usize][ms_stereo as usize][0 as libc::c_int as usize];
    tab2_0 =
        tabs[tab as usize][ms_stereo as usize][1 as libc::c_int as usize];
    if (*gr_info).block_type == 2 as libc::c_int as libc::c_uint {
        let mut lwin: libc::c_int = 0;
        let mut do_l: libc::c_int = 0 as libc::c_int;
        if (*gr_info).mixed_block_flag != 0 { do_l = 1 as libc::c_int }
        lwin = 0 as libc::c_int;
        while lwin < 3 as libc::c_int {
            let mut is_p: libc::c_int = 0;
            let mut sb: libc::c_int = 0;
            let mut idx: libc::c_int = 0;
            let mut sfb: libc::c_int = (*gr_info).maxband[lwin as usize];
            if sfb > 3 as libc::c_int { do_l = 0 as libc::c_int }
            // process each window
			// get first band with zero values
            while sfb < 12 as libc::c_int {
                is_p =
                    *scalefac.offset(((sfb * 3 as libc::c_int + lwin) as
                                          libc::c_uint).wrapping_sub((*gr_info).mixed_block_flag)
                                         as isize); // scale: 0-15
                if is_p != 7 as libc::c_int {
                    let mut t1: libc::c_float = 0.;
                    let mut t2: libc::c_float = 0.;
                    sb = (*bi).shortDiff[sfb as usize] as libc::c_int;
                    idx = (*bi).shortIdx[sfb as usize] as libc::c_int + lwin;
                    t1 = *tab1_0.offset(is_p as isize);
                    t2 = *tab2_0.offset(is_p as isize);
                    while sb > 0 as libc::c_int {
                        let mut v: libc::c_float =
                            (*xr.offset(0 as libc::c_int as
                                            isize))[idx as usize];
                        (*xr.offset(0 as libc::c_int as isize))[idx as usize]
                            = v * t1;
                        (*xr.offset(1 as libc::c_int as isize))[idx as usize]
                            = v * t2;
                        sb -= 1;
                        idx += 3 as libc::c_int
                    }
                }
                sfb += 1
            }
            // in the original: copy 10 to 11 , here: copy 11 to 12
			// maybe still wrong??? (copy 12 to 13?)
            is_p =
                *scalefac.offset(((11 as libc::c_int * 3 as libc::c_int +
                                       lwin) as
                                      libc::c_uint).wrapping_sub((*gr_info).mixed_block_flag)
                                     as isize); // scale: 0-15
            sb = (*bi).shortDiff[12 as libc::c_int as usize] as libc::c_int;
            idx =
                (*bi).shortIdx[12 as libc::c_int as usize] as libc::c_int +
                    lwin;
            if is_p != 7 as libc::c_int {
                let mut t1_0: libc::c_float = 0.;
                let mut t2_0: libc::c_float = 0.;
                t1_0 = *tab1_0.offset(is_p as isize);
                t2_0 = *tab2_0.offset(is_p as isize);
                while sb > 0 as libc::c_int {
                    let mut v_0: libc::c_float =
                        (*xr.offset(0 as libc::c_int as isize))[idx as usize];
                    (*xr.offset(0 as libc::c_int as isize))[idx as usize] =
                        v_0 * t1_0;
                    (*xr.offset(1 as libc::c_int as isize))[idx as usize] =
                        v_0 * t2_0;
                    sb -= 1;
                    idx += 3 as libc::c_int
                }
            }
            lwin += 1
        }
        // also check l-part, if ALL bands in the three windows are 'empty' and mode = mixed_mode
        if do_l != 0 {
            let mut idx_0: libc::c_int =
                0; // similarity fix related to CVE-2006-1655
            let mut sfb_0: libc::c_int = (*gr_info).maxbandl; // scale: 0-15
            if sfb_0 > 21 as libc::c_int {
                return
            } // tightened fix for CVE-2006-1655
            idx_0 =
                (*bi).longIdx[sfb_0 as usize] as libc::c_int; // scale: 0-15
            while sfb_0 < 8 as libc::c_int {
                let mut sb_0: libc::c_int =
                    (*bi).longDiff[sfb_0 as usize] as libc::c_int;
                let mut is_p_0: libc::c_int =
                    *scalefac.offset(sfb_0 as isize);
                if is_p_0 != 7 as libc::c_int {
                    let mut t1_1: libc::c_float = 0.;
                    let mut t2_1: libc::c_float = 0.;
                    t1_1 = *tab1_0.offset(is_p_0 as isize);
                    t2_1 = *tab2_0.offset(is_p_0 as isize);
                    while sb_0 > 0 as libc::c_int {
                        let mut v_1: libc::c_float =
                            (*xr.offset(0 as libc::c_int as
                                            isize))[idx_0 as usize];
                        (*xr.offset(0 as libc::c_int as
                                        isize))[idx_0 as usize] = v_1 * t1_1;
                        (*xr.offset(1 as libc::c_int as
                                        isize))[idx_0 as usize] = v_1 * t2_1;
                        sb_0 -= 1;
                        idx_0 += 1
                    }
                } else { idx_0 += sb_0 }
                sfb_0 += 1
            }
        }
    } else {
        let mut sfb_1: libc::c_int = (*gr_info).maxbandl;
        let mut is_p_1: libc::c_int = 0;
        let mut idx_1: libc::c_int = 0;
        if sfb_1 > 21 as libc::c_int { return }
        idx_1 = (*bi).longIdx[sfb_1 as usize] as libc::c_int;
        while sfb_1 < 21 as libc::c_int {
            let mut sb_1: libc::c_int =
                (*bi).longDiff[sfb_1 as usize] as libc::c_int;
            is_p_1 = *scalefac.offset(sfb_1 as isize);
            if is_p_1 != 7 as libc::c_int {
                let mut t1_2: libc::c_float = 0.;
                let mut t2_2: libc::c_float = 0.;
                t1_2 = *tab1_0.offset(is_p_1 as isize);
                t2_2 = *tab2_0.offset(is_p_1 as isize);
                while sb_1 > 0 as libc::c_int {
                    let mut v_2: libc::c_float =
                        (*xr.offset(0 as libc::c_int as
                                        isize))[idx_1 as usize];
                    (*xr.offset(0 as libc::c_int as isize))[idx_1 as usize] =
                        v_2 * t1_2;
                    (*xr.offset(1 as libc::c_int as isize))[idx_1 as usize] =
                        v_2 * t2_2;
                    sb_1 -= 1;
                    idx_1 += 1
                }
            } else { idx_1 += sb_1 }
            sfb_1 += 1
        }
        is_p_1 = *scalefac.offset(20 as libc::c_int as isize);
        if is_p_1 != 7 as libc::c_int {
            let mut t1_3: libc::c_float = 0.;
            let mut t2_3: libc::c_float = 0.;
            let mut sb_2: libc::c_int = 0;
            t1_3 = *tab1_0.offset(is_p_1 as isize);
            t2_3 = *tab2_0.offset(is_p_1 as isize);
            // copy l-band 20 to l-band 21
            sb_2 = (*bi).longDiff[21 as libc::c_int as usize] as libc::c_int;
            while sb_2 > 0 as libc::c_int {
                let mut v_3: libc::c_float =
                    (*xr.offset(0 as libc::c_int as isize))[idx_1 as usize];
                (*xr.offset(0 as libc::c_int as isize))[idx_1 as usize] =
                    v_3 * t1_3;
                (*xr.offset(1 as libc::c_int as isize))[idx_1 as usize] =
                    v_3 * t2_3;
                sb_2 -= 1;
                idx_1 += 1
            }
        }
    };
}
unsafe extern "C" fn III_antialias(mut xr: *mut [libc::c_float; 18],
                                   mut gr_info: *mut gr_info_t) {
    let mut sblim: libc::c_int = 0;
    let mut sb: libc::c_int = 0;
    let mut xr1: *mut libc::c_float = 0 as *mut libc::c_float;
    if (*gr_info).block_type == 2 as libc::c_int as libc::c_uint {
        if (*gr_info).mixed_block_flag == 0 { return }
        sblim = 1 as libc::c_int
    } else {
        sblim =
            (*gr_info).maxb.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                libc::c_int
    }
    // 31 alias-reduction operations between each pair of sub-bands
	// with 8 butterflies between each pair
    xr1 = (*xr.offset(1 as libc::c_int as isize)).as_mut_ptr();
    sb = sblim;
    while sb != 0 {
        let mut cs: *mut libc::c_float = aa_cs.as_mut_ptr();
        let mut ca: *mut libc::c_float = aa_ca.as_mut_ptr();
        let mut xr2: *mut libc::c_float = xr1;
        let mut ss: libc::c_int = 0;
        ss = 7 as libc::c_int;
        while ss >= 0 as libc::c_int {
            // upper and lower butterfly inputs
            xr2 = xr2.offset(-1);
            let mut bu: libc::c_float = *xr2;
            let mut bd: libc::c_float = *xr1;
            *xr2 = bu * *cs - bd * *ca;
            let fresh84 = cs;
            cs = cs.offset(1);
            let fresh85 = ca;
            ca = ca.offset(1);
            let fresh86 = xr1;
            xr1 = xr1.offset(1);
            *fresh86 = bd * *fresh84 + bu * *fresh85;
            ss -= 1
        }
        sb -= 1;
        xr1 = xr1.offset(10 as libc::c_int as isize)
    };
}
unsafe extern "C" fn III_hybrid(mut fsIn: *mut [libc::c_float; 18],
                                mut tsOut: *mut [libc::c_float; 32],
                                mut ch: libc::c_int,
                                mut gr_info: *mut gr_info_t,
                                mut fr: *mut mpg123_handle_t) {
    let mut block: *mut [[libc::c_float; 576]; 2] =
        (*fr).hybrid_block.as_mut_ptr();
    let mut blc: *mut libc::c_int = (*fr).hybrid_blc.as_mut_ptr();
    let mut tspnt: *mut libc::c_float = tsOut as *mut libc::c_float;
    let mut rawout1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut rawout2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut bt: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut sb: size_t = 0 as libc::c_int as size_t;
    b = *blc.offset(ch as isize);
    rawout1 = (*block.offset(b as isize))[ch as usize].as_mut_ptr();
    b = -b + 1 as libc::c_int;
    rawout2 = (*block.offset(b as isize))[ch as usize].as_mut_ptr();
    *blc.offset(ch as isize) = b;
    if (*gr_info).mixed_block_flag != 0 {
        sb = 2 as libc::c_int as size_t;
        dct36((*fsIn.offset(0 as libc::c_int as isize)).as_mut_ptr(), rawout1,
              rawout2, win[0 as libc::c_int as usize].as_mut_ptr(), tspnt);
        dct36((*fsIn.offset(1 as libc::c_int as isize)).as_mut_ptr(),
              rawout1.offset(18 as libc::c_int as isize),
              rawout2.offset(18 as libc::c_int as isize),
              win1[0 as libc::c_int as usize].as_mut_ptr(),
              tspnt.offset(1 as libc::c_int as isize));
        rawout1 = rawout1.offset(36 as libc::c_int as isize);
        rawout2 = rawout2.offset(36 as libc::c_int as isize);
        tspnt = tspnt.offset(2 as libc::c_int as isize)
    }
    bt = (*gr_info).block_type as libc::c_int;
    if bt == 2 as libc::c_int {
        while sb < (*gr_info).maxb as libc::c_ulong {
            dct12((*fsIn.offset(sb as isize)).as_mut_ptr(), rawout1, rawout2,
                  win[2 as libc::c_int as usize].as_mut_ptr(), tspnt);
            dct12((*fsIn.offset(sb.wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong) as
                                    isize)).as_mut_ptr(),
                  rawout1.offset(18 as libc::c_int as isize),
                  rawout2.offset(18 as libc::c_int as isize),
                  win1[2 as libc::c_int as usize].as_mut_ptr(),
                  tspnt.offset(1 as libc::c_int as isize));
            sb =
                (sb as
                     libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            tspnt = tspnt.offset(2 as libc::c_int as isize);
            rawout1 = rawout1.offset(36 as libc::c_int as isize);
            rawout2 = rawout2.offset(36 as libc::c_int as isize)
        }
    } else {
        while sb < (*gr_info).maxb as libc::c_ulong {
            dct36((*fsIn.offset(sb as isize)).as_mut_ptr(), rawout1, rawout2,
                  win[bt as usize].as_mut_ptr(), tspnt);
            dct36((*fsIn.offset(sb.wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong) as
                                    isize)).as_mut_ptr(),
                  rawout1.offset(18 as libc::c_int as isize),
                  rawout2.offset(18 as libc::c_int as isize),
                  win1[bt as usize].as_mut_ptr(),
                  tspnt.offset(1 as libc::c_int as isize));
            sb =
                (sb as
                     libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            tspnt = tspnt.offset(2 as libc::c_int as isize);
            rawout1 = rawout1.offset(36 as libc::c_int as isize);
            rawout2 = rawout2.offset(36 as libc::c_int as isize)
        }
    }
    while sb < 32 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int;
        while i < 18 as libc::c_int {
            let fresh87 = rawout1;
            rawout1 = rawout1.offset(1);
            *tspnt.offset((i * 32 as libc::c_int) as isize) = *fresh87;
            let fresh88 = rawout2;
            rawout2 = rawout2.offset(1);
            *fresh88 = 0.0f64 as libc::c_float;
            i += 1
        }
        sb = sb.wrapping_add(1);
        tspnt = tspnt.offset(1)
    };
}
// and at the end... the main layer3 handler
#[no_mangle]
pub unsafe extern "C" fn do_layer3(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut gr: libc::c_int =
        0; // max 39 for short[13][3] mode, mixed: 38, long: 22
    let mut ch: libc::c_int = 0;
    let mut ss: libc::c_int = 0;
    let mut clip: libc::c_int = 0 as libc::c_int;
    let mut stereo: libc::c_int = (*fr).stereo;
    let mut single: libc::c_int = (*fr).single;
    let mut ms_stereo: libc::c_int = 0;
    let mut i_stereo: libc::c_int = 0;
    let mut sfreq: libc::c_int = (*fr).sampling_frequency;
    let mut scalefacs: [[libc::c_int; 39]; 2] = [[0; 39]; 2];
    let mut stereo1: libc::c_int = 0;
    let mut granules: libc::c_int = 0;
    let mut sideinfo: III_sideinfo =
        III_sideinfo{main_data_begin: 0,
                     private_bits: 0,
                     ch:
                         [C2RustUnnamed_1{gr:
                                              [gr_info_t{scfsi: 0,
                                                         part2_3_length: 0,
                                                         big_values: 0,
                                                         scalefac_compress: 0,
                                                         block_type: 0,
                                                         mixed_block_flag: 0,
                                                         table_select: [0; 3],
                                                         maxband: [0; 3],
                                                         maxbandl: 0,
                                                         maxb: 0,
                                                         region1start: 0,
                                                         region2start: 0,
                                                         preflag: 0,
                                                         scalefac_scale: 0,
                                                         count1table_select:
                                                             0,
                                                         full_gain:
                                                             [0 as
                                                                  *mut libc::c_float;
                                                                 3],
                                                         pow2gain:
                                                             0 as
                                                                 *mut libc::c_float,};
                                                  2],}; 2],};
    if stereo == 1 as libc::c_int {
        // stream is mono
        stereo1 = 1 as libc::c_int;
        single = 0 as libc::c_int
    } else if single != -(1 as libc::c_int) {
        // stream is stereo, but force to mono
        stereo1 = 1 as libc::c_int
    } else { stereo1 = 2 as libc::c_int }
    if (*fr).mode == 1 as libc::c_int {
        ms_stereo = ((*fr).mode_ext & 0x2 as libc::c_int) >> 1 as libc::c_int;
        i_stereo = (*fr).mode_ext & 0x1 as libc::c_int
    } else { i_stereo = 0 as libc::c_int; ms_stereo = i_stereo }
    granules =
        if (*fr).lsf != 0 { 1 as libc::c_int } else { 2 as libc::c_int };
    // quick hack to keep the music playing
	// after having seen this nasty test file...
    if III_get_side_info(fr, &mut sideinfo, stereo, ms_stereo,
                         sfreq as libc::c_long, single) != 0 {
        return clip
    } //  hybridIn[2][SBLIMIT][SSLIMIT]
    set_pointer(fr,
                sideinfo.main_data_begin as
                    libc::c_long); //  hybridOut[2][SSLIMIT][SBLIMIT]
    gr = 0 as libc::c_int; // *0.5 done by pow-scale
    while gr < granules {
        let mut hybridIn: *mut [[libc::c_float; 18]; 32] =
            (*fr).layer3.hybrid_in;
        let mut hybridOut: *mut [[libc::c_float; 32]; 18] =
            (*fr).layer3.hybrid_out;
        let mut gr_info: *mut gr_info_t =
            &mut *(*sideinfo.ch.as_mut_ptr().offset(0 as libc::c_int as
                                                        isize)).gr.as_mut_ptr().offset(gr
                                                                                           as
                                                                                           isize)
                as *mut gr_info_t;
        let mut part2bits: libc::c_long = 0;
        if (*fr).lsf != 0 {
            part2bits =
                III_get_scale_factors_2(fr,
                                        scalefacs[0 as libc::c_int as
                                                      usize].as_mut_ptr(),
                                        gr_info, 0 as libc::c_int) as
                    libc::c_long
        } else {
            part2bits =
                III_get_scale_factors_1(fr,
                                        scalefacs[0 as libc::c_int as
                                                      usize].as_mut_ptr(),
                                        gr_info) as libc::c_long
        }
        if III_dequantize_sample(fr,
                                 (*hybridIn.offset(0 as libc::c_int as
                                                       isize)).as_mut_ptr(),
                                 scalefacs[0 as libc::c_int as
                                               usize].as_mut_ptr(), gr_info,
                                 sfreq, part2bits as libc::c_int) != 0 {
            return clip
        }
        if stereo == 2 as libc::c_int {
            let mut in0: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut in1: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut i: libc::c_int = 0;
            gr_info =
                &mut *(*sideinfo.ch.as_mut_ptr().offset(1 as libc::c_int as
                                                            isize)).gr.as_mut_ptr().offset(gr
                                                                                               as
                                                                                               isize)
                    as *mut gr_info_t;
            if (*fr).lsf != 0 {
                part2bits =
                    III_get_scale_factors_2(fr,
                                            scalefacs[1 as libc::c_int as
                                                          usize].as_mut_ptr(),
                                            gr_info, i_stereo) as libc::c_long
            } else {
                part2bits =
                    III_get_scale_factors_1(fr,
                                            scalefacs[1 as libc::c_int as
                                                          usize].as_mut_ptr(),
                                            gr_info) as libc::c_long
            }
            if III_dequantize_sample(fr,
                                     (*hybridIn.offset(1 as libc::c_int as
                                                           isize)).as_mut_ptr(),
                                     scalefacs[1 as libc::c_int as
                                                   usize].as_mut_ptr(),
                                     gr_info, sfreq, part2bits as libc::c_int)
                   != 0 {
                return clip
            }
            if ms_stereo != 0 {
                let mut maxb: uint =
                    sideinfo.ch[0 as libc::c_int as
                                    usize].gr[gr as usize].maxb;
                let mut i_0: libc::c_int = 0;
                if sideinfo.ch[1 as libc::c_int as usize].gr[gr as usize].maxb
                       > maxb {
                    maxb =
                        sideinfo.ch[1 as libc::c_int as
                                        usize].gr[gr as usize].maxb
                }
                i_0 = 0 as libc::c_int;
                while i_0 < 18 as libc::c_int * maxb as libc::c_int {
                    let mut tmp0: libc::c_float =
                        *((*hybridIn.offset(0 as libc::c_int as
                                                isize)).as_mut_ptr() as
                              *mut libc::c_float).offset(i_0 as isize);
                    let mut tmp1: libc::c_float =
                        *((*hybridIn.offset(1 as libc::c_int as
                                                isize)).as_mut_ptr() as
                              *mut libc::c_float).offset(i_0 as isize);
                    *((*hybridIn.offset(0 as libc::c_int as
                                            isize)).as_mut_ptr() as
                          *mut libc::c_float).offset(i_0 as isize) =
                        tmp0 + tmp1;
                    *((*hybridIn.offset(1 as libc::c_int as
                                            isize)).as_mut_ptr() as
                          *mut libc::c_float).offset(i_0 as isize) =
                        tmp0 - tmp1;
                    i_0 += 1
                }
            }
            if i_stereo != 0 {
                III_i_stereo(hybridIn,
                             scalefacs[1 as libc::c_int as
                                           usize].as_mut_ptr(), gr_info,
                             sfreq, ms_stereo, (*fr).lsf);
            }
            if ms_stereo != 0 || i_stereo != 0 || single == 3 as libc::c_int {
                if (*gr_info).maxb >
                       sideinfo.ch[0 as libc::c_int as
                                       usize].gr[gr as usize].maxb {
                    sideinfo.ch[0 as libc::c_int as
                                    usize].gr[gr as usize].maxb =
                        (*gr_info).maxb
                } else {
                    (*gr_info).maxb =
                        sideinfo.ch[0 as libc::c_int as
                                        usize].gr[gr as usize].maxb
                }
            }
            match single {
                3 => {
                    in0 =
                        (*hybridIn.offset(0 as libc::c_int as
                                              isize)).as_mut_ptr() as
                            *mut libc::c_float;
                    in1 =
                        (*hybridIn.offset(1 as libc::c_int as
                                              isize)).as_mut_ptr() as
                            *mut libc::c_float;
                    i = 0 as libc::c_int;
                    while i <
                              18 as libc::c_int *
                                  (*gr_info).maxb as libc::c_int {
                        let fresh89 = in1;
                        in1 = in1.offset(1);
                        *in0 = *in0 + *fresh89;
                        i += 1;
                        in0 = in0.offset(1)
                    }
                }
                1 => {
                    in0 =
                        (*hybridIn.offset(0 as libc::c_int as
                                              isize)).as_mut_ptr() as
                            *mut libc::c_float;
                    in1 =
                        (*hybridIn.offset(1 as libc::c_int as
                                              isize)).as_mut_ptr() as
                            *mut libc::c_float;
                    i = 0 as libc::c_int;
                    while i <
                              18 as libc::c_int *
                                  (*gr_info).maxb as libc::c_int {
                        let fresh90 = in1;
                        in1 = in1.offset(1);
                        let fresh91 = in0;
                        in0 = in0.offset(1);
                        *fresh91 = *fresh90;
                        i += 1
                    }
                }
                _ => { }
            }
        }
        ch = 0 as libc::c_int;
        while ch < stereo1 {
            gr_info =
                &mut *(*sideinfo.ch.as_mut_ptr().offset(ch as
                                                            isize)).gr.as_mut_ptr().offset(gr
                                                                                               as
                                                                                               isize)
                    as *mut gr_info_t;
            III_antialias((*hybridIn.offset(ch as isize)).as_mut_ptr(),
                          gr_info);
            III_hybrid((*hybridIn.offset(ch as isize)).as_mut_ptr(),
                       (*hybridOut.offset(ch as isize)).as_mut_ptr(), ch,
                       gr_info, fr);
            ch += 1
        }
        ss = 0 as libc::c_int;
        while ss < 18 as libc::c_int {
            if single != -(1 as libc::c_int) {
                clip +=
                    (*fr).synth_mono.expect("non-null function pointer")((*hybridOut.offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize))[ss
                                                                                                            as
                                                                                                            usize].as_mut_ptr(),
                                                                         fr)
            } else {
                clip +=
                    (*fr).synth_stereo.expect("non-null function pointer")((*hybridOut.offset(0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize))[ss
                                                                                                              as
                                                                                                              usize].as_mut_ptr(),
                                                                           (*hybridOut.offset(1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize))[ss
                                                                                                              as
                                                                                                              usize].as_mut_ptr(),
                                                                           fr)
            }
            ss += 1
        }
        gr += 1
    }
    return clip;
}
