#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn init_synth(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn set_synth_functions(fr: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn bc_poolsize(_: *mut bufferchain_t, pool_size: size_t,
                   bufblock: size_t);
    #[no_mangle]
    fn open_stream_handle(fr: *mut mpg123_handle_t,
                          iohandle: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn open_feed(fr: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn feed_more(fr: *mut mpg123_handle_t, in_0: *const byte,
                 count: libc::c_long) -> libc::c_int;
    #[no_mangle]
    fn frame_init_par(fr: *mut mpg123_handle_t, mp: *mut mpg123_parm_t);
    #[no_mangle]
    fn frame_outbuffer(fr: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn frame_output_format(fr: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn frame_reset(fr: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn frame_buffers_reset(fr: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn frame_exit(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn frame_index_setup(fr: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn frame_expect_outsamples(fr: *mut mpg123_handle_t) -> mpg_off_t;
    #[no_mangle]
    fn frame_gapless_realinit(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn frame_gapless_update(fr: *mut mpg123_handle_t,
                            total_samples: mpg_off_t);
    #[no_mangle]
    fn frame_outs(fr: *mut mpg123_handle_t, num: mpg_off_t) -> mpg_off_t;
    #[no_mangle]
    fn frame_set_seek(fr: *mut mpg123_handle_t, sp: mpg_off_t);
    #[no_mangle]
    fn frame_set_frameseek(fr: *mut mpg123_handle_t, fe: mpg_off_t);
    #[no_mangle]
    fn frame_skip(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn do_rva(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn frame_freq(fr: *mut mpg123_handle_t) -> libc::c_long;
    #[no_mangle]
    fn read_frame(fr: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn invalidate_format(af: *mut audioformat_t);
    #[no_mangle]
    fn postprocess_buffer(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn decoder_synth_bytes(fr: *mut mpg123_handle_t, s: mpg_off_t)
     -> mpg_off_t;
    #[no_mangle]
    fn bytes_to_samples(fr: *mut mpg123_handle_t, b: mpg_off_t) -> mpg_off_t;
    #[no_mangle]
    fn samples_to_bytes(fr: *mut mpg123_handle_t, s: mpg_off_t) -> mpg_off_t;
    #[no_mangle]
    fn outblock_bytes(fr: *mut mpg123_handle_t, s: mpg_off_t) -> mpg_off_t;
    #[no_mangle]
    fn init_layer3();
    #[no_mangle]
    fn prepare_decode_tables();
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
pub type mpg123_parms = libc::c_uint;
pub const MPG123_FEEDBUFFER: mpg123_parms = 17;
pub const MPG123_FEEDPOOL: mpg123_parms = 16;
pub const MPG123_PREFRAMES: mpg123_parms = 15;
pub const MPG123_INDEX_SIZE: mpg123_parms = 14;
pub const MPG123_RESYNC_LIMIT: mpg123_parms = 13;
pub const MPG123_REMOVE_FLAGS: mpg123_parms = 12;
pub const MPG123_TIMEOUT: mpg123_parms = 11;
pub const MPG123_OUTSCALE: mpg123_parms = 10;
pub const MPG123_DECODE_FRAMES: mpg123_parms = 9;
pub const MPG123_START_FRAME: mpg123_parms = 8;
pub const MPG123_UPSPEED: mpg123_parms = 7;
pub const MPG123_DOWNSPEED: mpg123_parms = 6;
pub const MPG123_RVA: mpg123_parms = 5;
pub const MPG123_DOWN_SAMPLE: mpg123_parms = 4;
pub const MPG123_FORCE_RATE: mpg123_parms = 3;
pub const MPG123_ADD_FLAGS: mpg123_parms = 2;
pub const MPG123_FLAGS: mpg123_parms = 1;
pub const MPG123_VERBOSE: mpg123_parms = 0;
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
pub type mpg123_param_rva = libc::c_uint;
pub const MPG123_RVA_MAX: mpg123_param_rva = 2;
pub const MPG123_RVA_ALBUM: mpg123_param_rva = 2;
pub const MPG123_RVA_MIX: mpg123_param_rva = 1;
pub const MPG123_RVA_OFF: mpg123_param_rva = 0;
pub type frame_state_flags = libc::c_uint;
pub const FRAME_FRESH_DECODER: frame_state_flags = 4;
pub const FRAME_FRANKENSTEIN: frame_state_flags = 2;
pub const FRAME_ACCURATE: frame_state_flags = 1;
/*
mpg123.c - compact version of famous library mpg123
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
static mut initialized: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn mpg123_init() -> libc::c_int {
    if ::std::mem::size_of::<libc::c_short>() as libc::c_ulong !=
           2 as libc::c_int as libc::c_ulong ||
           (::std::mem::size_of::<libc::c_long>() as libc::c_ulong) <
               4 as libc::c_int as libc::c_ulong {
        return MPG123_BAD_TYPES as libc::c_int
    } // no need to initialize twice
    if initialized != 0 { return MPG123_OK as libc::c_int }
    init_layer3();
    prepare_decode_tables();
    initialized = 1 as libc::c_int;
    return MPG123_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_exit() {
    // nothing yet, but something later perhaps
}
// create a new handle with specified decoder, decoder can be "", "auto" or NULL for auto-detection
#[no_mangle]
pub unsafe extern "C" fn mpg123_new(mut error: *mut libc::c_int)
 -> *mut mpg123_handle_t {
    return mpg123_parnew(0 as *mut mpg123_parm_t, error);
}
// ...the full routine with optional initial parameters to override defaults.
#[no_mangle]
pub unsafe extern "C" fn mpg123_parnew(mut mp: *mut mpg123_parm_t,
                                       mut error: *mut libc::c_int)
 -> *mut mpg123_handle_t {
    let mut fr: *mut mpg123_handle_t = 0 as *mut mpg123_handle_t;
    let mut err: libc::c_int = MPG123_OK as libc::c_int;
    if initialized != 0 {
        fr =
            malloc(::std::mem::size_of::<mpg123_handle_t>() as libc::c_ulong)
                as *mut mpg123_handle_t
    } else { err = MPG123_NOT_INITIALIZED as libc::c_int }
    if !fr.is_null() { frame_init_par(fr, mp); init_synth(fr); }
    if !fr.is_null() {
        (*fr).decoder_change = 1 as libc::c_int
    } else if err == MPG123_OK as libc::c_int {
        err = MPG123_OUT_OF_MEM as libc::c_int
    }
    if !error.is_null() { *error = err }
    return fr;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_par(mut mp: *mut mpg123_parm_t,
                                    mut key: mpg123_parms,
                                    mut val: libc::c_long) -> libc::c_int {
    let mut ret: libc::c_int = MPG123_OK as libc::c_int;
    if mp.is_null() { return MPG123_BAD_PARS as libc::c_int }
    match key as libc::c_uint {
        0 => { (*mp).verbose = val as libc::c_int }
        1 => { if ret == MPG123_OK as libc::c_int { (*mp).flags = val } }
        2 => { (*mp).flags |= val }
        12 => { (*mp).flags &= !val }
        3 => {
            // should this trigger something
            if val > 0 as libc::c_int as libc::c_long {
                ret = MPG123_BAD_RATE as libc::c_int
            }
        }
        4 => {
            if val != 0 as libc::c_int as libc::c_long {
                ret = MPG123_BAD_RATE as libc::c_int
            }
        }
        5 => {
            if val < 0 as libc::c_int as libc::c_long ||
                   val > MPG123_RVA_MAX as libc::c_int as libc::c_long {
                ret = MPG123_BAD_RVA as libc::c_int
            } else { (*mp).rva = val as libc::c_int }
        }
        6 => {
            (*mp).halfspeed =
                if val < 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int as libc::c_long
                } else { val }
        }
        7 => {
            (*mp).doublespeed =
                if val < 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int as libc::c_long
                } else { val }
        }
        10 => {
            // choose the value that is non-zero, if any.
		// downscaling integers to 1.0.
            (*mp).outscale =
                val as libc::c_double / 32768 as libc::c_int as libc::c_double
        }
        11 => {
            if val > 0 as libc::c_int as libc::c_long {
                ret = MPG123_NO_TIMEOUT as libc::c_int
            }
        }
        13 => { (*mp).resync_limit = val }
        14 => { (*mp).index_size = val }
        15 => {
            if val >= 0 as libc::c_int as libc::c_long {
                (*mp).preframes = val
            } else { ret = MPG123_BAD_VALUE as libc::c_int }
        }
        16 => {
            if val >= 0 as libc::c_int as libc::c_long {
                (*mp).feedpool = val
            } else { ret = MPG123_BAD_VALUE as libc::c_int }
        }
        17 => {
            if val > 0 as libc::c_int as libc::c_long {
                (*mp).feedbuffer = val
            } else { ret = MPG123_BAD_VALUE as libc::c_int }
        }
        _ => { ret = MPG123_BAD_PARAM as libc::c_int }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_param(mut mh: *mut mpg123_handle_t,
                                      mut key: mpg123_parms,
                                      mut val: libc::c_long) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    r = mpg123_par(&mut (*mh).p, key, val);
    if r != MPG123_OK as libc::c_int {
        (*mh).err = r;
        return MPG123_ERR as libc::c_int
    } else {
        // special treatment for some settings.
        if key as libc::c_uint ==
               MPG123_INDEX_SIZE as libc::c_int as libc::c_uint {
            // apply frame index size and grow property on the fly.
            r = frame_index_setup(mh);
            if r != MPG123_OK as libc::c_int {
                (*mh).err = MPG123_INDEX_FAIL as libc::c_int
            }
        }
        // feeder pool size is applied right away, reader will react to that.
        if key as libc::c_uint ==
               MPG123_FEEDPOOL as libc::c_int as libc::c_uint ||
               key as libc::c_uint ==
                   MPG123_FEEDBUFFER as libc::c_int as libc::c_uint {
            bc_poolsize(&mut (*mh).rdat.buffer, (*mh).p.feedpool as size_t,
                        (*mh).p.feedbuffer as size_t);
        }
        return r
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_close(mut mh: *mut mpg123_handle_t)
 -> libc::c_int {
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    // mh->rd is never NULL!
    if (*(*mh).rd).close.is_some() {
        (*(*mh).rd).close.expect("non-null function pointer")(mh);
    }
    if (*mh).new_format != 0 {
        invalidate_format(&mut (*mh).af);
        (*mh).new_format = 0 as libc::c_int
    }
    // always reset the frame buffers on close, so we cannot forget it in funky opening routines (wrappers, even).
    frame_reset(mh);
    return MPG123_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_delete(mut mh: *mut mpg123_handle_t) {
    if !mh.is_null() {
        mpg123_close(mh);
        // free struct; cast?
        frame_exit(mh); // free buffers in frame
        free(mh as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_open_handle(mut mh: *mut mpg123_handle_t,
                                            mut iohandle: *mut libc::c_void)
 -> libc::c_int {
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    mpg123_close(mh);
    if (*mh).rdat.r_read_handle.is_none() {
        (*mh).err = MPG123_BAD_CUSTOM_IO as libc::c_int;
        return MPG123_ERR as libc::c_int
    }
    return open_stream_handle(mh, iohandle);
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_open_feed(mut mh: *mut mpg123_handle_t)
 -> libc::c_int {
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    mpg123_close(mh);
    return open_feed(mh);
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_replace_reader_handle(mut mh:
                                                          *mut mpg123_handle_t,
                                                      mut fread:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut libc::c_void,
                                                                                      _:
                                                                                          *mut libc::c_void,
                                                                                      _:
                                                                                          size_t)
                                                                     ->
                                                                         mpg_ssize_t>,
                                                      mut lseek:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut libc::c_void,
                                                                                      _:
                                                                                          mpg_off_t,
                                                                                      _:
                                                                                          libc::c_int)
                                                                     ->
                                                                         mpg_off_t>,
                                                      mut fclose:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut libc::c_void)
                                                                     -> ()>)
 -> libc::c_int {
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    mpg123_close(mh);
    (*mh).rdat.r_read_handle = fread;
    (*mh).rdat.r_lseek_handle = lseek;
    (*mh).rdat.cleanup_handle = fclose;
    return MPG123_OK as libc::c_int;
}
// update decoding engine for
// a) a new choice of decoder
// b) a changed native format of the MPEG stream
// ... calls are only valid after parsing some MPEG frame!
#[no_mangle]
pub unsafe extern "C" fn decode_update(mut mh: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut native_rate: libc::c_long =
        0; // select the new output format based on given constraints.
    let mut b: libc::c_int = 0; // store for later...
    if (*mh).num < 0 as libc::c_int as libc::c_long {
        (*mh).err =
            MPG123_BAD_DECODER_SETUP as libc::c_int; // flexible (fixed) rate
        return MPG123_ERR as libc::c_int
    }
    (*mh).state_flags |= FRAME_FRESH_DECODER as libc::c_int;
    native_rate = frame_freq(mh);
    b = frame_output_format(mh);
    if b < 0 as libc::c_int { return MPG123_ERR as libc::c_int }
    if b == 1 as libc::c_int { (*mh).new_format = 1 as libc::c_int }
    if (*mh).af.rate as libc::c_long == native_rate {
        (*mh).down_sample = 0 as libc::c_int
    } else if (*mh).af.rate as libc::c_long == native_rate >> 1 as libc::c_int
     {
        (*mh).down_sample = 1 as libc::c_int
    } else if (*mh).af.rate as libc::c_long == native_rate >> 2 as libc::c_int
     {
        (*mh).down_sample = 2 as libc::c_int
    } else { (*mh).down_sample = 3 as libc::c_int }
    match (*mh).down_sample {
        0 | 1 | 2 => {
            (*mh).down_sample_sblimit =
                32 as libc::c_int >> (*mh).down_sample;
            // with downsampling I get less samples per frame
            (*mh).outblock =
                outblock_bytes(mh, (*mh).spf >> (*mh).down_sample) as size_t
        }
        _ => { }
    }
    if (*mh).p.flags & MPG123_FORCE_MONO as libc::c_int as libc::c_long == 0 {
        if (*mh).af.channels == 1 as libc::c_int {
            (*mh).single = 3 as libc::c_int
        } else { (*mh).single = -(1 as libc::c_int) }
    } else {
        (*mh).single =
            (((*mh).p.flags &
                  MPG123_FORCE_MONO as libc::c_int as libc::c_long) -
                 1 as libc::c_int as libc::c_long) as libc::c_int
    }
    if set_synth_functions(mh) != 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    // the needed size of output buffer may have changed.
    if frame_outbuffer(mh) != MPG123_OK as libc::c_int {
        return -(1 as libc::c_int)
    }
    do_rva(mh);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_safe_buffer() -> size_t {
    // real is the largest possible output
    return (::std::mem::size_of::<libc::c_float>() as
                libc::c_ulong).wrapping_mul(2 as libc::c_int as
                                                libc::c_ulong).wrapping_mul(1152
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_outblock(mut mh: *mut mpg123_handle_t)
 -> size_t {
    // try to be helpful and never return zero output block size.
    if !mh.is_null() && (*mh).outblock > 0 as libc::c_int as libc::c_ulong {
        return (*mh).outblock
    }
    return mpg123_safe_buffer();
}
// read in the next frame we actually want for decoding.
// this includes skipping/ignoring frames, in additon to skipping junk in the parser.
unsafe extern "C" fn get_next_frame(mut mh: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut change: libc::c_int = (*mh).decoder_change;
    // ensure we got proper decoder for ignoring frames.
	// header can be changed from seeking around. But be careful: Only after at
	// least one frame got read, decoder update makes sense.
    if (*mh).header_change > 1 as libc::c_int &&
           (*mh).num >= 0 as libc::c_int as libc::c_long {
        change = 1 as libc::c_int;
        (*mh).header_change = 0 as libc::c_int;
        if decode_update(mh) < 0 as libc::c_int {
            return MPG123_ERR as libc::c_int
        }
    }
    loop  {
        let mut b: libc::c_int = 0;
        // decode & discard some frame(s) before beginning.
        if (*mh).to_ignore != 0 && (*mh).num < (*mh).firstframe &&
               (*mh).num >= (*mh).ignoreframe {
            // decoder structure must be current! decode_update has been called before...
            (*mh).do_layer.expect("non-null function pointer")(mh);
            (*mh).buffer.fill = 0 as libc::c_int as size_t;
            (*mh).to_decode = 0 as libc::c_int;
            (*mh).to_ignore = (*mh).to_decode
        }
        // read new frame data; possibly breaking out here for MPG123_NEED_MORE.
        (*mh).to_decode =
            0 as
                libc::c_int; // that sets to_decode only if a full frame was read.
        b = read_frame(mh);
        if b == MPG123_NEED_MORE as libc::c_int {
            return MPG123_NEED_MORE as libc::c_int
            // need another call with data
        } else {
            if b <= 0 as libc::c_int {
                // more sophisticated error control?
                if b == 0 as libc::c_int ||
                       (*mh).rdat.filelen >= 0 as libc::c_int as libc::c_long
                           && (*mh).rdat.filepos == (*mh).rdat.filelen {
                    // we simply reached the end.
                    (*mh).track_frames =
                        (*mh).num + 1 as libc::c_int as libc::c_long;
                    return MPG123_DONE as libc::c_int
                }
                return MPG123_ERR as libc::c_int
                // some real error.
            }
        }
        // now, there should be new data to decode ... and also possibly new stream properties
        if (*mh).header_change > 1 as libc::c_int {
            change = 1 as libc::c_int;
            (*mh).header_change = 0 as libc::c_int;
            // need to update decoder structure right away since frame might need to
			// be decoded on next loop iteration for properly ignoring its output.
            if decode_update(mh) < 0 as libc::c_int {
                return MPG123_ERR as libc::c_int
            }
        }
        // now some accounting: Look at the numbers and decide if we want this frame.
        (*mh).playnum += 1;
        // plain skipping without decoding, only when frame is not ignored on next cycle.
        if !((*mh).num < (*mh).firstframe ||
                 (*mh).p.doublespeed != 0 &&
                     (*mh).playnum % (*mh).p.doublespeed != 0) {
            break ;
        }
        if !((*mh).to_ignore != 0 && (*mh).num < (*mh).firstframe &&
                 (*mh).num >= (*mh).ignoreframe) {
            frame_skip(mh);
        }
    }
    // if we reach this point, we got a new frame ready to be decoded.
	// all other situations resulted in returns from the loop.
    if change != 0 {
        (*mh).decoder_change = 0 as libc::c_int;
        if (*mh).fresh != 0 {
            let mut b_0: libc::c_int = 0 as libc::c_int;
            // Could be error, need for more, new format...
            frame_gapless_realinit(mh);
            frame_set_frameseek(mh, (*mh).num);
            (*mh).fresh = 0 as libc::c_int;
            if (*mh).num < (*mh).firstframe { b_0 = get_next_frame(mh) }
            if b_0 < 0 as libc::c_int { return b_0 }
        }
    }
    return MPG123_OK as libc::c_int;
}
unsafe extern "C" fn init_track(mut mh: *mut mpg123_handle_t) -> libc::c_int {
    if (*mh).num < 0 as libc::c_int as libc::c_long {
        // prepare offsets for gapless decoding.
        // could this possibly happen? With a real big gapless offset...
        // fresh track, need first frame for basic info.
        let mut b: libc::c_int = get_next_frame(mh);
        if b < 0 as libc::c_int { return b }
    }
    return 0 as libc::c_int;
}
// from internal sample number to external.
unsafe extern "C" fn sample_adjust(mut mh: *mut mpg123_handle_t,
                                   mut x: mpg_off_t) -> mpg_off_t {
    let mut s: mpg_off_t = 0;
    if (*mh).p.flags & MPG123_GAPLESS as libc::c_int as libc::c_long != 0 {
        // it's a bit tricky to do this computation for the padding samples.
		// they are not there on the outside.
        if x > (*mh).end_os {
            if x < (*mh).fullend_os {
                s = (*mh).end_os - (*mh).begin_os
            } else {
                s = x - ((*mh).fullend_os - (*mh).end_os + (*mh).begin_os)
            }
        } else { s = x - (*mh).begin_os }
    } else { s = x }
    return s;
}
// from external samples to internal
unsafe extern "C" fn sample_unadjust(mut mh: *mut mpg123_handle_t,
                                     mut x: mpg_off_t) -> mpg_off_t {
    let mut s: mpg_off_t = 0;
    if (*mh).p.flags & MPG123_GAPLESS as libc::c_int as libc::c_long != 0 {
        s = x + (*mh).begin_os;
        // there is a hole; we don't create sample positions in there.
		// jump from the end of the gapless track directly to after the padding.
        if s >= (*mh).end_os { s += (*mh).fullend_os - (*mh).end_os }
    } else { s = x }
    return s;
}
// take the buffer after a frame decode (strictly: it is the data from frame fr->num!) and cut samples out.
// fr->buffer.fill may then be smaller than before...
unsafe extern "C" fn frame_buffercheck(mut fr: *mut mpg123_handle_t) {
    // when we have no accurate position, gapless code does not make sense.
    if (*fr).state_flags & FRAME_ACCURATE as libc::c_int == 0 { return }
    // get a grip on dirty streams that start with a gapless header.
	// simply accept all data from frames that are too much,
	// they are supposedly attached to the stream after the fact.
    if (*fr).gapless_frames > 0 as libc::c_int as libc::c_long &&
           (*fr).num >= (*fr).gapless_frames {
        return
    }
    // important: We first cut samples from the end, then cut from beginning (including left-shift of the buffer).
	// this order works also for the case where firstframe == lastframe.
    // the last interesting (planned) frame: Only use some leading samples.
	// note a difference from the below: The last frame and offset are unchanges by seeks.
	// the lastoff keeps being valid.
    if (*fr).lastframe > -(1 as libc::c_int) as libc::c_long &&
           (*fr).num >= (*fr).lastframe {
        // there can be more than one frame of padding at the end, so we ignore the whole frame if we are beyond lastframe.
        let mut byteoff: mpg_off_t =
            if (*fr).num == (*fr).lastframe {
                samples_to_bytes(fr, (*fr).lastoff)
            } else { 0 as libc::c_int as libc::c_long };
        if (*fr).buffer.fill as mpg_off_t > byteoff {
            (*fr).buffer.fill = byteoff as size_t
        }
    }
    // the first interesting frame: Skip some leading samples.
    if (*fr).firstoff != 0 && (*fr).num == (*fr).firstframe {
        let mut byteoff_0: mpg_off_t = samples_to_bytes(fr, (*fr).firstoff);
        if (*fr).buffer.fill as mpg_off_t > byteoff_0 {
            (*fr).buffer.fill =
                ((*fr).buffer.fill as
                     libc::c_ulong).wrapping_sub(byteoff_0 as libc::c_ulong)
                    as size_t as size_t;
            if (*fr).own_buffer != 0 {
                (*fr).buffer.p = (*fr).buffer.data.offset(byteoff_0 as isize)
            } else {
                memmove((*fr).buffer.data as *mut libc::c_void,
                        (*fr).buffer.data.offset(byteoff_0 as isize) as
                            *const libc::c_void, (*fr).buffer.fill);
            }
        } else { (*fr).buffer.fill = 0 as libc::c_int as size_t }
        // we can only reach this frame again by seeking. And on seeking, firstoff will be recomputed.
		// so it is safe to null it here (and it makes the if() decision abort earlier).
        (*fr).firstoff = 0 as libc::c_int as mpg_off_t
    };
}
// not part of the api. This just decodes the frame and fills missing bits with zeroes.
// there can be frames that are broken and thus make do_layer() fail.
unsafe extern "C" fn decode_the_frame(mut fr: *mut mpg123_handle_t) {
    let mut needed_bytes: size_t =
        decoder_synth_bytes(fr, frame_expect_outsamples(fr)) as size_t;
    (*fr).clip +=
        (*fr).do_layer.expect("non-null function pointer")(fr) as
            libc::c_long;
    // there could be less data than promised.
	// also, then debugging, we look out for coding errors that could result in _more_ data than expected.
    if (*fr).buffer.fill < needed_bytes {
        // one could do a loop with individual samples instead... but zero is zero
		// actually, that is wrong: zero is mostly a series of null bytes,
		// but we have funny 8bit formats that have a different opinion on zero...
		// unsigned 16 or 32 bit formats are handled later.
        memset((*fr).buffer.data.offset((*fr).buffer.fill as isize) as
                   *mut libc::c_void, 0 as libc::c_int,
               needed_bytes.wrapping_sub((*fr).buffer.fill));
        (*fr).buffer.fill = needed_bytes
    }
    postprocess_buffer(fr);
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_read(mut mh: *mut mpg123_handle_t,
                                     mut out: *mut byte, mut size: size_t,
                                     mut done: *mut size_t) -> libc::c_int {
    return mpg123_decode(mh, 0 as *const byte, 0 as libc::c_int as size_t,
                         out, size, done);
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_feed(mut mh: *mut mpg123_handle_t,
                                     mut in_0: *const byte, mut size: size_t)
 -> libc::c_int {
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    if size > 0 as libc::c_int as libc::c_ulong {
        if !in_0.is_null() {
            if feed_more(mh, in_0, size as libc::c_long) != 0 as libc::c_int {
                return MPG123_ERR as libc::c_int
            } else {
                // the need for more data might have triggered an error.
				// this one is outdated now with the new data.
                if (*mh).err == MPG123_ERR_READER as libc::c_int {
                    (*mh).err = MPG123_OK as libc::c_int
                } // not just give error, give chance to get a status message.
                return MPG123_OK as libc::c_int
            }
        } else {
            (*mh).err = MPG123_NULL_BUFFER as libc::c_int;
            return MPG123_ERR as libc::c_int
        }
    }
    return MPG123_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_decode(mut mh: *mut mpg123_handle_t,
                                       mut inmemory: *const byte,
                                       mut inmemsize: size_t,
                                       mut outmemory: *mut byte,
                                       mut outmemsize: size_t,
                                       mut done: *mut size_t) -> libc::c_int {
    let mut ret: libc::c_int = MPG123_OK as libc::c_int;
    let mut mdone: size_t = 0 as libc::c_int as size_t;
    if !done.is_null() { *done = 0 as libc::c_int as size_t }
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    if inmemsize > 0 as libc::c_int as libc::c_ulong &&
           mpg123_feed(mh, inmemory, inmemsize) != MPG123_OK as libc::c_int {
        ret = MPG123_ERR as libc::c_int
    } else {
        if outmemory.is_null() { outmemsize = 0 as libc::c_int as size_t }
        while ret == MPG123_OK as libc::c_int {
            // decode a frame that has been read before.
		// this only happens when buffer is empty!
            if (*mh).to_decode != 0 {
                if (*mh).new_format != 0 {
                    (*mh).new_format = 0 as libc::c_int;
                    ret = MPG123_NEW_FORMAT as libc::c_int;
                    break ;
                } else if (*mh).buffer.size.wrapping_sub((*mh).buffer.fill) <
                              (*mh).outblock {
                    ret = MPG123_NO_SPACE as libc::c_int;
                    break ;
                } else {
                    decode_the_frame(mh);
                    (*mh).to_ignore = 0 as libc::c_int;
                    (*mh).to_decode = (*mh).to_ignore;
                    (*mh).buffer.p = (*mh).buffer.data;
                    frame_buffercheck(mh);
                }
            }
            if (*mh).buffer.fill != 0 {
                let mut a: libc::c_int =
                    if (*mh).buffer.fill > outmemsize.wrapping_sub(mdone) {
                        outmemsize.wrapping_sub(mdone)
                    } else { (*mh).buffer.fill } as libc::c_int;
                // copy (part of) the decoded data to the caller's buffer.
			// get what is needed - or just what is there
                memcpy(outmemory as *mut libc::c_void,
                       (*mh).buffer.p as *const libc::c_void,
                       a as libc::c_ulong);
                // less data in frame buffer, less needed, output pointer increase, more data given...
                (*mh).buffer.fill =
                    ((*mh).buffer.fill as
                         libc::c_ulong).wrapping_sub(a as libc::c_ulong) as
                        size_t as size_t;
                outmemory = outmemory.offset(a as isize);
                mdone =
                    (mdone as libc::c_ulong).wrapping_add(a as libc::c_ulong)
                        as size_t as size_t;
                (*mh).buffer.p = (*mh).buffer.p.offset(a as isize);
                if !(outmemsize > mdone) { break ; }
            } else {
                // if we didn't have data, get a new frame.
                let mut b: libc::c_int = get_next_frame(mh);
                if !(b < 0 as libc::c_int) { continue ; }
                ret = b;
                break ;
            }
        }
    }
    if !done.is_null() { *done = mdone }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_getformat(mut mh: *mut mpg123_handle_t,
                                          mut rate: *mut libc::c_int,
                                          mut channels: *mut libc::c_int,
                                          mut encoding: *mut libc::c_int)
 -> libc::c_int {
    let mut b: libc::c_int = 0;
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    b = init_track(mh);
    if b < 0 as libc::c_int { return b }
    if !rate.is_null() { *rate = (*mh).af.rate }
    if !channels.is_null() { *channels = (*mh).af.channels }
    if !encoding.is_null() { *encoding = (*mh).af.encoding }
    (*mh).new_format = 0 as libc::c_int;
    return MPG123_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_scan(mut mh: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut track_frames: mpg_off_t = 0 as libc::c_int as mpg_off_t;
    let mut track_samples: mpg_off_t = 0 as libc::c_int as mpg_off_t;
    let mut oldpos: mpg_off_t = 0;
    let mut b: libc::c_int = 0;
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    if (*mh).rdat.flags & 0x4 as libc::c_int == 0 {
        (*mh).err = MPG123_NO_SEEK as libc::c_int;
        return MPG123_ERR as libc::c_int
    }
    // scan through the _whole_ file, since the current position is no count but computed assuming constant samples per frame.
	// also, we can just keep the current buffer and seek settings. Just operate on input frames here.
    b = init_track(mh); // mh->num >= 0 !!
    if b < 0 as libc::c_int {
        if b == MPG123_DONE as libc::c_int { return MPG123_OK as libc::c_int }
        return MPG123_ERR as libc::c_int
        // must be error here, NEED_MORE is not for seekable streams.
    }
    oldpos = mpg123_tell(mh);
    b =
        (*(*mh).rd).seek_frame.expect("non-null function pointer")(mh,
                                                                   0 as
                                                                       libc::c_int
                                                                       as
                                                                       mpg_off_t);
    if b < 0 as libc::c_int || (*mh).num != 0 as libc::c_int as libc::c_long {
        return MPG123_ERR as libc::c_int
    }
    // one frame must be there now.
    track_frames = 1 as libc::c_int as mpg_off_t; // internal samples.
    track_samples = (*mh).spf;
    // do not increment mh->track_frames in the loop as tha would confuse Frankenstein detection.
    while read_frame(mh) == 1 as libc::c_int {
        track_samples += (*mh).spf;
        track_frames += 1
    }
    (*mh).track_frames = track_frames;
    (*mh).track_samples = track_samples;
    // also, think about usefulness of that extra value track_samples ...
	// it could be used for consistency checking.
    if (*mh).p.flags & MPG123_GAPLESS as libc::c_int as libc::c_long != 0 {
        frame_gapless_update(mh, (*mh).track_samples);
    }
    return if mpg123_seek(mh, oldpos, 0 as libc::c_int) >=
                  0 as libc::c_int as libc::c_long {
               MPG123_OK as libc::c_int
           } else { MPG123_ERR as libc::c_int };
}
// now, where are we? We need to know the last decoded frame... and what's left of it in buffer.
// the current frame number can mean the last decoded frame or the to-be-decoded frame.
// if mh->to_decode, then mh->num frames have been decoded, the frame mh->num now coming next.
// if not, we have the possibility of mh->num+1 frames being decoded or nothing at all.
// then, there is firstframe...when we didn't reach it yet, then the next data will come from there.
// mh->num starts with -1
#[no_mangle]
pub unsafe extern "C" fn mpg123_tell(mut mh: *mut mpg123_handle_t)
 -> mpg_off_t {
    let mut pos: mpg_off_t = 0 as libc::c_int as mpg_off_t;
    if mh.is_null() { return MPG123_ERR as libc::c_int as mpg_off_t }
    if (*mh).num < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as mpg_off_t
    }
    // now we have all the info at hand.
    if (*mh).num < (*mh).firstframe ||
           (*mh).num == (*mh).firstframe && (*mh).to_decode != 0 {
        // we are at the beginning, expect output from firstframe on.
        pos = frame_outs(mh, (*mh).firstframe);
        pos += (*mh).firstoff
    } else if (*mh).to_decode != 0 {
        // we start fresh with this frame. Buffer should be empty, but we make sure to count it in.
        pos =
            frame_outs(mh, (*mh).num) -
                bytes_to_samples(mh, (*mh).buffer.fill as mpg_off_t)
    } else {
        // we serve what we have in buffer and then the beginning of next frame...
        pos =
            frame_outs(mh, (*mh).num + 1 as libc::c_int as libc::c_long) -
                bytes_to_samples(mh, (*mh).buffer.fill as mpg_off_t)
    }
    // substract padding and delay from the beginning. */
    pos = sample_adjust(mh, pos);
    // negative sample offsets are not right, less than nothing is still nothing.
    return if pos > 0 as libc::c_int as libc::c_long {
               pos
           } else { 0 as libc::c_int as libc::c_long };
}
unsafe extern "C" fn do_the_seek(mut mh: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut fnum: mpg_off_t =
        if (*mh).ignoreframe < 0 as libc::c_int as libc::c_long {
            0 as libc::c_int as libc::c_long
        } else { (*mh).ignoreframe };
    let mut b: libc::c_int = 0;
    (*mh).buffer.fill = 0 as libc::c_int as size_t;
    // If we are inside the ignoreframe - firstframe window,
	// we may get away without actual seeking.
    if (*mh).num < (*mh).firstframe {
        (*mh).to_decode =
            0 as
                libc::c_int; // In any case, don't decode the current frame, perhaps ignore instead.
        if (*mh).num > fnum { return MPG123_OK as libc::c_int }
    }
    // if we are already there, we are fine either for decoding or for ignoring.
    if (*mh).num == fnum && ((*mh).to_decode != 0 || fnum < (*mh).firstframe)
       {
        return MPG123_OK as libc::c_int
    }
    // we have the frame before... just go ahead as normal.
    if (*mh).num == fnum - 1 as libc::c_int as libc::c_long {
        (*mh).to_decode = 0 as libc::c_int;
        return MPG123_OK as libc::c_int
    }
    // OK, real seeking follows... clear buffers and go for it.
    frame_buffers_reset(mh);
    b = (*(*mh).rd).seek_frame.expect("non-null function pointer")(mh, fnum);
    if (*mh).header_change > 1 as libc::c_int {
        if decode_update(mh) < 0 as libc::c_int {
            return MPG123_ERR as libc::c_int
        }
        (*mh).header_change = 0 as libc::c_int
    }
    if b < 0 as libc::c_int { return b }
    // Only mh->to_ignore is TRUE.
    if (*mh).num < (*mh).firstframe {
        (*mh).to_decode = 0 as libc::c_int
    } // adjusted samples
    (*mh).playnum = (*mh).num;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_seek(mut mh: *mut mpg123_handle_t,
                                     mut sampleoff: mpg_off_t,
                                     mut whence: libc::c_int) -> mpg_off_t {
    let mut pos: mpg_off_t = 0;
    let mut b: libc::c_int = 0;
    pos = mpg123_tell(mh);
    // pos < 0 also can mean that simply a former seek failed at the lower levels.
	// in that case, we only allow absolute seeks.
    if pos < 0 as libc::c_int as libc::c_long && whence != 0 as libc::c_int {
        // unless we got the obvious error of NULL handle,
		// this is a special seek failure.
        if !mh.is_null() { (*mh).err = MPG123_NO_RELSEEK as libc::c_int }
        return MPG123_ERR as libc::c_int as mpg_off_t
    }
    b = init_track(mh);
    if b < 0 as libc::c_int { return b as mpg_off_t }
    match whence {
        1 => { pos += sampleoff }
        0 => { pos = sampleoff }
        2 => {
            // when we do not know the end already, we can try to find it.
            if (*mh).track_frames < 1 as libc::c_int as libc::c_long &&
                   (*mh).rdat.flags & 0x4 as libc::c_int != 0 {
                mpg123_scan(mh);
            }
            if (*mh).track_frames > 0 as libc::c_int as libc::c_long {
                pos =
                    sample_adjust(mh, frame_outs(mh, (*mh).track_frames)) -
                        sampleoff
            } else if (*mh).end_os > 0 as libc::c_int as libc::c_long {
                pos = sample_adjust(mh, (*mh).end_os) - sampleoff
            } else {
                (*mh).err = MPG123_NO_SEEK_FROM_END as libc::c_int;
                return MPG123_ERR as libc::c_int as mpg_off_t
            }
        }
        _ => {
            (*mh).err = MPG123_BAD_WHENCE as libc::c_int;
            return MPG123_ERR as libc::c_int as mpg_off_t
        }
    }
    if pos < 0 as libc::c_int as libc::c_long {
        pos = 0 as libc::c_int as mpg_off_t
    }
    // pos now holds the wanted sample offset in adjusted samples
    frame_set_seek(mh, sample_unadjust(mh, pos));
    pos = do_the_seek(mh) as mpg_off_t;
    if pos < 0 as libc::c_int as libc::c_long { return pos }
    return mpg123_tell(mh);
}
static mut mpg123_error: [*const libc::c_char; 44] =
    [b"No error... (code 0)\x00" as *const u8 as *const libc::c_char,
     b"Unable to set up output format! (code 1)\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid channel number specified. (code 2)\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid sample rate specified. (code 3)\x00" as *const u8 as
         *const libc::c_char,
     b"Unable to allocate memory for 16 to 8 converter table! (code 4)\x00" as
         *const u8 as *const libc::c_char,
     b"Bad parameter id! (code 5)\x00" as *const u8 as *const libc::c_char,
     b"Bad buffer given -- invalid pointer or too small size. (code 6)\x00" as
         *const u8 as *const libc::c_char,
     b"Out of memory -- some malloc() failed. (code 7)\x00" as *const u8 as
         *const libc::c_char,
     b"You didn\'t initialize the library! (code 8)\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid decoder choice. (code 9)\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid mpg123 handle. (code 10)\x00" as *const u8 as
         *const libc::c_char,
     b"Unable to initialize frame buffers (out of memory?)! (code 11)\x00" as
         *const u8 as *const libc::c_char,
     b"Invalid RVA mode. (code 12)\x00" as *const u8 as *const libc::c_char,
     b"This build doesn\'t support gapless decoding. (code 13)\x00" as
         *const u8 as *const libc::c_char,
     b"Not enough buffer space. (code 14)\x00" as *const u8 as
         *const libc::c_char,
     b"Incompatible numeric data types. (code 15)\x00" as *const u8 as
         *const libc::c_char,
     b"Bad equalizer band. (code 16)\x00" as *const u8 as *const libc::c_char,
     b"Null pointer given where valid storage address needed. (code 17)\x00"
         as *const u8 as *const libc::c_char,
     b"Error reading the stream. (code 18)\x00" as *const u8 as
         *const libc::c_char,
     b"Cannot seek from end (end is not known). (code 19)\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid \'whence\' for seek function. (code 20)\x00" as *const u8 as
         *const libc::c_char,
     b"Build does not support stream timeouts. (code 21)\x00" as *const u8 as
         *const libc::c_char,
     b"File access error. (code 22)\x00" as *const u8 as *const libc::c_char,
     b"Seek not supported by stream. (code 23)\x00" as *const u8 as
         *const libc::c_char,
     b"No stream opened. (code 24)\x00" as *const u8 as *const libc::c_char,
     b"Bad parameter handle. (code 25)\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid parameter addresses for index retrieval. (code 26)\x00" as
         *const u8 as *const libc::c_char,
     b"Lost track in the bytestream and did not attempt resync. (code 27)\x00"
         as *const u8 as *const libc::c_char,
     b"Failed to find valid MPEG data within limit on resync. (code 28)\x00"
         as *const u8 as *const libc::c_char,
     b"No 8bit encoding possible. (code 29)\x00" as *const u8 as
         *const libc::c_char,
     b"Stack alignment is not good. (code 30)\x00" as *const u8 as
         *const libc::c_char,
     b"You gave me a NULL buffer? (code 31)\x00" as *const u8 as
         *const libc::c_char,
     b"File position is screwed up, please do an absolute seek (code 32)\x00"
         as *const u8 as *const libc::c_char,
     b"Inappropriate NULL-pointer provided.\x00" as *const u8 as
         *const libc::c_char,
     b"Bad key value given.\x00" as *const u8 as *const libc::c_char,
     b"There is no frame index (disabled in this build).\x00" as *const u8 as
         *const libc::c_char,
     b"Frame index operation failed.\x00" as *const u8 as *const libc::c_char,
     b"Decoder setup failed (invalid combination of settings?)\x00" as
         *const u8 as *const libc::c_char,
     b"Feature not in this build.\x00" as *const u8 as *const libc::c_char,
     b"Some bad value has been provided.\x00" as *const u8 as
         *const libc::c_char,
     b"Low-level seeking has failed (call to lseek(), usually).\x00" as
         *const u8 as *const libc::c_char,
     b"Custom I/O obviously not prepared.\x00" as *const u8 as
         *const libc::c_char,
     b"Overflow in LFS (large file support) conversion.\x00" as *const u8 as
         *const libc::c_char,
     b"Overflow in integer conversion.\x00" as *const u8 as
         *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn mpg123_plain_strerror(mut errcode: libc::c_int)
 -> *const libc::c_char {
    if errcode >= 0 as libc::c_int &&
           (errcode as libc::c_ulong) <
               (::std::mem::size_of::<[*const libc::c_char; 44]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong) {
        return mpg123_error[errcode as usize]
    }
    match errcode {
        -1 => {
            return b"A generic mpg123 error.\x00" as *const u8 as
                       *const libc::c_char
        }
        -12 => {
            return b"Message: I am done with this track.\x00" as *const u8 as
                       *const libc::c_char
        }
        -10 => {
            return b"Message: Feed me more input data!\x00" as *const u8 as
                       *const libc::c_char
        }
        -11 => {
            return b"Message: Prepare for a changed audio format (query the new one)!\x00"
                       as *const u8 as *const libc::c_char
        }
        _ => {
            return b"I have no idea - an unknown error code!\x00" as *const u8
                       as *const libc::c_char
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_error(mut mh: *mut mpg123_handle_t)
 -> *const libc::c_char {
    if mh.is_null() {
        return mpg123_plain_strerror(MPG123_BAD_HANDLE as libc::c_int)
    }
    return mpg123_plain_strerror((*mh).err);
}
