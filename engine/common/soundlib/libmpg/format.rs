#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn frame_freq(fr: *mut mpg123_handle_t) -> libc::c_long;
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
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type int16_t = __int16_t;
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
pub type uint16_t = __uint16_t;
pub const MPG123_BAD_OUTFORMAT: mpg123_errors = 1;
pub const MPG123_AUTO_RESAMPLE: mpg123_param_flags = 32768;
pub const MPG123_FORCE_MONO: mpg123_param_flags = 7;
pub const MPG123_FORCE_STEREO: mpg123_param_flags = 8;
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
pub const MPG123_OK: mpg123_errors = 0;
pub const MPG123_ERR: mpg123_errors = -1;
pub const MPG123_NEED_MORE: mpg123_errors = -10;
pub const MPG123_NEW_FORMAT: mpg123_errors = -11;
pub const MPG123_DONE: mpg123_errors = -12;
pub type mpg123_param_flags = libc::c_uint;
pub const MPG123_IGNORE_INFOFRAME: mpg123_param_flags = 16384;
pub const MPG123_IGNORE_STREAMLENGTH: mpg123_param_flags = 4096;
pub const MPG123_FUZZY: mpg123_param_flags = 512;
pub const MPG123_SEEKBUFFER: mpg123_param_flags = 256;
pub const MPG123_NO_RESYNC: mpg123_param_flags = 128;
pub const MPG123_GAPLESS: mpg123_param_flags = 64;
pub const MPG123_QUIET: mpg123_param_flags = 32;
pub const MPG123_MONO_MIX: mpg123_param_flags = 4;
pub const MPG123_MONO_RIGHT: mpg123_param_flags = 2;
pub const MPG123_MONO_LEFT: mpg123_param_flags = 1;
pub const MPG123_MONO: mpg123_channelcount = 1;
pub const MPG123_STEREO: mpg123_channelcount = 2;
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
pub type mpg123_channelcount = libc::c_uint;
// only the standard rates
static mut my_rates: [libc::c_long; 9] =
    [8000 as libc::c_int as libc::c_long,
     11025 as libc::c_int as libc::c_long,
     12000 as libc::c_int as libc::c_long,
     16000 as libc::c_int as libc::c_long,
     22050 as libc::c_int as libc::c_long,
     24000 as libc::c_int as libc::c_long,
     32000 as libc::c_int as libc::c_long,
     44100 as libc::c_int as libc::c_long,
     48000 as libc::c_int as libc::c_long];
static mut my_encodings: [libc::c_int; 2] =
    [MPG123_ENC_SIGNED_16 as libc::c_int,
     MPG123_ENC_UNSIGNED_16 as libc::c_int];
// the list of actually possible encodings.
static mut good_encodings: [libc::c_int; 2] =
    [MPG123_ENC_SIGNED_16 as libc::c_int,
     MPG123_ENC_UNSIGNED_16 as libc::c_int];
