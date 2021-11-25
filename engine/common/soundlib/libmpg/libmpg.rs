#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn mpg123_exit();
    #[no_mangle]
    fn mpg123_init() -> libc::c_int;
    #[no_mangle]
    fn mpg123_tell(mh: *mut mpg123_handle_t) -> mpg_off_t;
    #[no_mangle]
    fn mpg123_delete(mh: *mut mpg123_handle_t);
    #[no_mangle]
    fn mpg123_open_feed(mh: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn get_songlen(fr: *mut mpg123_handle_t, no: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mpg123_format_none(mh: *mut mpg123_handle_t) -> libc::c_int;
    #[no_mangle]
    fn mpg123_format(mh: *mut mpg123_handle_t, rate: libc::c_long,
                     channels: libc::c_int, encodings: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn mpg123_new(error: *mut libc::c_int) -> *mut mpg123_handle_t;
    #[no_mangle]
    fn mpg123_param(mh: *mut mpg123_handle_t, key: mpg123_parms,
                    val: libc::c_long) -> libc::c_int;
    #[no_mangle]
    fn mpg123_open_handle(mh: *mut mpg123_handle_t,
                          iohandle: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn mpg123_replace_reader_handle(mh: *mut mpg123_handle_t,
                                    fread:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_void,
                                                                    _:
                                                                        *mut libc::c_void,
                                                                    _: size_t)
                                                   -> mpg_ssize_t>,
                                    lseek:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_void,
                                                                    _:
                                                                        mpg_off_t,
                                                                    _:
                                                                        libc::c_int)
                                                   -> mpg_off_t>,
                                    fclose:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_void)
                                                   -> ()>) -> libc::c_int;
    #[no_mangle]
    fn mpg123_decode(mh: *mut mpg123_handle_t, inmemory: *const byte,
                     inmemsize: size_t, outmemory: *mut byte,
                     outmemsize: size_t, done: *mut size_t) -> libc::c_int;
    #[no_mangle]
    fn mpg123_getformat(mh: *mut mpg123_handle_t, rate: *mut libc::c_int,
                        channels: *mut libc::c_int,
                        encoding: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mpg123_read(mh: *mut mpg123_handle_t, out: *mut byte, size: size_t,
                   done: *mut size_t) -> libc::c_int;
    #[no_mangle]
    fn mpg123_seek(mh: *mut mpg123_handle_t, sampleoff: mpg_off_t,
                   whence: libc::c_int) -> mpg_off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavinfo_t {
    pub rate: libc::c_int,
    pub channels: libc::c_int,
    pub playtime: libc::c_int,
}
pub type pfread
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: size_t) -> libc::c_long>;
pub type pfseek
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_long,
                                _: libc::c_int) -> libc::c_long>;
