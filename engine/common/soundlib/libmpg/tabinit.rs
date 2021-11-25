#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
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
/*
tabinit.c - compact version of famous library mpg123
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
static mut cos64: [libc::c_float; 16] = [0.; 16];
static mut cos32: [libc::c_float; 8] = [0.; 8];
static mut cos16: [libc::c_float; 4] = [0.; 4];
static mut cos8: [libc::c_float; 2] = [0.; 2];
static mut cos4: [libc::c_float; 1] = [0.; 1];
static mut intwinbase: [libc::c_long; 257] =
    [0 as libc::c_int as libc::c_long, -(1 as libc::c_int) as libc::c_long,
     -(1 as libc::c_int) as libc::c_long, -(1 as libc::c_int) as libc::c_long,
     -(1 as libc::c_int) as libc::c_long, -(1 as libc::c_int) as libc::c_long,
     -(1 as libc::c_int) as libc::c_long, -(2 as libc::c_int) as libc::c_long,
     -(2 as libc::c_int) as libc::c_long, -(2 as libc::c_int) as libc::c_long,
     -(2 as libc::c_int) as libc::c_long, -(3 as libc::c_int) as libc::c_long,
     -(3 as libc::c_int) as libc::c_long, -(4 as libc::c_int) as libc::c_long,
     -(4 as libc::c_int) as libc::c_long, -(5 as libc::c_int) as libc::c_long,
     -(5 as libc::c_int) as libc::c_long, -(6 as libc::c_int) as libc::c_long,
     -(7 as libc::c_int) as libc::c_long, -(7 as libc::c_int) as libc::c_long,
     -(8 as libc::c_int) as libc::c_long, -(9 as libc::c_int) as libc::c_long,
     -(10 as libc::c_int) as libc::c_long,
     -(11 as libc::c_int) as libc::c_long,
     -(13 as libc::c_int) as libc::c_long,
     -(14 as libc::c_int) as libc::c_long,
     -(16 as libc::c_int) as libc::c_long,
     -(17 as libc::c_int) as libc::c_long,
     -(19 as libc::c_int) as libc::c_long,
     -(21 as libc::c_int) as libc::c_long,
     -(24 as libc::c_int) as libc::c_long,
     -(26 as libc::c_int) as libc::c_long,
     -(29 as libc::c_int) as libc::c_long,
     -(31 as libc::c_int) as libc::c_long,
     -(35 as libc::c_int) as libc::c_long,
     -(38 as libc::c_int) as libc::c_long,
     -(41 as libc::c_int) as libc::c_long,
     -(45 as libc::c_int) as libc::c_long,
     -(49 as libc::c_int) as libc::c_long,
     -(53 as libc::c_int) as libc::c_long,
     -(58 as libc::c_int) as libc::c_long,
     -(63 as libc::c_int) as libc::c_long,
     -(68 as libc::c_int) as libc::c_long,
     -(73 as libc::c_int) as libc::c_long,
     -(79 as libc::c_int) as libc::c_long,
     -(85 as libc::c_int) as libc::c_long,
     -(91 as libc::c_int) as libc::c_long,
     -(97 as libc::c_int) as libc::c_long,
     -(104 as libc::c_int) as libc::c_long,
     -(111 as libc::c_int) as libc::c_long,
     -(117 as libc::c_int) as libc::c_long,
     -(125 as libc::c_int) as libc::c_long,
     -(132 as libc::c_int) as libc::c_long,
     -(139 as libc::c_int) as libc::c_long,
     -(147 as libc::c_int) as libc::c_long,
     -(154 as libc::c_int) as libc::c_long,
     -(161 as libc::c_int) as libc::c_long,
     -(169 as libc::c_int) as libc::c_long,
     -(176 as libc::c_int) as libc::c_long,
     -(183 as libc::c_int) as libc::c_long,
     -(190 as libc::c_int) as libc::c_long,
     -(196 as libc::c_int) as libc::c_long,
     -(202 as libc::c_int) as libc::c_long,
     -(208 as libc::c_int) as libc::c_long,
     -(213 as libc::c_int) as libc::c_long,
     -(218 as libc::c_int) as libc::c_long,
     -(222 as libc::c_int) as libc::c_long,
     -(225 as libc::c_int) as libc::c_long,
     -(227 as libc::c_int) as libc::c_long,
     -(228 as libc::c_int) as libc::c_long,
     -(228 as libc::c_int) as libc::c_long,
     -(227 as libc::c_int) as libc::c_long,
     -(224 as libc::c_int) as libc::c_long,
     -(221 as libc::c_int) as libc::c_long,
     -(215 as libc::c_int) as libc::c_long,
     -(208 as libc::c_int) as libc::c_long,
     -(200 as libc::c_int) as libc::c_long,
     -(189 as libc::c_int) as libc::c_long,
     -(177 as libc::c_int) as libc::c_long,
     -(163 as libc::c_int) as libc::c_long,
     -(146 as libc::c_int) as libc::c_long,
     -(127 as libc::c_int) as libc::c_long,
     -(106 as libc::c_int) as libc::c_long,
     -(83 as libc::c_int) as libc::c_long,
     -(57 as libc::c_int) as libc::c_long,
     -(29 as libc::c_int) as libc::c_long, 2 as libc::c_int as libc::c_long,
     36 as libc::c_int as libc::c_long, 72 as libc::c_int as libc::c_long,
     111 as libc::c_int as libc::c_long, 153 as libc::c_int as libc::c_long,
     197 as libc::c_int as libc::c_long, 244 as libc::c_int as libc::c_long,
     294 as libc::c_int as libc::c_long, 347 as libc::c_int as libc::c_long,
     401 as libc::c_int as libc::c_long, 459 as libc::c_int as libc::c_long,
     519 as libc::c_int as libc::c_long, 581 as libc::c_int as libc::c_long,
     645 as libc::c_int as libc::c_long, 711 as libc::c_int as libc::c_long,
     779 as libc::c_int as libc::c_long, 848 as libc::c_int as libc::c_long,
     919 as libc::c_int as libc::c_long, 991 as libc::c_int as libc::c_long,
     1064 as libc::c_int as libc::c_long, 1137 as libc::c_int as libc::c_long,
     1210 as libc::c_int as libc::c_long, 1283 as libc::c_int as libc::c_long,
     1356 as libc::c_int as libc::c_long, 1428 as libc::c_int as libc::c_long,
     1498 as libc::c_int as libc::c_long, 1567 as libc::c_int as libc::c_long,
     1634 as libc::c_int as libc::c_long, 1698 as libc::c_int as libc::c_long,
     1759 as libc::c_int as libc::c_long, 1817 as libc::c_int as libc::c_long,
     1870 as libc::c_int as libc::c_long, 1919 as libc::c_int as libc::c_long,
     1962 as libc::c_int as libc::c_long, 2001 as libc::c_int as libc::c_long,
     2032 as libc::c_int as libc::c_long, 2057 as libc::c_int as libc::c_long,
     2075 as libc::c_int as libc::c_long, 2085 as libc::c_int as libc::c_long,
     2087 as libc::c_int as libc::c_long, 2080 as libc::c_int as libc::c_long,
     2063 as libc::c_int as libc::c_long, 2037 as libc::c_int as libc::c_long,
     2000 as libc::c_int as libc::c_long, 1952 as libc::c_int as libc::c_long,
     1893 as libc::c_int as libc::c_long, 1822 as libc::c_int as libc::c_long,
     1739 as libc::c_int as libc::c_long, 1644 as libc::c_int as libc::c_long,
     1535 as libc::c_int as libc::c_long, 1414 as libc::c_int as libc::c_long,
     1280 as libc::c_int as libc::c_long, 1131 as libc::c_int as libc::c_long,
     970 as libc::c_int as libc::c_long, 794 as libc::c_int as libc::c_long,
     605 as libc::c_int as libc::c_long, 402 as libc::c_int as libc::c_long,
     185 as libc::c_int as libc::c_long, -(45 as libc::c_int) as libc::c_long,
     -(288 as libc::c_int) as libc::c_long,
     -(545 as libc::c_int) as libc::c_long,
     -(814 as libc::c_int) as libc::c_long,
     -(1095 as libc::c_int) as libc::c_long,
     -(1388 as libc::c_int) as libc::c_long,
     -(1692 as libc::c_int) as libc::c_long,
     -(2006 as libc::c_int) as libc::c_long,
     -(2330 as libc::c_int) as libc::c_long,
     -(2663 as libc::c_int) as libc::c_long,
     -(3004 as libc::c_int) as libc::c_long,
     -(3351 as libc::c_int) as libc::c_long,
     -(3705 as libc::c_int) as libc::c_long,
     -(4063 as libc::c_int) as libc::c_long,
     -(4425 as libc::c_int) as libc::c_long,
     -(4788 as libc::c_int) as libc::c_long,
     -(5153 as libc::c_int) as libc::c_long,
     -(5517 as libc::c_int) as libc::c_long,
     -(5879 as libc::c_int) as libc::c_long,
     -(6237 as libc::c_int) as libc::c_long,
     -(6589 as libc::c_int) as libc::c_long,
     -(6935 as libc::c_int) as libc::c_long,
     -(7271 as libc::c_int) as libc::c_long,
     -(7597 as libc::c_int) as libc::c_long,
     -(7910 as libc::c_int) as libc::c_long,
     -(8209 as libc::c_int) as libc::c_long,
     -(8491 as libc::c_int) as libc::c_long,
     -(8755 as libc::c_int) as libc::c_long,
     -(8998 as libc::c_int) as libc::c_long,
     -(9219 as libc::c_int) as libc::c_long,
     -(9416 as libc::c_int) as libc::c_long,
     -(9585 as libc::c_int) as libc::c_long,
     -(9727 as libc::c_int) as libc::c_long,
     -(9838 as libc::c_int) as libc::c_long,
     -(9916 as libc::c_int) as libc::c_long,
     -(9959 as libc::c_int) as libc::c_long,
     -(9966 as libc::c_int) as libc::c_long,
     -(9935 as libc::c_int) as libc::c_long,
     -(9863 as libc::c_int) as libc::c_long,
     -(9750 as libc::c_int) as libc::c_long,
     -(9592 as libc::c_int) as libc::c_long,
     -(9389 as libc::c_int) as libc::c_long,
     -(9139 as libc::c_int) as libc::c_long,
     -(8840 as libc::c_int) as libc::c_long,
     -(8492 as libc::c_int) as libc::c_long,
     -(8092 as libc::c_int) as libc::c_long,
     -(7640 as libc::c_int) as libc::c_long,
     -(7134 as libc::c_int) as libc::c_long,
     -(6574 as libc::c_int) as libc::c_long,
     -(5959 as libc::c_int) as libc::c_long,
     -(5288 as libc::c_int) as libc::c_long,
     -(4561 as libc::c_int) as libc::c_long,
     -(3776 as libc::c_int) as libc::c_long,
     -(2935 as libc::c_int) as libc::c_long,
     -(2037 as libc::c_int) as libc::c_long,
     -(1082 as libc::c_int) as libc::c_long,
     -(70 as libc::c_int) as libc::c_long, 998 as libc::c_int as libc::c_long,
     2122 as libc::c_int as libc::c_long, 3300 as libc::c_int as libc::c_long,
     4533 as libc::c_int as libc::c_long, 5818 as libc::c_int as libc::c_long,
     7154 as libc::c_int as libc::c_long, 8540 as libc::c_int as libc::c_long,
     9975 as libc::c_int as libc::c_long,
     11455 as libc::c_int as libc::c_long,
     12980 as libc::c_int as libc::c_long,
     14548 as libc::c_int as libc::c_long,
     16155 as libc::c_int as libc::c_long,
     17799 as libc::c_int as libc::c_long,
     19478 as libc::c_int as libc::c_long,
     21189 as libc::c_int as libc::c_long,
     22929 as libc::c_int as libc::c_long,
     24694 as libc::c_int as libc::c_long,
     26482 as libc::c_int as libc::c_long,
     28289 as libc::c_int as libc::c_long,
     30112 as libc::c_int as libc::c_long,
     31947 as libc::c_int as libc::c_long,
     33791 as libc::c_int as libc::c_long,
     35640 as libc::c_int as libc::c_long,
     37489 as libc::c_int as libc::c_long,
     39336 as libc::c_int as libc::c_long,
     41176 as libc::c_int as libc::c_long,
     43006 as libc::c_int as libc::c_long,
     44821 as libc::c_int as libc::c_long,
     46617 as libc::c_int as libc::c_long,
     48390 as libc::c_int as libc::c_long,
     50137 as libc::c_int as libc::c_long,
     51853 as libc::c_int as libc::c_long,
     53534 as libc::c_int as libc::c_long,
     55178 as libc::c_int as libc::c_long,
     56778 as libc::c_int as libc::c_long,
     58333 as libc::c_int as libc::c_long,
     59838 as libc::c_int as libc::c_long,
     61289 as libc::c_int as libc::c_long,
     62684 as libc::c_int as libc::c_long,
     64019 as libc::c_int as libc::c_long,
     65290 as libc::c_int as libc::c_long,
     66494 as libc::c_int as libc::c_long,
     67629 as libc::c_int as libc::c_long,
     68692 as libc::c_int as libc::c_long,
     69679 as libc::c_int as libc::c_long,
     70590 as libc::c_int as libc::c_long,
     71420 as libc::c_int as libc::c_long,
     72169 as libc::c_int as libc::c_long,
     72835 as libc::c_int as libc::c_long,
     73415 as libc::c_int as libc::c_long,
     73908 as libc::c_int as libc::c_long,
     74313 as libc::c_int as libc::c_long,
     74630 as libc::c_int as libc::c_long,
     74856 as libc::c_int as libc::c_long,
     74992 as libc::c_int as libc::c_long,
     75038 as libc::c_int as libc::c_long];
#[no_mangle]
pub static mut pnts: [*mut libc::c_float; 5] =
    unsafe {
        [cos64.as_ptr() as *mut _, cos32.as_ptr() as *mut _,
         cos16.as_ptr() as *mut _, cos8.as_ptr() as *mut _,
         cos4.as_ptr() as *mut _]
    };
#[no_mangle]
pub unsafe extern "C" fn prepare_decode_tables() {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kr: libc::c_int = 0;
    let mut divv: libc::c_int = 0;
    let mut costab: *mut libc::c_float = 0 as *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        kr = 0x10 as libc::c_int >> i;
        divv = 0x40 as libc::c_int >> i;
        costab = pnts[i as usize];
        k = 0 as libc::c_int;
        while k < kr {
            *costab.offset(k as isize) =
                (1.0f64 /
                     (2.0f64 *
                          cos(3.14159265358979323846f64 *
                                  (k as libc::c_double * 2.0f64 + 1.0f64) /
                                  divv as libc::c_double))) as libc::c_float;
            k += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn make_decode_tables(mut fr: *mut mpg123_handle_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut scaleval: libc::c_double = 0.;
    // scale is always based on 1.0.
    scaleval =
        -0.5f64 *
            (if (*fr).lastscale < 0 as libc::c_int as libc::c_double {
                 (*fr).p.outscale
             } else { (*fr).lastscale });
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if idx < 512 as libc::c_int + 16 as libc::c_int {
            let ref mut fresh0 = *(*fr).decwin.offset(idx as isize);
            *fresh0 =
                (intwinbase[j as usize] as libc::c_double * scaleval) as
                    libc::c_float;
            *(*fr).decwin.offset((idx + 16 as libc::c_int) as isize) = *fresh0
        }
        if i % 32 as libc::c_int == 31 as libc::c_int {
            idx -= 1023 as libc::c_int
        }
        if i % 64 as libc::c_int == 63 as libc::c_int { scaleval = -scaleval }
        i += 1;
        j += 1;
        idx += 32 as libc::c_int
    }
    while i < 512 as libc::c_int {
        if idx < 512 as libc::c_int + 16 as libc::c_int {
            let ref mut fresh1 = *(*fr).decwin.offset(idx as isize);
            *fresh1 =
                (intwinbase[j as usize] as libc::c_double * scaleval) as
                    libc::c_float;
            *(*fr).decwin.offset((idx + 16 as libc::c_int) as isize) = *fresh1
        }
        if i % 32 as libc::c_int == 31 as libc::c_int {
            idx -= 1023 as libc::c_int
        }
        if i % 64 as libc::c_int == 63 as libc::c_int { scaleval = -scaleval }
        i += 1;
        j -= 1;
        idx += 32 as libc::c_int
    };
}