// check if encoding is a valid one in this build.
unsafe extern "C" fn good_enc(enc: libc::c_int) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[libc::c_int; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                                   as libc::c_ulong) {
        if enc == good_encodings[i as usize] {
            return (0 as libc::c_int == 0) as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_rates(mut list: *mut *const libc::c_long,
                                      mut number: *mut size_t) {
    if !number.is_null() {
        *number =
            (::std::mem::size_of::<[libc::c_long; 9]>() as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_long>()
                                                 as libc::c_ulong)
    }
    if !list.is_null() { *list = my_rates.as_ptr() };
}
// now that's a bit tricky... One build of the library knows only a subset of the encodings.
#[no_mangle]
pub unsafe extern "C" fn mpg123_encodings(mut list: *mut *const libc::c_int,
                                          mut number: *mut size_t) {
    if !number.is_null() {
        *number =
            (::std::mem::size_of::<[libc::c_int; 2]>() as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                                 as libc::c_ulong)
    }
    if !list.is_null() { *list = good_encodings.as_ptr() };
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_encsize(mut encoding: libc::c_int)
 -> libc::c_int {
    return ::std::mem::size_of::<libc::c_short>() as libc::c_ulong as
               libc::c_int;
}
unsafe extern "C" fn rate2num(mut r: libc::c_long) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if my_rates[i as usize] == r { return i }
        i += 1
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn enc2num(mut encoding: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if my_encodings[i as usize] == encoding { return i }
        i += 1
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cap_fit(mut fr: *mut mpg123_handle_t,
                             mut nf: *mut audioformat_t, mut f0: libc::c_int,
                             mut f2: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = (*nf).channels - 1 as libc::c_int;
    let mut rn: libc::c_int = rate2num((*nf).rate as libc::c_long);
    if rn >= 0 as libc::c_int {
        i = f0;
        while i < f2 {
            if (*fr).p.audio_caps[c as usize][rn as usize][i as usize] != 0 {
                (*nf).encoding = my_encodings[i as usize];
                return 1 as libc::c_int
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn freq_fit(mut fr: *mut mpg123_handle_t,
                              mut nf: *mut audioformat_t, mut f0: libc::c_int,
                              mut f2: libc::c_int) -> libc::c_int {
    (*nf).rate = (frame_freq(fr) >> (*fr).p.down_sample) as libc::c_int;
    if cap_fit(fr, nf, f0, f2) != 0 { return 1 as libc::c_int }
    if (*fr).p.flags & MPG123_AUTO_RESAMPLE as libc::c_int as libc::c_long !=
           0 {
        (*nf).rate >>= 1 as libc::c_int;
        if cap_fit(fr, nf, f0, f2) != 0 { return 1 as libc::c_int }
        (*nf).rate >>= 1 as libc::c_int;
        if cap_fit(fr, nf, f0, f2) != 0 { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
// match constraints against supported audio formats, store possible setup in frame
// return: -1: error; 0: no format change; 1: format change
#[no_mangle]
pub unsafe extern "C" fn frame_output_format(mut fr: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut f0: libc::c_int = 0 as libc::c_int;
    let mut f2: libc::c_int = 2 as libc::c_int;
    let mut p: *mut mpg123_parm_t = &mut (*fr).p;
    let mut nf: audioformat_t =
        audioformat_t{encoding: 0,
                      encsize: 0,
                      dec_enc: 0,
                      dec_encsize: 0,
                      channels: 0,
                      rate: 0,};
    // initialize new format, encoding comes later
    nf.channels = (*fr).stereo;
    // force stereo is stronger
    if (*p).flags & MPG123_FORCE_MONO as libc::c_int as libc::c_long != 0 {
        nf.channels = 1 as libc::c_int
    } // try rates with 16bit
    if (*p).flags & MPG123_FORCE_STEREO as libc::c_int as libc::c_long != 0 {
        nf.channels = 2 as libc::c_int
    } // ... 8bit
    if !(freq_fit(fr, &mut nf, f0, 2 as libc::c_int) != 0) {
        if !(freq_fit(fr, &mut nf,
                      if f0 <= 2 as libc::c_int {
                          2 as libc::c_int
                      } else { f0 }, f2) != 0) {
            // try again with different stereoness
            if nf.channels == 2 as libc::c_int &&
                   (*p).flags &
                       MPG123_FORCE_STEREO as libc::c_int as libc::c_long == 0
               {
                nf.channels = 1 as libc::c_int
            } else if nf.channels == 1 as libc::c_int &&
                          (*p).flags &
                              MPG123_FORCE_MONO as libc::c_int as libc::c_long
                              == 0 {
                nf.channels = 2 as libc::c_int
            } // try rates with 16bit
            if !(freq_fit(fr, &mut nf, f0, 2 as libc::c_int) != 0) {
                if !(freq_fit(fr, &mut nf,
                              if f0 <= 2 as libc::c_int {
                                  2 as libc::c_int
                              } else { f0 }, f2) != 0) {
                    (*fr).err =
                        MPG123_BAD_OUTFORMAT as libc::c_int; // ... 8bit
                    return -(1 as libc::c_int)
                }
            }
        }
    }
    // here is the _good_ end.
	// we had a successful match, now see if there's a change
    if nf.rate == (*fr).af.rate && nf.channels == (*fr).af.channels &&
           nf.encoding == (*fr).af.encoding {
        return 0 as libc::c_int
        // the same format as before
    } else {
        // a new format
        (*fr).af.rate = nf.rate;
        (*fr).af.channels = nf.channels;
        (*fr).af.encoding = nf.encoding;
        // cache the size of one sample in bytes, for ease of use.
        (*fr).af.encsize = mpg123_encsize((*fr).af.encoding);
        if (*fr).af.encsize < 1 as libc::c_int {
            (*fr).err = MPG123_BAD_OUTFORMAT as libc::c_int;
            return -(1 as libc::c_int)
        }
        // set up the decoder synth format. Might differ.
		// without high-precision synths, 16 bit signed is the basis for
		// everything higher than 8 bit.
        if (*fr).af.encsize > 2 as libc::c_int {
            (*fr).af.dec_enc = MPG123_ENC_SIGNED_16 as libc::c_int
        } else {
            match (*fr).af.encoding {
                96 => {
                    (*fr).af.dec_enc = MPG123_ENC_SIGNED_16 as libc::c_int
                }
                _ => { (*fr).af.dec_enc = (*fr).af.encoding }
            }
        }
        (*fr).af.dec_encsize = mpg123_encsize((*fr).af.dec_enc);
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn mpg123_fmt_none(mut mp: *mut mpg123_parm_t)
 -> libc::c_int {
    if mp.is_null() { return MPG123_BAD_PARS as libc::c_int }
    memset((*mp).audio_caps.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[[[libc::c_char; 2]; 10]; 2]>() as
               libc::c_ulong);
    return MPG123_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_fmt_all(mut mp: *mut mpg123_parm_t)
 -> libc::c_int {
    let mut rate: size_t = 0;
    let mut ch: size_t = 0;
    let mut enc: size_t = 0;
    if mp.is_null() { return MPG123_BAD_PARS as libc::c_int }
    ch = 0 as libc::c_int as size_t;
    while ch < 2 as libc::c_int as libc::c_ulong {
        rate = 0 as libc::c_int as size_t;
        while rate < (9 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
            enc = 0 as libc::c_int as size_t;
            while enc < 2 as libc::c_int as libc::c_ulong {
                (*mp).audio_caps[ch as usize][rate as usize][enc as usize] =
                    good_enc(my_encodings[enc as usize]) as libc::c_char;
                enc = enc.wrapping_add(1)
            }
            rate = rate.wrapping_add(1)
        }
        ch = ch.wrapping_add(1)
    }
    return MPG123_OK as libc::c_int;
}
unsafe extern "C" fn mpg123_fmt(mut mp: *mut mpg123_parm_t,
                                mut rate: libc::c_long,
                                mut channels: libc::c_int,
                                mut encodings: libc::c_int) -> libc::c_int {
    let mut ie: libc::c_int = 0;
    let mut ic: libc::c_int = 0;
    let mut ratei: libc::c_int = 0;
    let mut ch: [libc::c_int; 2] = [0 as libc::c_int, 1 as libc::c_int];
    if mp.is_null() { return MPG123_BAD_PARS as libc::c_int }
    if channels & (MPG123_MONO as libc::c_int | MPG123_STEREO as libc::c_int)
           == 0 {
        return MPG123_BAD_CHANNEL as libc::c_int
    }
    if channels & MPG123_STEREO as libc::c_int == 0 {
        ch[1 as libc::c_int as usize] = 0 as libc::c_int
    } else if channels & MPG123_MONO as libc::c_int == 0 {
        ch[0 as libc::c_int as usize] = 1 as libc::c_int
    }
    ratei = rate2num(rate);
    if ratei < 0 as libc::c_int { return MPG123_BAD_RATE as libc::c_int }
    // now match the encodings
    ic = 0 as libc::c_int;
    while ic < 2 as libc::c_int {
        ie = 0 as libc::c_int;
        while ie < 2 as libc::c_int {
            if good_enc(my_encodings[ie as usize]) != 0 &&
                   my_encodings[ie as usize] & encodings ==
                       my_encodings[ie as usize] {
                (*mp).audio_caps[ch[ic as usize] as
                                     usize][ratei as usize][ie as usize] =
                    1 as libc::c_int as libc::c_char
            }
            ie += 1
        }
        if ch[0 as libc::c_int as usize] == ch[1 as libc::c_int as usize] {
            break ;
        }
        ic += 1
        // no need to do it again
    }
    return MPG123_OK as libc::c_int;
}
unsafe extern "C" fn mpg123_fmt_support(mut mp: *mut mpg123_parm_t,
                                        mut rate: libc::c_long,
                                        mut encoding: libc::c_int)
 -> libc::c_int {
    let mut ratei: libc::c_int = 0;
    let mut enci: libc::c_int = 0;
    let mut ch: libc::c_int = 0 as libc::c_int;
    ratei = rate2num(rate);
    enci = enc2num(encoding);
    if mp.is_null() || ratei < 0 as libc::c_int || enci < 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if (*mp).audio_caps[0 as libc::c_int as
                            usize][ratei as usize][enci as usize] != 0 {
        ch |= MPG123_MONO as libc::c_int
    }
    if (*mp).audio_caps[1 as libc::c_int as
                            usize][ratei as usize][enci as usize] != 0 {
        ch |= MPG123_STEREO as libc::c_int
    }
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_format_none(mut mh: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    r = mpg123_fmt_none(&mut (*mh).p);
    if r != MPG123_OK as libc::c_int {
        (*mh).err = r;
        return MPG123_ERR as libc::c_int
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_format_all(mut mh: *mut mpg123_handle_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    r = mpg123_fmt_all(&mut (*mh).p);
    if r != MPG123_OK as libc::c_int {
        (*mh).err = r;
        return MPG123_ERR as libc::c_int
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_format(mut mh: *mut mpg123_handle_t,
                                       mut rate: libc::c_long,
                                       mut channels: libc::c_int,
                                       mut encodings: libc::c_int)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    if mh.is_null() { return MPG123_BAD_HANDLE as libc::c_int }
    r = mpg123_fmt(&mut (*mh).p, rate, channels, encodings);
    if r != MPG123_OK as libc::c_int {
        (*mh).err = r;
        return MPG123_ERR as libc::c_int
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mpg123_format_support(mut mh: *mut mpg123_handle_t,
                                               mut rate: libc::c_long,
                                               mut encoding: libc::c_int)
 -> libc::c_int {
    if mh.is_null() { return 0 as libc::c_int }
    return mpg123_fmt_support(&mut (*mh).p, rate, encoding);
}
// call this one to ensure that any valid format will be something different than this.
#[no_mangle]
pub unsafe extern "C" fn invalidate_format(mut af: *mut audioformat_t) {
    (*af).encoding = 0 as libc::c_int;
    (*af).channels = 0 as libc::c_int;
    (*af).rate = 0 as libc::c_int;
}
// number of bytes the decoder produces.
#[no_mangle]
pub unsafe extern "C" fn decoder_synth_bytes(mut fr: *mut mpg123_handle_t,
                                             mut s: mpg_off_t) -> mpg_off_t {
    return s * (*fr).af.dec_encsize as libc::c_long *
               (*fr).af.channels as libc::c_long;
}
// samples/bytes for output buffer after post-processing.
// take into account: channels, bytes per sample -- NOT resampling!
#[no_mangle]
pub unsafe extern "C" fn samples_to_bytes(mut fr: *mut mpg123_handle_t,
                                          mut s: mpg_off_t) -> mpg_off_t {
    return s * (*fr).af.encsize as libc::c_long *
               (*fr).af.channels as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn bytes_to_samples(mut fr: *mut mpg123_handle_t,
                                          mut b: mpg_off_t) -> mpg_off_t {
    return b / (*fr).af.encsize as libc::c_long /
               (*fr).af.channels as libc::c_long;
}
// number of bytes needed for decoding _and_ post-processing.
#[no_mangle]
pub unsafe extern "C" fn outblock_bytes(mut fr: *mut mpg123_handle_t,
                                        mut s: mpg_off_t) -> mpg_off_t {
    let mut encsize: libc::c_int =
        if (*fr).af.encsize > (*fr).af.dec_encsize {
            (*fr).af.encsize
        } else { (*fr).af.dec_encsize };
    return s * encsize as libc::c_long * (*fr).af.channels as libc::c_long;
}
unsafe extern "C" fn conv_s16_to_u16(mut buf: *mut outbuffer_t) {
    let mut ssamples: *mut int16_t = (*buf).data as *mut int16_t;
    let mut usamples: *mut uint16_t = (*buf).data as *mut uint16_t;
    let mut count: size_t =
        (*buf).fill.wrapping_div(::std::mem::size_of::<int16_t>() as
                                     libc::c_ulong);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < count {
        let mut tmp: libc::c_long =
            *ssamples.offset(i as isize) as libc::c_long +
                32768 as libc::c_int as libc::c_long;
        *usamples.offset(i as isize) = tmp as uint16_t;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn postprocess_buffer(mut fr: *mut mpg123_handle_t) {
    match (*fr).af.dec_enc {
        208 => {
            match (*fr).af.encoding {
                96 => { conv_s16_to_u16(&mut (*fr).buffer); }
                _ => { }
            }
        }
        _ => { }
    };
}
