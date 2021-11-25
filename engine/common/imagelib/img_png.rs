#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type mz_internal_state;
    pub type decallist_s;
    #[no_mangle]
    fn mz_deflateInit(pStream: mz_streamp, level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mz_deflate(pStream: mz_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mz_deflateEnd(pStream: mz_streamp) -> libc::c_int;
    #[no_mangle]
    fn mz_deflateBound(pStream: mz_streamp, source_len: mz_ulong) -> mz_ulong;
    #[no_mangle]
    fn mz_inflateInit2(pStream: mz_streamp, window_bits: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn mz_inflate(pStream: mz_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mz_inflateEnd(pStream: mz_streamp) -> libc::c_int;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn Image_CheckFlag(bit: libc::c_int) -> qboolean;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CRC32_Init(pulCRC: *mut dword);
    #[no_mangle]
    fn CRC32_ProcessBuffer(pulCRC: *mut dword, pBuffer: *const libc::c_void,
                           nBuffer: libc::c_int);
    #[no_mangle]
    fn CRC32_Final(pulCRC: dword) -> dword;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn _Mem_Realloc(poolptr: poolhandle_t, memptr: *mut libc::c_void,
                    size: size_t, clear: qboolean,
                    filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_WriteFile(filename: *const libc::c_char, data: *const libc::c_void,
                    len: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut image: imglib_t;
}
pub type size_t = libc::c_ulong;
pub type mz_ulong = libc::c_ulong;
pub type mz_alloc_func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t, _: size_t)
               -> *mut libc::c_void>;
pub type mz_free_func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub type C2RustUnnamed = libc::c_int;
pub const MZ_DEFAULT_COMPRESSION: C2RustUnnamed = -1;
pub const MZ_DEFAULT_LEVEL: C2RustUnnamed = 6;
pub const MZ_UBER_COMPRESSION: C2RustUnnamed = 10;
pub const MZ_BEST_COMPRESSION: C2RustUnnamed = 9;
pub const MZ_BEST_SPEED: C2RustUnnamed = 1;
pub const MZ_NO_COMPRESSION: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MZ_BLOCK: C2RustUnnamed_0 = 5;
pub const MZ_FINISH: C2RustUnnamed_0 = 4;
pub const MZ_FULL_FLUSH: C2RustUnnamed_0 = 3;
pub const MZ_SYNC_FLUSH: C2RustUnnamed_0 = 2;
pub const MZ_PARTIAL_FLUSH: C2RustUnnamed_0 = 1;
pub const MZ_NO_FLUSH: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_int;
pub const MZ_PARAM_ERROR: C2RustUnnamed_1 = -10000;
pub const MZ_VERSION_ERROR: C2RustUnnamed_1 = -6;
pub const MZ_BUF_ERROR: C2RustUnnamed_1 = -5;
pub const MZ_MEM_ERROR: C2RustUnnamed_1 = -4;
pub const MZ_DATA_ERROR: C2RustUnnamed_1 = -3;
pub const MZ_STREAM_ERROR: C2RustUnnamed_1 = -2;
pub const MZ_ERRNO: C2RustUnnamed_1 = -1;
pub const MZ_NEED_DICT: C2RustUnnamed_1 = 2;
pub const MZ_STREAM_END: C2RustUnnamed_1 = 1;
pub const MZ_OK: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mz_stream_s {
    pub next_in: *const libc::c_uchar,
    pub avail_in: libc::c_uint,
    pub total_in: mz_ulong,
    pub next_out: *mut libc::c_uchar,
    pub avail_out: libc::c_uint,
    pub total_out: mz_ulong,
    pub msg: *mut libc::c_char,
    pub state: *mut mz_internal_state,
    pub zalloc: mz_alloc_func,
    pub zfree: mz_free_func,
    pub opaque: *mut libc::c_void,
    pub data_type: libc::c_int,
    pub adler: mz_ulong,
    pub reserved: mz_ulong,
}
pub type mz_stream = mz_stream_s;
pub type mz_streamp = *mut mz_stream;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type off_t = __off_t;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type rgba_t = [byte; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type fs_offset_t = off_t;
pub type word = libc::c_ushort;
pub type host_status_t = libc::c_uint;
pub const HOST_CRASHED: host_status_t = 6;
pub const HOST_NOFOCUS: host_status_t = 5;
pub const HOST_SLEEP: host_status_t = 4;
pub const HOST_ERR_FATAL: host_status_t = 3;
pub const HOST_SHUTDOWN: host_status_t = 2;
pub const HOST_FRAME: host_status_t = 1;
pub const HOST_INIT: host_status_t = 0;
pub type host_state_t = libc::c_uint;
pub const STATE_GAME_SHUTDOWN: host_state_t = 4;
pub const STATE_CHANGELEVEL: host_state_t = 3;
pub const STATE_LOAD_GAME: host_state_t = 2;
pub const STATE_LOAD_LEVEL: host_state_t = 1;
pub const STATE_RUNFRAME: host_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_status_t {
    pub curstate: host_state_t,
    pub nextstate: host_state_t,
    pub levelName: [libc::c_char; 64],
    pub landmarkName: [libc::c_char; 64],
    pub backgroundMap: qboolean,
    pub loadGame: qboolean,
    pub newGame: qboolean,
}
pub type rdtype_t = libc::c_uint;
pub const RD_PACKET: rdtype_t = 2;
pub const RD_CLIENT: rdtype_t = 1;
pub const RD_NONE: rdtype_t = 0;
pub type netadrtype_t = libc::c_uint;
pub const NA_BROADCAST_IPX: netadrtype_t = 5;
pub const NA_IPX: netadrtype_t = 4;
pub const NA_IP: netadrtype_t = 3;
pub const NA_BROADCAST: netadrtype_t = 2;
pub const NA_LOOPBACK: netadrtype_t = 1;
pub const NA_UNUSED: netadrtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netadr_s {
    pub type_0: netadrtype_t,
    pub ip: [libc::c_uchar; 4],
    pub ipx: [libc::c_uchar; 10],
    pub port: libc::c_ushort,
}
pub type netadr_t = netadr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_redirect_s {
    pub target: rdtype_t,
    pub buffer: *mut libc::c_char,
    pub buffersize: size_t,
    pub address: netadr_t,
    pub flush: Option<unsafe extern "C" fn(_: netadr_t, _: rdtype_t,
                                           _: *mut libc::c_char) -> ()>,
    pub lines: libc::c_int,
}
pub type host_redirect_t = host_redirect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_parm_s {
    pub hInst: HINSTANCE,
    pub hMutex: HANDLE,
    pub status: host_status_t,
    pub game: game_status_t,
    pub type_0: uint,
    pub abortframe: jmp_buf,
    pub errorframe: dword,
    pub mempool: poolhandle_t,
    pub finalmsg: string,
    pub downloadfile: string,
    pub downloadcount: libc::c_int,
    pub deferred_cmd: [libc::c_char; 128],
    pub rd: host_redirect_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub realtime: libc::c_double,
    pub frametime: libc::c_double,
    pub realframetime: libc::c_double,
    pub framecount: uint,
    pub draw_decals: [[libc::c_char; 64]; 512],
    pub player_mins: [vec3_t; 4],
    pub player_maxs: [vec3_t; 4],
    pub hWnd: *mut libc::c_void,
    pub allow_console: qboolean,
    pub allow_console_init: qboolean,
    pub key_overstrike: qboolean,
    pub stuffcmds_pending: qboolean,
    pub allow_cheats: qboolean,
    pub con_showalways: qboolean,
    pub change_game: qboolean,
    pub mouse_visible: qboolean,
    pub shutdown_issued: qboolean,
    pub force_draw_version: qboolean,
    pub force_draw_version_time: libc::c_float,
    pub apply_game_config: qboolean,
    pub apply_opengl_config: qboolean,
    pub config_executed: qboolean,
    pub sv_cvars_restored: libc::c_int,
    pub crashed: qboolean,
    pub daemonized: qboolean,
    pub enabledll: qboolean,
    pub textmode: qboolean,
    pub userinfo_changed: qboolean,
    pub movevars_changed: qboolean,
    pub renderinfo_changed: qboolean,
    pub rootdir: [libc::c_char; 260],
    pub rodir: [libc::c_char; 260],
    pub gamefolder: [libc::c_char; 64],
    pub imagepool: poolhandle_t,
    pub soundpool: poolhandle_t,
    pub features: uint,
    pub window_center_x: libc::c_int,
    pub window_center_y: libc::c_int,
    pub decalList: *mut decallist_s,
    pub numdecals: libc::c_int,
}
pub type host_parm_t = host_parm_s;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const PF_TOTALCOUNT: C2RustUnnamed_2 = 12;
pub const PF_ATI2: C2RustUnnamed_2 = 11;
pub const PF_DXT5: C2RustUnnamed_2 = 10;
pub const PF_DXT3: C2RustUnnamed_2 = 9;
pub const PF_DXT1: C2RustUnnamed_2 = 8;
pub const PF_LUMINANCE: C2RustUnnamed_2 = 7;
pub const PF_BGR_24: C2RustUnnamed_2 = 6;
pub const PF_RGB_24: C2RustUnnamed_2 = 5;
pub const PF_BGRA_32: C2RustUnnamed_2 = 4;
pub const PF_RGBA_32: C2RustUnnamed_2 = 3;
pub const PF_INDEXED_32: C2RustUnnamed_2 = 2;
pub const PF_INDEXED_24: C2RustUnnamed_2 = 1;
pub const PF_UNKNOWN: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const IL_OVERVIEW: C2RustUnnamed_3 = 64;
pub const IL_LOAD_DECAL: C2RustUnnamed_3 = 32;
pub const IL_DDS_HARDWARE: C2RustUnnamed_3 = 16;
pub const IL_DONTFLIP_TGA: C2RustUnnamed_3 = 8;
pub const IL_ALLOW_OVERWRITE: C2RustUnnamed_3 = 4;
pub const IL_KEEP_8BIT: C2RustUnnamed_3 = 2;
pub const IL_USE_LERPING: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const IMAGE_REMAP: C2RustUnnamed_4 = 134217728;
pub const IMAGE_LIGHTGAMMA: C2RustUnnamed_4 = 67108864;
pub const IMAGE_QUANTIZE: C2RustUnnamed_4 = 33554432;
pub const IMAGE_MAKE_LUMA: C2RustUnnamed_4 = 16777216;
pub const IMAGE_FORCE_RGBA: C2RustUnnamed_4 = 8388608;
pub const IMAGE_RESAMPLE: C2RustUnnamed_4 = 1048576;
pub const IMAGE_EMBOSS: C2RustUnnamed_4 = 524288;
pub const IMAGE_ROT270: C2RustUnnamed_4 = 458752;
pub const IMAGE_ROT180: C2RustUnnamed_4 = 196608;
pub const IMAGE_ROT_90: C2RustUnnamed_4 = 262144;
pub const IMAGE_FLIP_Y: C2RustUnnamed_4 = 131072;
pub const IMAGE_FLIP_X: C2RustUnnamed_4 = 65536;
pub const IMAGE_QUAKEPAL: C2RustUnnamed_4 = 1024;
pub const IMAGE_ONEBIT_ALPHA: C2RustUnnamed_4 = 512;
pub const IMAGE_MULTILAYER: C2RustUnnamed_4 = 256;
pub const IMAGE_DDS_FORMAT: C2RustUnnamed_4 = 128;
pub const IMAGE_QUAKESKY: C2RustUnnamed_4 = 64;
pub const IMAGE_SKYBOX: C2RustUnnamed_4 = 32;
pub const IMAGE_HAS_LUMA: C2RustUnnamed_4 = 16;
pub const IMAGE_COLORINDEX: C2RustUnnamed_4 = 8;
pub const IMAGE_HAS_COLOR: C2RustUnnamed_4 = 4;
pub const IMAGE_HAS_ALPHA: C2RustUnnamed_4 = 2;
pub const IMAGE_CUBEMAP: C2RustUnnamed_4 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgbdata_s {
    pub width: word,
    pub height: word,
    pub depth: word,
    pub type_0: uint,
    pub flags: uint,
    pub encode: word,
    pub numMips: byte,
    pub palette: *mut byte,
    pub buffer: *mut byte,
    pub fogParams: rgba_t,
    pub size: size_t,
}
pub type rgbdata_t = rgbdata_s;
pub type image_hint_t = libc::c_uint;
pub const IL_HINT_HL: image_hint_t = 2;
pub const IL_HINT_Q1: image_hint_t = 1;
pub const IL_HINT_NO: image_hint_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loadformat_s {
    pub formatstring: *const libc::c_char,
    pub ext: *const libc::c_char,
    pub loadfunc: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const byte, _: fs_offset_t)
                             -> qboolean>,
    pub hint: image_hint_t,
}
pub type loadpixformat_t = loadformat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saveformat_s {
    pub formatstring: *const libc::c_char,
    pub ext: *const libc::c_char,
    pub savefunc: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *mut rgbdata_t) -> qboolean>,
}
pub type savepixformat_t = saveformat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imglib_s {
    pub loadformats: *const loadpixformat_t,
    pub saveformats: *const savepixformat_t,
    pub width: word,
    pub height: word,
    pub depth: word,
    pub num_mips: byte,
    pub encode: word,
    pub type_0: uint,
    pub flags: uint,
    pub size: size_t,
    pub ptr: uint,
    pub bpp: libc::c_int,
    pub rgba: *mut byte,
    pub source_width: libc::c_int,
    pub source_height: libc::c_int,
    pub source_type: uint,
    pub num_sides: libc::c_int,
    pub cubemap: *mut byte,
    pub d_currentpal: *mut uint,
    pub d_rendermode: libc::c_int,
    pub palette: *mut byte,
    pub fogParams: rgba_t,
    pub hint: image_hint_t,
    pub tempbuffer: *mut byte,
    pub cmd_flags: libc::c_int,
    pub force_flags: libc::c_int,
    pub custom_palette: qboolean,
}
pub type imglib_t = imglib_s;
pub const PNG_CT_RGB: png_colortype = 2;
pub type png_ihdr_t = png_ihdr_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct png_ihdr_s {
    pub width: uint32_t,
    pub height: uint32_t,
    pub bitdepth: uint8_t,
    pub colortype: uint8_t,
    pub compression: uint8_t,
    pub filter: uint8_t,
    pub interlace: uint8_t,
}
pub type png_t = png_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct png_s {
    pub sign: [uint8_t; 8],
    pub ihdr_len: uint32_t,
    pub ihdr_sign: [uint8_t; 4],
    pub ihdr_chunk: png_ihdr_t,
    pub ihdr_crc32: uint32_t,
}
pub const PNG_F_PAETH: png_filter = 4;
pub const PNG_F_AVERAGE: png_filter = 3;
pub const PNG_F_UP: png_filter = 2;
pub const PNG_F_SUB: png_filter = 1;
pub const PNG_F_NONE: png_filter = 0;
pub const PNG_CT_RGBA: png_colortype = 6;
pub type png_footer_t = png_footer_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct png_footer_s {
    pub idat_crc32: uint32_t,
    pub iend_len: uint32_t,
    pub iend_sign: [uint8_t; 4],
    pub iend_crc32: uint32_t,
}
pub type png_colortype = libc::c_uint;
pub const PNG_CT_ALPHA: png_colortype = 4;
pub const PNG_CT_PALLETE: png_colortype = 1;
pub const PNG_CT_GREY: png_colortype = 0;
pub type png_filter = libc::c_uint;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
               (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
               (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
               (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
/*
img_png.c - png format load & save
Copyright (C) 2019 Andrey Akhmichin

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
static mut png_sign: [libc::c_char; 8] =
    [0x89 as libc::c_int as libc::c_char, 'P' as i32 as libc::c_char,
     'N' as i32 as libc::c_char, 'G' as i32 as libc::c_char,
     '\r' as i32 as libc::c_char, '\n' as i32 as libc::c_char,
     0x1a as libc::c_int as libc::c_char, '\n' as i32 as libc::c_char];
static mut ihdr_sign: [libc::c_char; 4] =
    ['I' as i32 as libc::c_char, 'H' as i32 as libc::c_char,
     'D' as i32 as libc::c_char, 'R' as i32 as libc::c_char];
static mut idat_sign: [libc::c_char; 4] =
    ['I' as i32 as libc::c_char, 'D' as i32 as libc::c_char,
     'A' as i32 as libc::c_char, 'T' as i32 as libc::c_char];
static mut iend_sign: [libc::c_char; 4] =
    ['I' as i32 as libc::c_char, 'E' as i32 as libc::c_char,
     'N' as i32 as libc::c_char, 'D' as i32 as libc::c_char];
static mut iend_crc32: libc::c_int =
    0xae426082 as libc::c_uint as libc::c_int;
/*
=============
Image_LoadPNG
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadPNG(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut ret: libc::c_int = 0;
    let mut p: libc::c_short = 0;
    let mut a: libc::c_short = 0;
    let mut b: libc::c_short = 0;
    let mut c: libc::c_short = 0;
    let mut pa: libc::c_short = 0;
    let mut pb: libc::c_short = 0;
    let mut pc: libc::c_short = 0;
    let mut buf_p: *mut byte = 0 as *mut byte;
    let mut pixbuf: *mut byte = 0 as *mut byte;
    let mut raw: *mut byte = 0 as *mut byte;
    let mut prior: *mut byte = 0 as *mut byte;
    let mut idat_buf: *mut byte = 0 as *mut byte;
    let mut uncompressed_buffer: *mut byte = 0 as *mut byte;
    let mut rowend: *mut byte = 0 as *mut byte;
    let mut chunk_len: uint = 0;
    let mut mz_crc32: uint = 0;
    let mut crc32_check: uint = 0;
    let mut oldsize: uint = 0 as libc::c_int as uint;
    let mut newsize: uint = 0 as libc::c_int as uint;
    let mut rowsize: uint = 0;
    let mut uncompressed_size: uint = 0;
    let mut pixel_size: uint = 0;
    let mut i: uint = 0;
    let mut y: uint = 0;
    let mut filter_type: uint = 0;
    let mut chunk_sign: uint = 0;
    let mut has_iend_chunk: qboolean = false_0;
    let mut stream: mz_stream =
        {
            let mut init =
                mz_stream_s{next_in: 0 as *const libc::c_uchar,
                            avail_in: 0,
                            total_in: 0,
                            next_out: 0 as *mut libc::c_uchar,
                            avail_out: 0,
                            total_out: 0,
                            msg: 0 as *mut libc::c_char,
                            state: 0 as *mut mz_internal_state,
                            zalloc: None,
                            zfree: None,
                            opaque: 0 as *mut libc::c_void,
                            data_type: 0,
                            adler: 0,
                            reserved: 0,};
            init
        };
    let mut png_hdr: png_t =
        png_t{sign: [0; 8],
              ihdr_len: 0,
              ihdr_sign: [0; 4],
              ihdr_chunk:
                  png_ihdr_t{width: 0,
                             height: 0,
                             bitdepth: 0,
                             colortype: 0,
                             compression: 0,
                             filter: 0,
                             interlace: 0,},
              ihdr_crc32: 0,};
    if (filesize as libc::c_ulong) <
           ::std::mem::size_of::<png_t>() as libc::c_ulong {
        return false_0
    }
    buf_p = buffer as *mut byte;
    // get png header
    memcpy(&mut png_hdr as *mut png_t as *mut libc::c_void,
           buffer as *const libc::c_void,
           ::std::mem::size_of::<png_t>() as libc::c_ulong);
    // check png signature
    if memcmp(png_hdr.sign.as_mut_ptr() as *const libc::c_void,
              png_sign.as_ptr() as *const libc::c_void,
              ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong) !=
           0 {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Invalid PNG signature (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    // convert IHDR chunk length to little endian
    png_hdr.ihdr_len = __bswap_32(png_hdr.ihdr_len);
    // check IHDR chunk length (valid value - 13)
    if png_hdr.ihdr_len as libc::c_ulong !=
           ::std::mem::size_of::<png_ihdr_t>() as libc::c_ulong {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Invalid IHDR chunk size (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    // check IHDR chunk signature
    if memcmp(png_hdr.ihdr_sign.as_mut_ptr() as *const libc::c_void,
              ihdr_sign.as_ptr() as *const libc::c_void,
              ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong) !=
           0 {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: IHDR chunk corrupted (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    // convert image width and height to little endian
    png_hdr.ihdr_chunk.height = __bswap_32(png_hdr.ihdr_chunk.height);
    png_hdr.ihdr_chunk.width = __bswap_32(png_hdr.ihdr_chunk.width);
    if png_hdr.ihdr_chunk.height == 0 as libc::c_int as libc::c_uint ||
           png_hdr.ihdr_chunk.width == 0 as libc::c_int as libc::c_uint {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Invalid image size %dx%d (%s)\n\x00"
                        as *const u8 as *const libc::c_char,
                    png_hdr.ihdr_chunk.width, png_hdr.ihdr_chunk.height,
                    name);
        return false_0
    }
    if png_hdr.ihdr_chunk.bitdepth as libc::c_int != 8 as libc::c_int {
        Con_DPrintf(b"^3Warning:^7 Image_LoadPNG: Only 8-bit images is supported (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    if png_hdr.ihdr_chunk.colortype as libc::c_int !=
           PNG_CT_RGB as libc::c_int &&
           png_hdr.ihdr_chunk.colortype as libc::c_int !=
               PNG_CT_RGBA as libc::c_int {
        Con_DPrintf(b"^3Warning:^7 Image_LoadPNG: Only 8-bit RGB and RGBA images is supported (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    if png_hdr.ihdr_chunk.compression as libc::c_int > 0 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Unknown compression method (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    if png_hdr.ihdr_chunk.filter as libc::c_int > 0 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Unknown filter type (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    if png_hdr.ihdr_chunk.interlace as libc::c_int == 1 as libc::c_int {
        Con_DPrintf(b"^3Warning:^7 Image_LoadPNG: Adam7 Interlacing not supported (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    if png_hdr.ihdr_chunk.interlace as libc::c_int > 0 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Unknown interlacing type (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    // calculate IHDR chunk CRC
    CRC32_Init(&mut crc32_check);
    CRC32_ProcessBuffer(&mut crc32_check,
                        buf_p.offset(::std::mem::size_of::<[uint8_t; 8]>() as
                                         libc::c_ulong as
                                         isize).offset(::std::mem::size_of::<uint32_t>()
                                                           as libc::c_ulong as
                                                           isize) as
                            *const libc::c_void,
                        (png_hdr.ihdr_len as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<[uint8_t; 4]>()
                                                             as libc::c_ulong)
                            as libc::c_int);
    crc32_check = CRC32_Final(crc32_check);
    // check IHDR chunk CRC
    if __bswap_32(png_hdr.ihdr_crc32) != crc32_check {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: IHDR chunk has wrong CRC32 sum (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    // move pointer
    buf_p =
        buf_p.offset(::std::mem::size_of::<png_t>() as libc::c_ulong as
                         isize);
    // find all critical chunks
    while has_iend_chunk as u64 == 0 &&
              (buf_p.wrapping_offset_from(buffer) as libc::c_long) < filesize
          {
        // get chunk length
        memcpy(&mut chunk_len as *mut uint as *mut libc::c_void,
               buf_p as *const libc::c_void,
               ::std::mem::size_of::<uint>() as libc::c_ulong);
        // convert chunk length to little endian
        chunk_len = __bswap_32(chunk_len);
        if chunk_len > 2147483647 as libc::c_int as libc::c_uint {
            Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Found chunk with wrong size (%s)\n\x00"
                            as *const u8 as *const libc::c_char, name);
            _Mem_Free(idat_buf as *mut libc::c_void,
                      b"../engine/common/imagelib/img_png.c\x00" as *const u8
                          as *const libc::c_char, 155 as libc::c_int);
            return false_0
        }
        // move pointer
        buf_p =
            buf_p.offset(::std::mem::size_of::<uint>() as libc::c_ulong as
                             isize);
        // get all IDAT chunks data
        if memcmp(buf_p as *const libc::c_void,
                  idat_sign.as_ptr() as *const libc::c_void,
                  ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
               == 0 {
            newsize = oldsize.wrapping_add(chunk_len);
            idat_buf =
                _Mem_Realloc(host.imagepool, idat_buf as *mut libc::c_void,
                             newsize as size_t, true_0,
                             b"../engine/common/imagelib/img_png.c\x00" as
                                 *const u8 as *const libc::c_char,
                             166 as libc::c_int) as *mut byte;
            memcpy(idat_buf.offset(oldsize as isize) as *mut libc::c_void,
                   buf_p.offset(::std::mem::size_of::<[libc::c_char; 4]>() as
                                    libc::c_ulong as isize) as
                       *const libc::c_void, chunk_len as libc::c_ulong);
            oldsize = newsize
        } else if memcmp(buf_p as *const libc::c_void,
                         iend_sign.as_ptr() as *const libc::c_void,
                         ::std::mem::size_of::<[libc::c_char; 4]>() as
                             libc::c_ulong) == 0 {
            has_iend_chunk = true_0
        }
        // calculate chunk CRC
        CRC32_Init(&mut crc32_check);
        CRC32_ProcessBuffer(&mut crc32_check, buf_p as *const libc::c_void,
                            (chunk_len as
                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 4]>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int);
        crc32_check = CRC32_Final(crc32_check);
        // move pointer
        buf_p =
            buf_p.offset(::std::mem::size_of::<uint>() as libc::c_ulong as
                             isize);
        buf_p = buf_p.offset(chunk_len as isize);
        // get real chunk CRC
        memcpy(&mut mz_crc32 as *mut uint as *mut libc::c_void,
               buf_p as *const libc::c_void,
               ::std::mem::size_of::<uint>() as libc::c_ulong);
        // check chunk CRC
        if __bswap_32(mz_crc32) != crc32_check {
            Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Found chunk with wrong CRC32 sum (%s)\n\x00"
                            as *const u8 as *const libc::c_char, name);
            _Mem_Free(idat_buf as *mut libc::c_void,
                      b"../engine/common/imagelib/img_png.c\x00" as *const u8
                          as *const libc::c_char, 189 as libc::c_int);
            return false_0
        }
        // move pointer
        buf_p =
            buf_p.offset(::std::mem::size_of::<uint>() as libc::c_ulong as
                             isize)
    } // make compiler happy
    if has_iend_chunk as u64 == 0 {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: IEND chunk not found (%s)\n\x00"
                        as *const u8 as *const libc::c_char,
                    name); // always exctracted to 32-bit buffer
        _Mem_Free(idat_buf as *mut libc::c_void,
                  b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                      *const libc::c_char,
                  200 as libc::c_int); // +1 for filter
        return false_0
    }
    if chunk_len != 0 as libc::c_int as libc::c_uint {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: IEND chunk has wrong size (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        _Mem_Free(idat_buf as *mut libc::c_void,
                  b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                      *const libc::c_char, 207 as libc::c_int);
        return false_0
    }
    if oldsize == 0 as libc::c_int as libc::c_uint {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Couldn\'t find IDAT chunks (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    match png_hdr.ihdr_chunk.colortype as libc::c_int {
        2 => { pixel_size = 3 as libc::c_int as uint }
        6 => { pixel_size = 4 as libc::c_int as uint }
        _ => {
            pixel_size = 0 as libc::c_int as uint;
            if false_0 as libc::c_int == 0 {
                Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                              *const libc::c_char,
                          b"../engine/common/imagelib/img_png.c\x00" as
                              *const u8 as *const libc::c_char,
                          227 as libc::c_int);
            }
        }
    }
    image.type_0 = PF_RGBA_32 as libc::c_int as uint;
    image.width = png_hdr.ihdr_chunk.width as word;
    image.height = png_hdr.ihdr_chunk.height as word;
    image.size =
        (image.height as libc::c_int * image.width as libc::c_int *
             4 as libc::c_int) as size_t;
    image.flags |= IMAGE_HAS_COLOR as libc::c_int as libc::c_uint;
    if png_hdr.ihdr_chunk.colortype as libc::c_int ==
           PNG_CT_RGBA as libc::c_int {
        image.flags |= IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
    }
    image.depth = 1 as libc::c_int as word;
    rowsize = pixel_size.wrapping_mul(image.width as libc::c_uint);
    uncompressed_size =
        (image.height as
             libc::c_uint).wrapping_mul(rowsize.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint));
    uncompressed_buffer =
        _Mem_Alloc(host.imagepool, uncompressed_size as size_t, false_0,
                   b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                       *const libc::c_char, 245 as libc::c_int) as *mut byte;
    stream.next_in = idat_buf;
    stream.avail_in = newsize;
    stream.total_in = stream.avail_in as mz_ulong;
    stream.next_out = uncompressed_buffer;
    stream.avail_out = uncompressed_size;
    stream.total_out = stream.avail_out as mz_ulong;
    // uncompress image
    if mz_inflateInit2(&mut stream, 15 as libc::c_int) != MZ_OK as libc::c_int
       {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: IDAT chunk decompression failed (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        _Mem_Free(uncompressed_buffer as *mut libc::c_void,
                  b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                      *const libc::c_char, 256 as libc::c_int);
        _Mem_Free(idat_buf as *mut libc::c_void,
                  b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                      *const libc::c_char, 257 as libc::c_int);
        return false_0
    }
    ret = mz_inflate(&mut stream, MZ_NO_FLUSH as libc::c_int);
    mz_inflateEnd(&mut stream);
    _Mem_Free(idat_buf as *mut libc::c_void,
              b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                  *const libc::c_char, 264 as libc::c_int);
    if ret != MZ_OK as libc::c_int && ret != MZ_STREAM_END as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Image_LoadPNG: IDAT chunk decompression failed (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        _Mem_Free(uncompressed_buffer as *mut libc::c_void,
                  b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                      *const libc::c_char, 269 as libc::c_int);
        return false_0
    }
    image.rgba =
        _Mem_Alloc(host.imagepool, image.size, false_0,
                   b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                       *const libc::c_char, 273 as libc::c_int) as *mut byte;
    pixbuf = image.rgba;
    prior = pixbuf;
    i = 0 as libc::c_int as uint;
    raw = uncompressed_buffer;
    if png_hdr.ihdr_chunk.colortype as libc::c_int ==
           PNG_CT_RGB as libc::c_int {
        pixbuf = raw;
        prior = pixbuf
    }
    let fresh0 = raw;
    raw = raw.offset(1);
    filter_type = *fresh0 as uint;
    // decode adaptive filter
    match filter_type {
        0 | 2 => {
            while i < rowsize {
                *pixbuf.offset(i as isize) = *raw.offset(i as isize);
                i = i.wrapping_add(1)
            }
        }
        1 | 4 => {
            while i < pixel_size {
                *pixbuf.offset(i as isize) = *raw.offset(i as isize);
                i = i.wrapping_add(1)
            }
            while i < rowsize {
                *pixbuf.offset(i as isize) =
                    (*raw.offset(i as isize) as libc::c_int +
                         *pixbuf.offset(i.wrapping_sub(pixel_size) as isize)
                             as libc::c_int) as byte;
                i = i.wrapping_add(1)
            }
        }
        3 => {
            while i < pixel_size {
                *pixbuf.offset(i as isize) = *raw.offset(i as isize);
                i = i.wrapping_add(1)
            }
            while i < rowsize {
                *pixbuf.offset(i as isize) =
                    (*raw.offset(i as isize) as libc::c_int +
                         (*pixbuf.offset(i.wrapping_sub(pixel_size) as isize)
                              as libc::c_int >> 1 as libc::c_int)) as byte;
                i = i.wrapping_add(1)
            }
        }
        _ => {
            Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Found unknown filter type (%s)\n\x00"
                            as *const u8 as *const libc::c_char, name);
            _Mem_Free(uncompressed_buffer as *mut libc::c_void,
                      b"../engine/common/imagelib/img_png.c\x00" as *const u8
                          as *const libc::c_char, 309 as libc::c_int);
            _Mem_Free(image.rgba as *mut libc::c_void,
                      b"../engine/common/imagelib/img_png.c\x00" as *const u8
                          as *const libc::c_char, 310 as libc::c_int);
            return false_0
        }
    }
    y = 1 as libc::c_int as uint;
    while y < image.height as libc::c_uint {
        i = 0 as libc::c_int as uint;
        pixbuf = pixbuf.offset(rowsize as isize);
        raw = raw.offset(rowsize as isize);
        let fresh1 = raw;
        raw = raw.offset(1);
        filter_type = *fresh1 as uint;
        match filter_type {
            0 => {
                while i < rowsize {
                    *pixbuf.offset(i as isize) = *raw.offset(i as isize);
                    i = i.wrapping_add(1)
                }
            }
            1 => {
                while i < pixel_size {
                    *pixbuf.offset(i as isize) = *raw.offset(i as isize);
                    i = i.wrapping_add(1)
                }
                while i < rowsize {
                    *pixbuf.offset(i as isize) =
                        (*raw.offset(i as isize) as libc::c_int +
                             *pixbuf.offset(i.wrapping_sub(pixel_size) as
                                                isize) as libc::c_int) as
                            byte;
                    i = i.wrapping_add(1)
                }
            }
            2 => {
                while i < rowsize {
                    *pixbuf.offset(i as isize) =
                        (*raw.offset(i as isize) as libc::c_int +
                             *prior.offset(i as isize) as libc::c_int) as
                            byte;
                    i = i.wrapping_add(1)
                }
            }
            3 => {
                while i < pixel_size {
                    *pixbuf.offset(i as isize) =
                        (*raw.offset(i as isize) as libc::c_int +
                             (*prior.offset(i as isize) as libc::c_int >>
                                  1 as libc::c_int)) as byte;
                    i = i.wrapping_add(1)
                }
                while i < rowsize {
                    *pixbuf.offset(i as isize) =
                        (*raw.offset(i as isize) as libc::c_int +
                             (*pixbuf.offset(i.wrapping_sub(pixel_size) as
                                                 isize) as libc::c_int +
                                  *prior.offset(i as isize) as libc::c_int >>
                                  1 as libc::c_int)) as byte;
                    i = i.wrapping_add(1)
                }
            }
            4 => {
                while i < pixel_size {
                    *pixbuf.offset(i as isize) =
                        (*raw.offset(i as isize) as libc::c_int +
                             *prior.offset(i as isize) as libc::c_int) as
                            byte;
                    i = i.wrapping_add(1)
                }
                while i < rowsize {
                    a =
                        *pixbuf.offset(i.wrapping_sub(pixel_size) as isize) as
                            libc::c_short;
                    b = *prior.offset(i as isize) as libc::c_short;
                    c =
                        *prior.offset(i.wrapping_sub(pixel_size) as isize) as
                            libc::c_short;
                    p =
                        (a as libc::c_int + b as libc::c_int -
                             c as libc::c_int) as libc::c_short;
                    pa =
                        abs(p as libc::c_int - a as libc::c_int) as
                            libc::c_short;
                    pb =
                        abs(p as libc::c_int - b as libc::c_int) as
                            libc::c_short;
                    pc =
                        abs(p as libc::c_int - c as libc::c_int) as
                            libc::c_short;
                    *pixbuf.offset(i as isize) = *raw.offset(i as isize);
                    if (pc as libc::c_int) < pa as libc::c_int &&
                           (pc as libc::c_int) < pb as libc::c_int {
                        let ref mut fresh2 = *pixbuf.offset(i as isize);
                        *fresh2 =
                            (*fresh2 as libc::c_int + c as libc::c_int) as
                                byte
                    } else if (pb as libc::c_int) < pa as libc::c_int {
                        let ref mut fresh3 = *pixbuf.offset(i as isize);
                        *fresh3 =
                            (*fresh3 as libc::c_int + b as libc::c_int) as
                                byte
                    } else {
                        let ref mut fresh4 = *pixbuf.offset(i as isize);
                        *fresh4 =
                            (*fresh4 as libc::c_int + a as libc::c_int) as
                                byte
                    }
                    i = i.wrapping_add(1)
                }
            }
            _ => {
                Con_DPrintf(b"^1Error:^7 Image_LoadPNG: Found unknown filter type (%s)\n\x00"
                                as *const u8 as *const libc::c_char, name);
                _Mem_Free(uncompressed_buffer as *mut libc::c_void,
                          b"../engine/common/imagelib/img_png.c\x00" as
                              *const u8 as *const libc::c_char,
                          373 as libc::c_int);
                _Mem_Free(image.rgba as *mut libc::c_void,
                          b"../engine/common/imagelib/img_png.c\x00" as
                              *const u8 as *const libc::c_char,
                          374 as libc::c_int);
                return false_0
            }
        }
        prior = pixbuf;
        y = y.wrapping_add(1)
    }
    // convert RGB-to-RGBA
    if png_hdr.ihdr_chunk.colortype as libc::c_int ==
           PNG_CT_RGB as libc::c_int {
        pixbuf = image.rgba;
        raw = uncompressed_buffer;
        y = 0 as libc::c_int as uint;
        while y < image.height as libc::c_uint {
            rowend = raw.offset(rowsize as isize);
            while raw < rowend {
                let fresh5 = pixbuf;
                pixbuf = pixbuf.offset(1);
                *fresh5 = *raw.offset(0 as libc::c_int as isize);
                let fresh6 = pixbuf;
                pixbuf = pixbuf.offset(1);
                *fresh6 = *raw.offset(1 as libc::c_int as isize);
                let fresh7 = pixbuf;
                pixbuf = pixbuf.offset(1);
                *fresh7 = *raw.offset(2 as libc::c_int as isize);
                let fresh8 = pixbuf;
                pixbuf = pixbuf.offset(1);
                *fresh8 = 0xff as libc::c_int as byte;
                raw = raw.offset(pixel_size as isize)
            }
            y = y.wrapping_add(1)
        }
    }
    _Mem_Free(uncompressed_buffer as *mut libc::c_void,
              b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                  *const libc::c_char, 400 as libc::c_int);
    return true_0;
}
/*
=============
Image_SavePNG
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_SavePNG(mut name: *const libc::c_char,
                                       mut pix: *mut rgbdata_t) -> qboolean {
    let mut ret: libc::c_int = 0; // already existed
    let mut y: uint = 0;
    let mut outsize: uint = 0;
    let mut pixel_size: uint = 0;
    let mut filtered_size: uint = 0;
    let mut idat_len: uint = 0;
    let mut ihdr_len: uint = 0;
    let mut mz_crc32: uint = 0;
    let mut rowsize: uint = 0;
    let mut big_idat_len: uint = 0;
    let mut in_0: *mut byte = 0 as *mut byte;
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut out: *mut byte = 0 as *mut byte;
    let mut filtered_buffer: *mut byte = 0 as *mut byte;
    let mut rowend: *mut byte = 0 as *mut byte;
    let mut stream: mz_stream =
        {
            let mut init =
                mz_stream_s{next_in: 0 as *const libc::c_uchar,
                            avail_in: 0,
                            total_in: 0,
                            next_out: 0 as *mut libc::c_uchar,
                            avail_out: 0,
                            total_out: 0,
                            msg: 0 as *mut libc::c_char,
                            state: 0 as *mut mz_internal_state,
                            zalloc: None,
                            zfree: None,
                            opaque: 0 as *mut libc::c_void,
                            data_type: 0,
                            adler: 0,
                            reserved: 0,};
            init
        };
    let mut png_hdr: png_t =
        png_t{sign: [0; 8],
              ihdr_len: 0,
              ihdr_sign: [0; 4],
              ihdr_chunk:
                  png_ihdr_t{width: 0,
                             height: 0,
                             bitdepth: 0,
                             colortype: 0,
                             compression: 0,
                             filter: 0,
                             interlace: 0,},
              ihdr_crc32: 0,};
    let mut png_ftr: png_footer_t =
        png_footer_t{idat_crc32: 0,
                     iend_len: 0,
                     iend_sign: [0; 4],
                     iend_crc32: 0,};
    if FS_FileExists(name, false_0 as libc::c_int) != 0 &&
           Image_CheckFlag(IL_ALLOW_OVERWRITE as libc::c_int) as u64 == 0 {
        return false_0
    }
    // bogus parameter check
    if (*pix).buffer.is_null() { return false_0 }
    // get image description
    match (*pix).type_0 {
        6 | 5 => { pixel_size = 3 as libc::c_int as uint }
        4 | 3 => { pixel_size = 4 as libc::c_int as uint }
        _ => { return false_0 }
    }
    rowsize = ((*pix).width as libc::c_uint).wrapping_mul(pixel_size);
    // get filtered image size
    filtered_size =
        rowsize.wrapping_add(1 as libc::c_int as
                                 libc::c_uint).wrapping_mul((*pix).height as
                                                                libc::c_uint);
    filtered_buffer =
        _Mem_Alloc(host.imagepool, filtered_size as size_t, false_0,
                   b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                       *const libc::c_char, 443 as libc::c_int) as *mut byte;
    out = filtered_buffer;
    // apply adaptive filter to image
    match (*pix).type_0 {
        5 | 3 => {
            y = 0 as libc::c_int as uint;
            while y < (*pix).height as libc::c_uint {
                in_0 =
                    (*pix).buffer.offset(y.wrapping_mul((*pix).width as
                                                            libc::c_uint).wrapping_mul(pixel_size)
                                             as isize);
                let fresh9 = out;
                out = out.offset(1);
                *fresh9 = PNG_F_NONE as libc::c_int as byte;
                rowend = in_0.offset(rowsize as isize);
                while in_0 < rowend {
                    let fresh10 = out;
                    out = out.offset(1);
                    *fresh10 = *in_0.offset(0 as libc::c_int as isize);
                    let fresh11 = out;
                    out = out.offset(1);
                    *fresh11 = *in_0.offset(1 as libc::c_int as isize);
                    let fresh12 = out;
                    out = out.offset(1);
                    *fresh12 = *in_0.offset(2 as libc::c_int as isize);
                    if (*pix).flags &
                           IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint != 0
                       {
                        let fresh13 = out;
                        out = out.offset(1);
                        *fresh13 = *in_0.offset(3 as libc::c_int as isize)
                    }
                    in_0 = in_0.offset(pixel_size as isize)
                }
                y = y.wrapping_add(1)
            }
        }
        6 | 4 => {
            y = 0 as libc::c_int as uint;
            while y < (*pix).height as libc::c_uint {
                in_0 =
                    (*pix).buffer.offset(y.wrapping_mul((*pix).width as
                                                            libc::c_uint).wrapping_mul(pixel_size)
                                             as isize);
                let fresh14 = out;
                out = out.offset(1);
                *fresh14 = PNG_F_NONE as libc::c_int as byte;
                rowend = in_0.offset(rowsize as isize);
                while in_0 < rowend {
                    let fresh15 = out;
                    out = out.offset(1);
                    *fresh15 = *in_0.offset(2 as libc::c_int as isize);
                    let fresh16 = out;
                    out = out.offset(1);
                    *fresh16 = *in_0.offset(1 as libc::c_int as isize);
                    let fresh17 = out;
                    out = out.offset(1);
                    *fresh17 = *in_0.offset(0 as libc::c_int as isize);
                    if (*pix).flags &
                           IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint != 0
                       {
                        let fresh18 = out;
                        out = out.offset(1);
                        *fresh18 = *in_0.offset(3 as libc::c_int as isize)
                    }
                    in_0 = in_0.offset(pixel_size as isize)
                }
                y = y.wrapping_add(1)
            }
        }
        _ => { }
    }
    // get IHDR chunk length
    ihdr_len = ::std::mem::size_of::<png_ihdr_t>() as libc::c_ulong as uint;
    // predict IDAT chunk length
    idat_len =
        mz_deflateBound(0 as mz_streamp, filtered_size as mz_ulong) as uint;
    // calculate PNG filesize
    outsize = ::std::mem::size_of::<png_t>() as libc::c_ulong as uint;
    outsize =
        (outsize as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint>() as
                                             libc::c_ulong) as uint as uint;
    outsize =
        (outsize as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 4]>()
                                             as libc::c_ulong) as uint as
            uint;
    outsize =
        (outsize as libc::c_uint).wrapping_add(idat_len) as uint as uint;
    outsize =
        (outsize as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<png_footer_t>()
                                             as libc::c_ulong) as uint as
            uint;
    // write PNG header
    memcpy(png_hdr.sign.as_mut_ptr() as *mut libc::c_void,
           png_sign.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong);
    // write IHDR chunk length
    png_hdr.ihdr_len = __bswap_32(ihdr_len);
    // write IHDR chunk signature
    memcpy(png_hdr.ihdr_sign.as_mut_ptr() as *mut libc::c_void,
           ihdr_sign.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong);
    // write image width
    png_hdr.ihdr_chunk.width = __bswap_32((*pix).width as __uint32_t);
    // write image height
    png_hdr.ihdr_chunk.height = __bswap_32((*pix).height as __uint32_t);
    // write image bitdepth
    png_hdr.ihdr_chunk.bitdepth = 8 as libc::c_int as uint8_t;
    // write image colortype
    png_hdr.ihdr_chunk.colortype =
        if (*pix).flags & IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint != 0
           {
            PNG_CT_RGBA as libc::c_int
        } else { PNG_CT_RGB as libc::c_int } as uint8_t; // 8 bits of alpha
    // write image comression method
    png_hdr.ihdr_chunk.compression = 0 as libc::c_int as uint8_t;
    // write image filter type
    png_hdr.ihdr_chunk.filter = 0 as libc::c_int as uint8_t;
    // write image interlacing
    png_hdr.ihdr_chunk.interlace = 0 as libc::c_int as uint8_t;
    // get IHDR chunk CRC
    CRC32_Init(&mut mz_crc32);
    CRC32_ProcessBuffer(&mut mz_crc32,
                        &mut png_hdr.ihdr_sign as *mut [uint8_t; 4] as
                            *const libc::c_void,
                        (ihdr_len as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 4]>()
                                                             as libc::c_ulong)
                            as libc::c_int);
    mz_crc32 = CRC32_Final(mz_crc32);
    // write IHDR chunk CRC
    png_hdr.ihdr_crc32 = __bswap_32(mz_crc32);
    buffer =
        _Mem_Alloc(host.imagepool, outsize as size_t, false_0,
                   b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                       *const libc::c_char, 537 as libc::c_int) as *mut byte;
    out = buffer;
    stream.next_in = filtered_buffer;
    stream.avail_in = filtered_size;
    stream.next_out =
        buffer.offset(::std::mem::size_of::<png_t>() as libc::c_ulong as
                          isize).offset(::std::mem::size_of::<uint>() as
                                            libc::c_ulong as
                                            isize).offset(::std::mem::size_of::<[libc::c_char; 4]>()
                                                              as libc::c_ulong
                                                              as isize);
    stream.avail_out = idat_len;
    // compress image
    if mz_deflateInit(&mut stream, MZ_BEST_COMPRESSION as libc::c_int) !=
           MZ_OK as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Image_SavePNG: deflateInit failed (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        _Mem_Free(filtered_buffer as *mut libc::c_void,
                  b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                      *const libc::c_char, 548 as libc::c_int);
        _Mem_Free(buffer as *mut libc::c_void,
                  b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                      *const libc::c_char, 549 as libc::c_int);
        return false_0
    }
    ret = mz_deflate(&mut stream, MZ_FINISH as libc::c_int);
    mz_deflateEnd(&mut stream);
    _Mem_Free(filtered_buffer as *mut libc::c_void,
              b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                  *const libc::c_char, 556 as libc::c_int);
    if ret != MZ_OK as libc::c_int && ret != MZ_STREAM_END as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Image_SavePNG: IDAT chunk compression failed (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        _Mem_Free(buffer as *mut libc::c_void,
                  b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                      *const libc::c_char, 561 as libc::c_int);
        return false_0
    }
    // get final filesize
    outsize =
        (outsize as libc::c_uint).wrapping_sub(idat_len) as uint as uint;
    idat_len = stream.total_out as uint;
    outsize =
        (outsize as libc::c_uint).wrapping_add(idat_len) as uint as uint;
    memcpy(out as *mut libc::c_void,
           &mut png_hdr as *mut png_t as *const libc::c_void,
           ::std::mem::size_of::<png_t>() as libc::c_ulong);
    out =
        out.offset(::std::mem::size_of::<png_t>() as libc::c_ulong as isize);
    // convert IDAT chunk length to big endian
    big_idat_len = __bswap_32(idat_len);
    // write IDAT chunk length
    memcpy(out as *mut libc::c_void,
           &mut big_idat_len as *mut uint as *const libc::c_void,
           ::std::mem::size_of::<uint>() as libc::c_ulong);
    out = out.offset(::std::mem::size_of::<uint>() as libc::c_ulong as isize);
    // write IDAT chunk signature
    memcpy(out as *mut libc::c_void,
           idat_sign.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong);
    // calculate IDAT chunk CRC
    CRC32_Init(&mut mz_crc32);
    CRC32_ProcessBuffer(&mut mz_crc32, out as *const libc::c_void,
                        (idat_len as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 4]>()
                                                             as libc::c_ulong)
                            as libc::c_int);
    mz_crc32 = CRC32_Final(mz_crc32);
    out =
        out.offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                       as isize);
    out = out.offset(idat_len as isize);
    // write IDAT chunk CRC
    png_ftr.idat_crc32 = __bswap_32(mz_crc32);
    // write IEND chunk length
    png_ftr.iend_len = 0 as libc::c_int as uint32_t;
    // write IEND chunk signature
    memcpy(png_ftr.iend_sign.as_mut_ptr() as *mut libc::c_void,
           iend_sign.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong);
    // write IEND chunk CRC
    png_ftr.iend_crc32 = __bswap_32(iend_crc32 as __uint32_t);
    // write PNG footer to buffer
    memcpy(out as *mut libc::c_void,
           &mut png_ftr as *mut png_footer_t as *const libc::c_void,
           ::std::mem::size_of::<png_footer_t>() as libc::c_ulong);
    FS_WriteFile(name, buffer as *const libc::c_void, outsize as fs_offset_t);
    _Mem_Free(buffer as *mut libc::c_void,
              b"../engine/common/imagelib/img_png.c\x00" as *const u8 as
                  *const libc::c_char, 610 as libc::c_int);
    return true_0;
}