/*
libmpg.c - compact version of famous library mpg123
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
#[no_mangle]
pub unsafe extern "C" fn create_decoder(mut error: *mut libc::c_int)
 -> *mut libc::c_void {
    let mut mpg: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: libc::c_int = 0;
    if !error.is_null() { *error = 0 as libc::c_int }
    mpg123_init();
    mpg = mpg123_new(&mut ret) as *mut libc::c_void;
    if mpg.is_null() { return 0 as *mut libc::c_void }
    ret =
        mpg123_param(mpg as *mut mpg123_handle_t, MPG123_FLAGS,
                     (MPG123_FUZZY as libc::c_int |
                          MPG123_SEEKBUFFER as libc::c_int |
                          MPG123_GAPLESS as libc::c_int) as libc::c_long);
    if ret != MPG123_OK as libc::c_int && !error.is_null() {
        *error = 1 as libc::c_int
    }
    // let the seek index auto-grow and contain an entry for every frame
    ret =
        mpg123_param(mpg as *mut mpg123_handle_t, MPG123_INDEX_SIZE,
                     -(1 as libc::c_int) as libc::c_long);
    if ret != MPG123_OK as libc::c_int && !error.is_null() {
        *error = 1 as libc::c_int
    }
    return mpg;
}
#[no_mangle]
pub unsafe extern "C" fn feed_mpeg_header(mut mpg: *mut libc::c_void,
                                          mut data: *const byte,
                                          mut bufsize: libc::c_long,
                                          mut streamsize: libc::c_long,
                                          mut sc: *mut wavinfo_t)
 -> libc::c_int {
    let mut mh: *mut mpg123_handle_t = mpg as *mut mpg123_handle_t;
    let mut ret: libc::c_int = 0;
    let mut no: libc::c_int = 0;
    if mh.is_null() || sc.is_null() { return 0 as libc::c_int }
    ret = mpg123_open_feed(mh);
    if ret != MPG123_OK as libc::c_int { return 0 as libc::c_int }
    // feed input chunk and get first chunk of decoded audio.
    ret =
        mpg123_decode(mh, data, bufsize as size_t, 0 as *mut byte,
                      0 as libc::c_int as size_t,
                      0 as *mut size_t); // there were errors
    if ret != MPG123_NEW_FORMAT as libc::c_int { return 0 as libc::c_int }
    mpg123_getformat(mh, &mut (*sc).rate, &mut (*sc).channels, &mut no);
    mpg123_format_none(mh);
    mpg123_format(mh, (*sc).rate as libc::c_long, (*sc).channels,
                  MPG123_ENC_SIGNED_16 as libc::c_int);
    // some hacking to get function get_songlen to working properly
    (*mh).rdat.filelen = streamsize;
    (*sc).playtime =
        get_songlen(mh, -(1 as libc::c_int)) * 1000 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn feed_mpeg_stream(mut mpg: *mut libc::c_void,
                                          mut data: *const byte,
                                          mut bufsize: libc::c_long,
                                          mut outbuf: *mut byte,
                                          mut outsize: *mut size_t)
 -> libc::c_int {
    match mpg123_decode(mpg as *mut mpg123_handle_t, data, bufsize as size_t,
                        outbuf, 8192 as libc::c_int as size_t, outsize) {
        -10 => { return 1 as libc::c_int }
        0 => { return 0 as libc::c_int }
        _ => { return -(1 as libc::c_int) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn open_mpeg_stream(mut mpg: *mut libc::c_void,
                                          mut file: *mut libc::c_void,
                                          mut f_read: pfread,
                                          mut f_seek: pfseek,
                                          mut sc: *mut wavinfo_t)
 -> libc::c_int {
    let mut mh: *mut mpg123_handle_t = mpg as *mut mpg123_handle_t;
    let mut ret: libc::c_int = 0;
    let mut no: libc::c_int = 0;
    if mh.is_null() || sc.is_null() { return 0 as libc::c_int }
    ret =
        mpg123_replace_reader_handle(mh,
                                     ::std::mem::transmute::<*mut libc::c_void,
                                                             Option<unsafe extern "C" fn(_:
                                                                                             *mut libc::c_void,
                                                                                         _:
                                                                                             *mut libc::c_void,
                                                                                         _:
                                                                                             size_t)
                                                                        ->
                                                                            mpg_ssize_t>>(::std::mem::transmute::<pfread,
                                                                                                                  *mut libc::c_void>(f_read)),
                                     ::std::mem::transmute::<*mut libc::c_void,
                                                             Option<unsafe extern "C" fn(_:
                                                                                             *mut libc::c_void,
                                                                                         _:
                                                                                             mpg_off_t,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            mpg_off_t>>(::std::mem::transmute::<pfseek,
                                                                                                                *mut libc::c_void>(f_seek)),
                                     None);
    if ret != MPG123_OK as libc::c_int { return 0 as libc::c_int }
    ret = mpg123_open_handle(mh, file);
    if ret != MPG123_OK as libc::c_int { return 0 as libc::c_int }
    ret = mpg123_getformat(mh, &mut (*sc).rate, &mut (*sc).channels, &mut no);
    if ret != MPG123_OK as libc::c_int { return 0 as libc::c_int }
    mpg123_format_none(mh);
    mpg123_format(mh, (*sc).rate as libc::c_long, (*sc).channels,
                  MPG123_ENC_SIGNED_16 as libc::c_int);
    (*sc).playtime =
        get_songlen(mh, -(1 as libc::c_int)) * 1000 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn read_mpeg_stream(mut mpg: *mut libc::c_void,
                                          mut outbuf: *mut byte,
                                          mut outsize: *mut size_t)
 -> libc::c_int {
    match mpg123_read(mpg as *mut mpg123_handle_t, outbuf,
                      8192 as libc::c_int as size_t, outsize) {
        0 => { return 0 as libc::c_int }
        _ => { return -(1 as libc::c_int) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_stream_pos(mut mpg: *mut libc::c_void)
 -> libc::c_int {
    return mpg123_tell(mpg as *mut mpg123_handle_t) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_stream_pos(mut mpg: *mut libc::c_void,
                                        mut curpos: libc::c_int)
 -> libc::c_int {
    return mpg123_seek(mpg as *mut mpg123_handle_t, curpos as mpg_off_t,
                       0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn close_decoder(mut mpg: *mut libc::c_void) {
    mpg123_delete(mpg as *mut mpg123_handle_t);
    mpg123_exit();
}
