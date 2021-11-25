#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn set_pointer(fr: *mut mpg123_handle_t, backstep: libc::c_long);
    #[no_mangle]
    fn invalidate_format(af: *mut audioformat_t);
    #[no_mangle]
    fn mpg123_fmt_all(mp: *mut mpg123_parm_t) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn fi_init(fi: *mut frame_index_t);
    #[no_mangle]
    fn fi_exit(fi: *mut frame_index_t);
    #[no_mangle]
    fn fi_resize(fi: *mut frame_index_t, newsize: size_t) -> libc::c_int;
    #[no_mangle]
    fn fi_reset(fi: *mut frame_index_t);
    #[no_mangle]
    fn bc_prepare(_: *mut bufferchain_t, pool_size: size_t, bufblock: size_t);
    #[no_mangle]
    fn bc_cleanup(_: *mut bufferchain_t);
    #[no_mangle]
    fn open_bad(fr: *mut mpg123_handle_t);
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
pub const MPG123_ERR: mpg123_errors = -1;
pub const MPG123_OK: mpg123_errors = 0;
pub const MPG123_CBR: mpg123_vbr = 0;
pub const FRAME_ACCURATE: frame_state_flags = 1;
pub const MPG123_AUTO_RESAMPLE: mpg123_param_flags = 32768;
pub const MPG123_GAPLESS: mpg123_param_flags = 64;
pub const MPG123_OUT_OF_MEM: mpg123_errors = 7;
pub const MPG123_BAD_BUFFER: mpg123_errors = 6;
pub const MPG123_FUZZY: mpg123_param_flags = 512;
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
pub const MPG123_BAD_PARAM: mpg123_errors = 5;
pub const MPG123_ERR_16TO8TABLE: mpg123_errors = 4;
pub const MPG123_BAD_RATE: mpg123_errors = 3;
pub const MPG123_BAD_CHANNEL: mpg123_errors = 2;
pub const MPG123_BAD_OUTFORMAT: mpg123_errors = 1;
pub const MPG123_NEED_MORE: mpg123_errors = -10;
pub const MPG123_NEW_FORMAT: mpg123_errors = -11;
pub const MPG123_DONE: mpg123_errors = -12;
pub type mpg123_param_flags = libc::c_uint;
pub const MPG123_IGNORE_INFOFRAME: mpg123_param_flags = 16384;
pub const MPG123_IGNORE_STREAMLENGTH: mpg123_param_flags = 4096;
pub const MPG123_SEEKBUFFER: mpg123_param_flags = 256;
pub const MPG123_NO_RESYNC: mpg123_param_flags = 128;
pub const MPG123_QUIET: mpg123_param_flags = 32;
pub const MPG123_FORCE_STEREO: mpg123_param_flags = 8;
pub const MPG123_MONO_MIX: mpg123_param_flags = 4;
pub const MPG123_MONO_RIGHT: mpg123_param_flags = 2;
pub const MPG123_MONO_LEFT: mpg123_param_flags = 1;
pub const MPG123_FORCE_MONO: mpg123_param_flags = 7;
pub type frame_state_flags = libc::c_uint;
pub const FRAME_FRESH_DECODER: frame_state_flags = 4;
pub const FRAME_FRANKENSTEIN: frame_state_flags = 2;
pub type mpg123_vbr = libc::c_uint;
pub const MPG123_ABR: mpg123_vbr = 2;
pub const MPG123_VBR: mpg123_vbr = 1;
/*
frame.c - compact version of famous library mpg123
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
unsafe extern "C" fn aligned_pointer(mut base: *mut libc::c_void,
                                     mut alignment: uint)
 -> *mut libc::c_void {
    // work in unsigned integer realm, explicitly.
	// tricking the compiler into integer operations like % by invoking base-NULL is dangerous:
	// it results into ptrdiff_t, which gets negative on big addresses. Big screw up, that.
	// i try to do it "properly" here: Casting only to size_t and no artihmethic with void*.
    let mut baseval: size_t =
        base as *mut libc::c_char as
            size_t; // that's good  for layer 3 ISO compliance bitstream.
    let mut aoff: size_t = baseval.wrapping_rem(alignment as libc::c_ulong);
    if aoff != 0 {
        return (base as
                    *mut libc::c_char).offset(alignment as
                                                  isize).offset(-(aoff as
                                                                      isize))
                   as *mut libc::c_void
    }
    return base;
}
unsafe extern "C" fn frame_default_parm(mut mp: *mut mpg123_parm_t) {
    (*mp).outscale = 1.0f64;
    (*mp).flags = 0 as libc::c_int as libc::c_long;
    (*mp).flags |= MPG123_GAPLESS as libc::c_int as libc::c_long;
    (*mp).flags |= MPG123_AUTO_RESAMPLE as libc::c_int as libc::c_long;
    (*mp).down_sample = 0 as libc::c_int;
    (*mp).rva = 0 as libc::c_int;
    (*mp).halfspeed = 0 as libc::c_int as libc::c_long;
    (*mp).doublespeed = 0 as libc::c_int as libc::c_long;
    (*mp).verbose = 0 as libc::c_int;
    (*mp).timeout = 0 as libc::c_int as libc::c_long;
    (*mp).resync_limit = 1024 as libc::c_int as libc::c_long;
    (*mp).index_size = 1000 as libc::c_int as libc::c_long;
    (*mp).preframes = 4 as libc::c_int as libc::c_long;
    mpg123_fmt_all(mp);
    // default of keeping some 4K buffers at hand, should cover the "usual" use case
	// (using 16K pipe buffers as role model).
    (*mp).feedpool = 5 as libc::c_int as libc::c_long;
    (*mp).feedbuffer = 4096 as libc::c_int as libc::c_long;
}
// reset everythign except dynamic memory.
unsafe extern "C" fn frame_fixed_reset(mut fr: *mut mpg123_handle_t) {
    open_bad(fr); // this will be set before decoding!
    (*fr).to_decode = 0 as libc::c_int; // the usual bo
    (*fr).to_ignore =
        0 as libc::c_int; // here or indeed only on first-time init?
    (*fr).metaflags = 0 as libc::c_int;
    (*fr).outblock = 0 as libc::c_int as size_t;
    (*fr).num = -(1 as libc::c_int) as mpg_off_t;
    (*fr).input_offset = -(1 as libc::c_int) as mpg_off_t;
    (*fr).playnum = -(1 as libc::c_int) as mpg_off_t;
    (*fr).state_flags = FRAME_ACCURATE as libc::c_int;
    (*fr).silent_resync = 0 as libc::c_int as libc::c_char;
    (*fr).audio_start = 0 as libc::c_int as mpg_off_t;
    (*fr).clip = 0 as libc::c_int as libc::c_long;
    (*fr).oldhead = 0 as libc::c_int as ulong;
    (*fr).firsthead = 0 as libc::c_int as ulong;
    (*fr).vbr = MPG123_CBR as libc::c_int;
    (*fr).abr_rate = 0 as libc::c_int;
    (*fr).track_frames = 0 as libc::c_int as mpg_off_t;
    (*fr).track_samples = -(1 as libc::c_int) as mpg_off_t;
    (*fr).framesize = 0 as libc::c_int;
    (*fr).mean_frames = 0 as libc::c_int as mpg_off_t;
    (*fr).mean_framesize = 0 as libc::c_int as libc::c_double;
    (*fr).freesize = 0 as libc::c_int;
    (*fr).lastscale = -(1 as libc::c_int) as libc::c_double;
    (*fr).rva.level[0 as libc::c_int as usize] = -(1 as libc::c_int);
    (*fr).rva.level[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*fr).rva.gain[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    (*fr).rva.gain[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    (*fr).rva.peak[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    (*fr).rva.peak[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    (*fr).fsizeold = 0 as libc::c_int;
    (*fr).firstframe = 0 as libc::c_int as mpg_off_t;
    (*fr).ignoreframe = (*fr).firstframe - (*fr).p.preframes;
    (*fr).header_change = 0 as libc::c_int;
    (*fr).lastframe = -(1 as libc::c_int) as mpg_off_t;
    (*fr).fresh = 1 as libc::c_int;
    (*fr).new_format = 0 as libc::c_int;
    frame_gapless_init(fr, -(1 as libc::c_int) as mpg_off_t,
                       0 as libc::c_int as mpg_off_t,
                       0 as libc::c_int as mpg_off_t);
    (*fr).lastoff = 0 as libc::c_int as mpg_off_t;
    (*fr).firstoff = 0 as libc::c_int as mpg_off_t;
    (*fr).bo = 1 as libc::c_int;
    (*fr).halfphase = 0 as libc::c_int;
    (*fr).error_protection = 0 as libc::c_int;
    (*fr).freeformat_framesize = -(1 as libc::c_int) as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn frame_index_setup(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut ret: libc::c_int = MPG123_ERR as libc::c_int;
    if (*fr).p.index_size >= 0 as libc::c_int as libc::c_long {
        // simple fixed index.
        (*fr).index.grow_size = 0 as libc::c_int as size_t;
        ret = fi_resize(&mut (*fr).index, (*fr).p.index_size as size_t)
    } else {
        // a growing index. we give it a start, though.
        (*fr).index.grow_size = -(*fr).p.index_size as size_t;
        if (*fr).index.size < (*fr).index.grow_size {
            ret = fi_resize(&mut (*fr).index, (*fr).index.grow_size)
        } else { ret = MPG123_OK as libc::c_int }
        // we have minimal size already... and since growing is OK...
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn frame_init_par(mut fr: *mut mpg123_handle_t,
                                        mut mp: *mut mpg123_parm_t) {
    (*fr).own_buffer = (0 as libc::c_int == 0) as libc::c_int;
    (*fr).buffer.data = 0 as *mut byte;
    (*fr).buffer.rdata = 0 as *mut byte;
    (*fr).buffer.fill = 0 as libc::c_int as size_t;
    (*fr).buffer.size = 0 as libc::c_int as size_t;
    (*fr).rawbuffs = 0 as *mut byte;
    (*fr).rawbuffss = 0 as libc::c_int;
    (*fr).rawdecwin = 0 as *mut byte;
    (*fr).rawdecwins = 0 as libc::c_int;
    (*fr).layerscratch = 0 as *mut libc::c_float;
    (*fr).xing_toc = 0 as *mut byte;
    // unnecessary: fr->buffer.size = fr->buffer.fill = 0;
	// frame_outbuffer is missing...
	// frame_buffers is missing... that one needs cpu opt setting!
	// after these... frame_reset is needed before starting full decode
    invalidate_format(&mut (*fr).af); // initialize to silence harmless errors when debugging.
    (*fr).rdat.r_read =
        None; // reset only the fixed data, dynamic buffers are not there yet!
    (*fr).rdat.r_lseek = None;
    (*fr).rdat.iohandle = 0 as *mut libc::c_void;
    (*fr).rdat.r_read_handle = None;
    (*fr).rdat.r_lseek_handle = None;
    (*fr).rdat.cleanup_handle = None;
    (*fr).wrapperdata = 0 as *mut libc::c_void;
    (*fr).wrapperclean = None;
    (*fr).decoder_change = 1 as libc::c_int;
    (*fr).err = MPG123_OK as libc::c_int;
    if mp.is_null() {
        frame_default_parm(&mut (*fr).p);
    } else {
        memcpy(&mut (*fr).p as *mut mpg123_parm_t as *mut libc::c_void,
               mp as *const libc::c_void,
               ::std::mem::size_of::<mpg123_parm_t>() as libc::c_ulong);
    }
    bc_prepare(&mut (*fr).rdat.buffer, (*fr).p.feedpool as size_t,
               (*fr).p.feedbuffer as size_t);
    (*fr).down_sample = 0 as libc::c_int;
    frame_fixed_reset(fr);
    (*fr).synth = None;
    (*fr).synth_mono = None;
    (*fr).make_decode_tables = None;
    fi_init(&mut (*fr).index);
    frame_index_setup(fr);
    // apply the size setting.
}
unsafe extern "C" fn frame_decode_buffers_reset(mut fr:
                                                    *mut mpg123_handle_t) {
    if !(*fr).rawbuffs.is_null() {
        /* memset(NULL, 0, 0) not desired */
        memset((*fr).rawbuffs as *mut libc::c_void, 0 as libc::c_int,
               (*fr).rawbuffss as libc::c_ulong); // for 16-byte alignment
    };
}
#[no_mangle]
pub unsafe extern "C" fn frame_buffers(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut buffssize: libc::c_int = 4352 as libc::c_int;
    buffssize += 15 as libc::c_int;
    if !(*fr).rawbuffs.is_null() && (*fr).rawbuffss != buffssize {
        free((*fr).rawbuffs as *mut libc::c_void);
        (*fr).rawbuffs = 0 as *mut byte
    }
    if (*fr).rawbuffs.is_null() {
        (*fr).rawbuffs = malloc(buffssize as libc::c_ulong) as *mut byte
    }
    if (*fr).rawbuffs.is_null() { return -(1 as libc::c_int) }
    (*fr).rawbuffss = buffssize;
    (*fr).short_buffs[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        aligned_pointer((*fr).rawbuffs as *mut libc::c_void,
                        16 as libc::c_int as uint) as *mut libc::c_short;
    (*fr).short_buffs[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*fr).short_buffs[0 as libc::c_int as
                              usize][0 as libc::c_int as
                                         usize].offset(0x110 as libc::c_int as
                                                           isize);
    (*fr).short_buffs[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*fr).short_buffs[0 as libc::c_int as
                              usize][1 as libc::c_int as
                                         usize].offset(0x110 as libc::c_int as
                                                           isize);
    (*fr).short_buffs[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*fr).short_buffs[1 as libc::c_int as
                              usize][0 as libc::c_int as
                                         usize].offset(0x110 as libc::c_int as
                                                           isize);
    (*fr).float_buffs[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        aligned_pointer((*fr).rawbuffs as *mut libc::c_void,
                        16 as libc::c_int as uint) as *mut libc::c_float;
    (*fr).float_buffs[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*fr).float_buffs[0 as libc::c_int as
                              usize][0 as libc::c_int as
                                         usize].offset(0x110 as libc::c_int as
                                                           isize);
    (*fr).float_buffs[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*fr).float_buffs[0 as libc::c_int as
                              usize][1 as libc::c_int as
                                         usize].offset(0x110 as libc::c_int as
                                                           isize);
    (*fr).float_buffs[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*fr).float_buffs[1 as libc::c_int as
                              usize][0 as libc::c_int as
                                         usize].offset(0x110 as libc::c_int as
                                                           isize);
    // now the different decwins... all of the same size, actually
	// the MMX ones want 32byte alignment, which I'll try to ensure manually
    let mut decwin_size: libc::c_int =
        ((512 as libc::c_int + 32 as libc::c_int) as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                             as libc::c_ulong) as libc::c_int;
    // hm, that's basically realloc() ...
    if !(*fr).rawdecwin.is_null() && (*fr).rawdecwins != decwin_size {
        free((*fr).rawdecwin as *mut libc::c_void);
        (*fr).rawdecwin = 0 as *mut byte
    }
    if (*fr).rawdecwin.is_null() {
        (*fr).rawdecwin = malloc(decwin_size as libc::c_ulong) as *mut byte
    }
    if (*fr).rawdecwin.is_null() { return -(1 as libc::c_int) }
    (*fr).rawdecwins = decwin_size;
    (*fr).decwin = (*fr).rawdecwin as *mut libc::c_float;
    // layer scratch buffers are of compile-time fixed size, so allocate only once.
    if (*fr).layerscratch.is_null() {
        // allocate specific layer3 buffers
        let mut scratchsize: size_t = 0 as libc::c_int as size_t; // hybrid_in
        let mut scratcher: *mut libc::c_float =
            0 as *mut libc::c_float; // hybrid_out
        scratchsize =
            (scratchsize as
                 libc::c_ulong).wrapping_add((::std::mem::size_of::<libc::c_float>()
                                                  as
                                                  libc::c_ulong).wrapping_mul(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong).wrapping_mul(32
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_mul(18
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong))
                as size_t as size_t;
        scratchsize =
            (scratchsize as
                 libc::c_ulong).wrapping_add((::std::mem::size_of::<libc::c_float>()
                                                  as
                                                  libc::c_ulong).wrapping_mul(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong).wrapping_mul(18
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_mul(32
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong))
                as size_t as size_t;
        (*fr).layerscratch =
            malloc(scratchsize.wrapping_add(63 as libc::c_int as
                                                libc::c_ulong)) as
                *mut libc::c_float;
        if (*fr).layerscratch.is_null() { return -(1 as libc::c_int) }
        // get aligned part of the memory, then divide it up.
        scratcher =
            aligned_pointer((*fr).layerscratch as *mut libc::c_void,
                            64 as libc::c_int as uint) as *mut libc::c_float;
        // those funky pointer casts silence compilers...
		// One might change the code at hand to really just use 1D arrays,
		// but in practice, that would not make a (positive) difference.
        (*fr).layer3.hybrid_in = scratcher as *mut [[libc::c_float; 18]; 32];
        scratcher =
            scratcher.offset((2 as libc::c_int * 32 as libc::c_int *
                                  18 as libc::c_int) as isize);
        (*fr).layer3.hybrid_out = scratcher as *mut [[libc::c_float; 32]; 18];
        scratcher =
            scratcher.offset((2 as libc::c_int * 18 as libc::c_int *
                                  32 as libc::c_int) as isize)
    }
    // only reset the buffers we created just now.
    frame_decode_buffers_reset(fr); // hm, reset buffer fill... did we do a flush?
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn frame_buffers_reset(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    (*fr).buffer.fill = 0 as libc::c_int as size_t;
    (*fr).bsnum = 0 as libc::c_int;
    // wondering: could it be actually _wanted_ to retain buffer contents over different files? (special gapless / cut stuff)
    (*fr).bsbuf = (*fr).bsspace[1 as libc::c_int as usize].as_mut_ptr();
    (*fr).bsbufold = (*fr).bsbuf;
    (*fr).bitreservoir = 0 as libc::c_int as uint;
    frame_decode_buffers_reset(fr);
    memset((*fr).bsspace.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (2 as libc::c_int * (3456 as libc::c_int + 512 as libc::c_int)) as
               libc::c_ulong);
    memset((*fr).ssave.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           34 as libc::c_int as libc::c_ulong);
    (*fr).hybrid_blc[1 as libc::c_int as usize] = 0 as libc::c_int;
    (*fr).hybrid_blc[0 as libc::c_int as usize] =
        (*fr).hybrid_blc[1 as libc::c_int as usize];
    memset((*fr).hybrid_block.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<libc::c_float>() as
                libc::c_ulong).wrapping_mul(2 as libc::c_int as
                                                libc::c_ulong).wrapping_mul(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong).wrapping_mul(32
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_ulong).wrapping_mul(18
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                libc::c_ulong));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn frame_init(mut fr: *mut mpg123_handle_t) {
    frame_init_par(fr, 0 as *mut mpg123_parm_t);
}
#[no_mangle]
pub unsafe extern "C" fn frame_outbuffer(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut size: size_t = (*fr).outblock;
    if (*fr).own_buffer == 0 {
        if (*fr).buffer.size < size {
            (*fr).err = MPG123_BAD_BUFFER as libc::c_int;
            return MPG123_ERR as libc::c_int
        }
    }
    if !(*fr).buffer.rdata.is_null() && (*fr).buffer.size != size {
        free((*fr).buffer.rdata as *mut libc::c_void);
        (*fr).buffer.rdata = 0 as *mut byte
    }
    (*fr).buffer.size = size;
    (*fr).buffer.data = 0 as *mut byte;
    // be generous: use 16 byte alignment
    if (*fr).buffer.rdata.is_null() {
        (*fr).buffer.rdata =
            malloc((*fr).buffer.size.wrapping_add(15 as libc::c_int as
                                                      libc::c_ulong)) as
                *mut byte
    }
    if (*fr).buffer.rdata.is_null() {
        (*fr).err = MPG123_OUT_OF_MEM as libc::c_int;
        return MPG123_ERR as libc::c_int
    }
    (*fr).buffer.data =
        aligned_pointer((*fr).buffer.rdata as *mut libc::c_void,
                        16 as libc::c_int as uint) as *mut byte;
    (*fr).own_buffer = (0 as libc::c_int == 0) as libc::c_int;
    (*fr).buffer.fill = 0 as libc::c_int as size_t;
    return MPG123_OK as libc::c_int;
}
unsafe extern "C" fn frame_free_toc(mut fr: *mut mpg123_handle_t) {
    if !(*fr).xing_toc.is_null() {
        free((*fr).xing_toc as *mut libc::c_void);
        (*fr).xing_toc = 0 as *mut byte
    };
}
// Just copy the Xing TOC over...
#[no_mangle]
pub unsafe extern "C" fn frame_fill_toc(mut fr: *mut mpg123_handle_t,
                                        mut in_0: *mut byte) -> libc::c_int {
    if (*fr).xing_toc.is_null() {
        (*fr).xing_toc =
            malloc(100 as libc::c_int as libc::c_ulong) as *mut byte
    }
    if !(*fr).xing_toc.is_null() {
        memcpy((*fr).xing_toc as *mut libc::c_void,
               in_0 as *const libc::c_void,
               100 as libc::c_int as libc::c_ulong);
        return (0 as libc::c_int == 0) as libc::c_int
    }
    return 0 as libc::c_int;
}
// prepare the handle for a new track.
// reset variables, buffers...
#[no_mangle]
pub unsafe extern "C" fn frame_reset(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    frame_buffers_reset(fr);
    frame_fixed_reset(fr);
    frame_free_toc(fr);
    fi_reset(&mut (*fr).index);
    return 0 as libc::c_int;
}
unsafe extern "C" fn frame_free_buffers(mut fr: *mut mpg123_handle_t) {
    if !(*fr).rawbuffs.is_null() {
        free((*fr).rawbuffs as *mut libc::c_void);
    }
    (*fr).rawbuffs = 0 as *mut byte;
    (*fr).rawbuffss = 0 as libc::c_int;
    if !(*fr).rawdecwin.is_null() {
        free((*fr).rawdecwin as *mut libc::c_void);
    }
    (*fr).rawdecwin = 0 as *mut byte;
    (*fr).rawdecwins = 0 as libc::c_int;
    if !(*fr).layerscratch.is_null() {
        free((*fr).layerscratch as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn frame_exit(mut fr: *mut mpg123_handle_t) {
    if !(*fr).buffer.rdata.is_null() {
        free((*fr).buffer.rdata as *mut libc::c_void);
    }
    (*fr).buffer.rdata = 0 as *mut byte;
    frame_free_buffers(fr);
    frame_free_toc(fr);
    fi_exit(&mut (*fr).index);
    // clean up possible mess from LFS wrapper.
    if (*fr).wrapperclean.is_some() {
        (*fr).wrapperclean.expect("non-null function pointer")((*fr).wrapperdata);
        (*fr).wrapperdata = 0 as *mut libc::c_void
    }
    bc_cleanup(&mut (*fr).rdat.buffer);
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_framedata(mut mh: *mut mpg123_handle_t,
                                          mut header: *mut ulong,
                                          mut bodydata: *mut *mut byte,
                                          mut bodybytes: *mut size_t)
 -> libc::c_int {
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    if (*mh).to_decode == 0 { return MPG123_ERR as libc::c_int }
    if !header.is_null() { *header = (*mh).oldhead }
    if !bodydata.is_null() { *bodydata = (*mh).bsbuf }
    if !bodybytes.is_null() { *bodybytes = (*mh).framesize as size_t }
    return MPG123_OK as libc::c_int;
}
// Fuzzy frame offset searching (guessing).
// When we don't have an accurate position, we may use an inaccurate one.
// Possibilities:
//	- use approximate positions from Xing TOC (not yet parsed)
//	- guess wildly from mean framesize and offset of first frame / beginning of file.
unsafe extern "C" fn frame_fuzzy_find(mut fr: *mut mpg123_handle_t,
                                      mut want_frame: mpg_off_t,
                                      mut get_frame: *mut mpg_off_t)
 -> mpg_off_t {
    let mut ret: mpg_off_t =
        (*fr).audio_start; // default is to go to the beginning.
    *get_frame = 0 as libc::c_int as mpg_off_t;
    // but we try to find something better.
	// Xing VBR TOC works with relative positions, both in terms of audio frames and stream bytes.
	// thus, it only works when whe know the length of things.
	// oh... I assume the offsets are relative to the _total_ file length.
    if !(*fr).xing_toc.is_null() &&
           (*fr).track_frames > 0 as libc::c_int as libc::c_long &&
           (*fr).rdat.filelen > 0 as libc::c_int as libc::c_long {
        // one could round...
        let mut toc_entry: libc::c_int =
            (want_frame as libc::c_double * 100.0f64 /
                 (*fr).track_frames as libc::c_double) as libc::c_int;
        // it is an index in the 100-entry table.
        if toc_entry < 0 as libc::c_int { toc_entry = 0 as libc::c_int }
        if toc_entry > 99 as libc::c_int { toc_entry = 99 as libc::c_int }
        // now estimate back what frame we get.
        *get_frame =
            (toc_entry as libc::c_double / 100.0f64 *
                 (*fr).track_frames as libc::c_double) as mpg_off_t;
        (*fr).state_flags &= !(FRAME_ACCURATE as libc::c_int);
        (*fr).silent_resync = 1 as libc::c_int as libc::c_char;
        // question: Is the TOC for whole file size (with/without ID3) or the "real" audio data only?
		// ID3v1 info could also matter.
        ret =
            (*(*fr).xing_toc.offset(toc_entry as isize) as libc::c_double /
                 256.0f64 * (*fr).rdat.filelen as libc::c_double) as mpg_off_t
    } else if (*fr).mean_framesize > 0 as libc::c_int as libc::c_double {
        // just guess with mean framesize (may be exact with CBR files).
		// query filelen here or not?
        (*fr).state_flags &= !(FRAME_ACCURATE as libc::c_int); // fuzzy!
        (*fr).silent_resync = 1 as libc::c_int as libc::c_char;
        *get_frame = want_frame;
        ret =
            ((*fr).audio_start as libc::c_double +
                 (*fr).mean_framesize * want_frame as libc::c_double) as
                mpg_off_t
    }
    return ret;
}
// find the best frame in index just before the wanted one, seek to there
// then step to just before wanted one with read_frame
// do not care tabout the stuff that was in buffer but not played back
// everything that left the decoder is counted as played
// decide if you want low latency reaction and accurate timing info or stable long-time playback with buffer!
#[no_mangle]
pub unsafe extern "C" fn frame_index_find(mut fr: *mut mpg123_handle_t,
                                          mut want_frame: mpg_off_t,
                                          mut get_frame: *mut mpg_off_t)
 -> mpg_off_t {
    let mut gopos: mpg_off_t =
        0 as libc::c_int as
            mpg_off_t; // default is file start if no index position
    *get_frame = 0 as libc::c_int as mpg_off_t;
    // possibly use VBRI index, too? I'd need an example for this...
    if (*fr).index.fill != 0 {
        let mut fi: size_t = 0; // find in index
        // at index fi there is frame step*fi...
        fi = (want_frame / (*fr).index.step) as size_t;
        if fi >= (*fr).index.fill {
            // if we are beyond the end of frame index...
			// when fuzzy seek is allowed, we have some limited tolerance for the frames we want to read rather then jump over.
            if (*fr).p.flags & MPG123_FUZZY as libc::c_int as libc::c_long !=
                   0 &&
                   (want_frame as
                        libc::c_ulong).wrapping_sub((*fr).index.fill.wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong).wrapping_mul((*fr).index.step
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                       > 10 as libc::c_int as libc::c_ulong {
                gopos = frame_fuzzy_find(fr, want_frame, get_frame);
                if gopos > (*fr).audio_start { return gopos }
                // only in that case, we have a useful guess.
                // else... just continue, fuzzyness didn't help.
            }
            // use the last available position, slowly advancing from that one.
            fi =
                (*fr).index.fill.wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong)
        }
        // we have index position, that yields frame and byte offsets.
        *get_frame =
            fi.wrapping_mul((*fr).index.step as libc::c_ulong) as mpg_off_t;
        gopos = *(*fr).index.data.offset(fi as isize);
        (*fr).state_flags |= FRAME_ACCURATE as libc::c_int
    } else {
        if (*fr).p.flags & MPG123_FUZZY as libc::c_int as libc::c_long != 0 {
            return frame_fuzzy_find(fr, want_frame, get_frame)
        }
        // a bit hackish here... but we need to be fresh when looking for the first header again.
        (*fr).firsthead = 0 as libc::c_int as ulong;
        (*fr).oldhead = 0 as libc::c_int as ulong
    }
    return gopos;
}
#[no_mangle]
pub unsafe extern "C" fn frame_ins2outs(mut fr: *mut mpg123_handle_t,
                                        mut ins: mpg_off_t) -> mpg_off_t {
    let mut outs: mpg_off_t = 0 as libc::c_int as mpg_off_t;
    match (*fr).down_sample {
        0 => { outs = ins >> (*fr).down_sample }
        _ => { }
    }
    return outs;
}
#[no_mangle]
pub unsafe extern "C" fn frame_outs(mut fr: *mut mpg123_handle_t,
                                    mut num: mpg_off_t) -> mpg_off_t {
    let mut outs: mpg_off_t = 0 as libc::c_int as mpg_off_t;
    match (*fr).down_sample {
        0 => { outs = ((*fr).spf >> (*fr).down_sample) * num }
        _ => { }
    }
    return outs;
}
// compute the number of output samples we expect from this frame.
// this is either simple spf() or a tad more elaborate for ntom.
#[no_mangle]
pub unsafe extern "C" fn frame_expect_outsamples(mut fr: *mut mpg123_handle_t)
 -> mpg_off_t {
    let mut outs: mpg_off_t = 0 as libc::c_int as mpg_off_t;
    match (*fr).down_sample {
        0 => { outs = (*fr).spf >> (*fr).down_sample }
        _ => { }
    }
    return outs;
}
#[no_mangle]
pub unsafe extern "C" fn frame_offset(mut fr: *mut mpg123_handle_t,
                                      mut outs: mpg_off_t) -> mpg_off_t {
    let mut num: mpg_off_t = 0 as libc::c_int as mpg_off_t;
    match (*fr).down_sample {
        0 => { num = outs / ((*fr).spf >> (*fr).down_sample) }
        _ => { }
    }
    return num;
}
// input in _input_ samples
#[no_mangle]
pub unsafe extern "C" fn frame_gapless_init(mut fr: *mut mpg123_handle_t,
                                            mut framecount: mpg_off_t,
                                            mut bskip: mpg_off_t,
                                            mut eskip: mpg_off_t) {
    (*fr).gapless_frames = framecount;
    if (*fr).gapless_frames > 0 as libc::c_int as libc::c_long &&
           bskip >= 0 as libc::c_int as libc::c_long &&
           eskip >= 0 as libc::c_int as libc::c_long {
        (*fr).begin_s = bskip + 529 as libc::c_int as libc::c_long;
        (*fr).end_s =
            framecount * (*fr).spf - eskip +
                529 as libc::c_int as libc::c_long
    } else {
        (*fr).end_s = 0 as libc::c_int as mpg_off_t;
        (*fr).begin_s = (*fr).end_s
    }
    // these will get proper values later, from above plus resampling info.
    (*fr).begin_os = 0 as libc::c_int as mpg_off_t;
    (*fr).end_os = 0 as libc::c_int as mpg_off_t;
    (*fr).fullend_os = 0 as libc::c_int as mpg_off_t;
}
#[no_mangle]
pub unsafe extern "C" fn frame_gapless_realinit(mut fr:
                                                    *mut mpg123_handle_t) {
    (*fr).begin_os = frame_ins2outs(fr, (*fr).begin_s);
    (*fr).end_os = frame_ins2outs(fr, (*fr).end_s);
    if (*fr).gapless_frames > 0 as libc::c_int as libc::c_long {
        (*fr).fullend_os =
            frame_ins2outs(fr, (*fr).gapless_frames * (*fr).spf)
    } else { (*fr).fullend_os = 0 as libc::c_int as mpg_off_t };
}
// at least note when there is trouble...
#[no_mangle]
pub unsafe extern "C" fn frame_gapless_update(mut fr: *mut mpg123_handle_t,
                                              mut total_samples: mpg_off_t) {
    let mut gapless_samples: mpg_off_t = (*fr).gapless_frames * (*fr).spf;
    if (*fr).gapless_frames < 1 as libc::c_int as libc::c_long { return }
    if gapless_samples > total_samples {
        // This invalidates the current position... but what should I do?
        frame_gapless_init(fr, -(1 as libc::c_int) as mpg_off_t,
                           0 as libc::c_int as mpg_off_t,
                           0 as libc::c_int as mpg_off_t);
        frame_gapless_realinit(fr);
        (*fr).lastframe = -(1 as libc::c_int) as mpg_off_t;
        (*fr).lastoff = 0 as libc::c_int as mpg_off_t
    };
}
// compute the needed frame to ignore from, for getting accurate/consistent output for intended firstframe.
unsafe extern "C" fn ignoreframe(mut fr: *mut mpg123_handle_t) -> mpg_off_t {
    let mut preshift: mpg_off_t = (*fr).p.preframes;
    // layer 3 _really_ needs at least one frame before.
    if (*fr).lay == 3 as libc::c_int &&
           preshift < 1 as libc::c_int as libc::c_long {
        preshift = 1 as libc::c_int as mpg_off_t
    }
    // layer 1 & 2 really do not need more than 2.
    if (*fr).lay != 3 as libc::c_int &&
           preshift > 2 as libc::c_int as libc::c_long {
        preshift = 2 as libc::c_int as mpg_off_t
    }
    return (*fr).firstframe - preshift;
}
// the frame seek... this is not simply the seek to fe * fr->spf samples in output because we think of _input_ frames here.
// seek to frame offset 1 may be just seek to 200 samples offset in output since the beginning of first frame is delay/padding.
// hm, is that right? OK for the padding stuff, but actually, should the decoder delay be better totally hidden or not?
// with gapless, even the whole frame position could be advanced further than requested (since Homey don't play dat).
#[no_mangle]
pub unsafe extern "C" fn frame_set_frameseek(mut fr: *mut mpg123_handle_t,
                                             mut fe: mpg_off_t) {
    (*fr).firstframe = fe;
    if (*fr).p.flags & MPG123_GAPLESS as libc::c_int as libc::c_long != 0 &&
           (*fr).gapless_frames > 0 as libc::c_int as libc::c_long {
        // take care of the beginning...
        let mut beg_f: mpg_off_t = frame_offset(fr, (*fr).begin_os);
        if fe <= beg_f {
            (*fr).firstframe = beg_f;
            (*fr).firstoff = (*fr).begin_os - frame_outs(fr, beg_f)
        } else { (*fr).firstoff = 0 as libc::c_int as mpg_off_t }
        // the end is set once for a track at least, on the frame_set_frameseek called in get_next_frame()
        if (*fr).end_os > 0 as libc::c_int as libc::c_long {
            (*fr).lastframe = frame_offset(fr, (*fr).end_os);
            (*fr).lastoff = (*fr).end_os - frame_outs(fr, (*fr).lastframe)
        } else {
            (*fr).lastframe = -(1 as libc::c_int) as mpg_off_t;
            (*fr).lastoff = 0 as libc::c_int as mpg_off_t
        }
    } else {
        (*fr).lastoff = 0 as libc::c_int as mpg_off_t;
        (*fr).firstoff = (*fr).lastoff;
        (*fr).lastframe = -(1 as libc::c_int) as mpg_off_t
    }
    (*fr).ignoreframe = ignoreframe(fr);
}
#[no_mangle]
pub unsafe extern "C" fn frame_skip(mut fr: *mut mpg123_handle_t) {
    if (*fr).lay == 3 as libc::c_int {
        set_pointer(fr, 512 as libc::c_int as libc::c_long);
    };
}
// sample accurate seek prepare for decoder.
// this gets unadjusted output samples and takes resampling into account
#[no_mangle]
pub unsafe extern "C" fn frame_set_seek(mut fr: *mut mpg123_handle_t,
                                        mut sp: mpg_off_t) {
    (*fr).firstframe = frame_offset(fr, sp);
    (*fr).ignoreframe = ignoreframe(fr);
    (*fr).firstoff = sp - frame_outs(fr, (*fr).firstframe);
}
unsafe extern "C" fn get_rva(mut fr: *mut mpg123_handle_t,
                             mut peak: *mut libc::c_double,
                             mut gain: *mut libc::c_double) -> libc::c_int {
    let mut p: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    let mut g: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*fr).p.rva != 0 {
        let mut rt: libc::c_int = 0 as libc::c_int;
        // should one assume a zero RVA as no RVA?
        if (*fr).p.rva == 2 as libc::c_int &&
               (*fr).rva.level[1 as libc::c_int as usize] !=
                   -(1 as libc::c_int) {
            rt = 1 as libc::c_int
        }
        if (*fr).rva.level[rt as usize] != -(1 as libc::c_int) {
            p = (*fr).rva.peak[rt as usize] as libc::c_double;
            g = (*fr).rva.gain[rt as usize] as libc::c_double;
            ret = 1 as libc::c_int
            // success.
        }
    }
    if !peak.is_null() { *peak = p }
    if !gain.is_null() { *gain = g }
    return ret;
}
// adjust the volume, taking both fr->outscale and rva values into account
#[no_mangle]
pub unsafe extern "C" fn do_rva(mut fr: *mut mpg123_handle_t) {
    let mut peak: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut gain: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut newscale: libc::c_double = 0.;
    let mut rvafact: libc::c_double = 1 as libc::c_int as libc::c_double;
    if get_rva(fr, &mut peak, &mut gain) != 0 {
        rvafact =
            pow(10 as libc::c_int as libc::c_double,
                gain / 20 as libc::c_int as libc::c_double)
    }
    newscale = (*fr).p.outscale * rvafact;
    // if peak is unknown (== 0) this check won't hurt
    if peak * newscale > 1.0f64 { newscale = 1.0f64 / peak }
    // first rva setting is forced with fr->lastscale < 0
    if newscale != (*fr).lastscale || (*fr).decoder_change != 0 {
        (*fr).lastscale = newscale;
        // the actual work
        if (*fr).make_decode_tables.is_some() {
            (*fr).make_decode_tables.expect("non-null function pointer")(fr);
        }
    };
}
// it may be too early, actually.
