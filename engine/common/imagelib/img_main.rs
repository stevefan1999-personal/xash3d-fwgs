#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type decallist_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_sprintf(buffer: *mut libc::c_char, format: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn COM_FileExtension(in_0: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn COM_StripExtension(path: *mut libc::c_char);
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
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Image_Copy(size: size_t) -> *mut byte;
    #[no_mangle]
    fn Image_ResampleInternal(indata: *const libc::c_void, in_w: libc::c_int,
                              in_h: libc::c_int, out_w: libc::c_int,
                              out_h: libc::c_int, intype: libc::c_int,
                              done: *mut qboolean) -> *mut byte;
    #[no_mangle]
    fn Image_FlipInternal(in_0: *const byte, srcwidth: *mut word,
                          srcheight: *mut word, type_0: libc::c_int,
                          flags: libc::c_int) -> *mut byte;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint32_t = __uint32_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const PF_TOTALCOUNT: C2RustUnnamed = 12;
pub const PF_ATI2: C2RustUnnamed = 11;
pub const PF_DXT5: C2RustUnnamed = 10;
pub const PF_DXT3: C2RustUnnamed = 9;
pub const PF_DXT1: C2RustUnnamed = 8;
pub const PF_LUMINANCE: C2RustUnnamed = 7;
pub const PF_BGR_24: C2RustUnnamed = 6;
pub const PF_RGB_24: C2RustUnnamed = 5;
pub const PF_BGRA_32: C2RustUnnamed = 4;
pub const PF_RGBA_32: C2RustUnnamed = 3;
pub const PF_INDEXED_32: C2RustUnnamed = 2;
pub const PF_INDEXED_24: C2RustUnnamed = 1;
pub const PF_UNKNOWN: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
pub type bpc_desc_t = bpc_desc_s;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IMAGE_REMAP: C2RustUnnamed_0 = 134217728;
pub const IMAGE_LIGHTGAMMA: C2RustUnnamed_0 = 67108864;
pub const IMAGE_QUANTIZE: C2RustUnnamed_0 = 33554432;
pub const IMAGE_MAKE_LUMA: C2RustUnnamed_0 = 16777216;
pub const IMAGE_FORCE_RGBA: C2RustUnnamed_0 = 8388608;
pub const IMAGE_RESAMPLE: C2RustUnnamed_0 = 1048576;
pub const IMAGE_EMBOSS: C2RustUnnamed_0 = 524288;
pub const IMAGE_ROT270: C2RustUnnamed_0 = 458752;
pub const IMAGE_ROT180: C2RustUnnamed_0 = 196608;
pub const IMAGE_ROT_90: C2RustUnnamed_0 = 262144;
pub const IMAGE_FLIP_Y: C2RustUnnamed_0 = 131072;
pub const IMAGE_FLIP_X: C2RustUnnamed_0 = 65536;
pub const IMAGE_QUAKEPAL: C2RustUnnamed_0 = 1024;
pub const IMAGE_ONEBIT_ALPHA: C2RustUnnamed_0 = 512;
pub const IMAGE_MULTILAYER: C2RustUnnamed_0 = 256;
pub const IMAGE_DDS_FORMAT: C2RustUnnamed_0 = 128;
pub const IMAGE_QUAKESKY: C2RustUnnamed_0 = 64;
pub const IMAGE_SKYBOX: C2RustUnnamed_0 = 32;
pub const IMAGE_HAS_LUMA: C2RustUnnamed_0 = 16;
pub const IMAGE_COLORINDEX: C2RustUnnamed_0 = 8;
pub const IMAGE_HAS_COLOR: C2RustUnnamed_0 = 4;
pub const IMAGE_HAS_ALPHA: C2RustUnnamed_0 = 2;
pub const IMAGE_CUBEMAP: C2RustUnnamed_0 = 1;
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
pub type imglib_t = imglib_s;
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
pub type image_hint_t = libc::c_uint;
pub const IL_HINT_HL: image_hint_t = 2;
pub const IL_HINT_Q1: image_hint_t = 1;
pub const IL_HINT_NO: image_hint_t = 0;
pub type savepixformat_t = saveformat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saveformat_s {
    pub formatstring: *const libc::c_char,
    pub ext: *const libc::c_char,
    pub savefunc: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *mut rgbdata_t) -> qboolean>,
}
pub type loadpixformat_t = loadformat_s;
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
pub type suffix_t = suffix_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_s {
    pub suf: *const libc::c_char,
    pub flags: uint,
    pub hint: side_hint_t,
}
pub type side_hint_t = libc::c_uint;
pub const CB_FACECOUNT: side_hint_t = 7;
pub const CB_HINT_NEGY: side_hint_t = 6;
pub const CB_HINT_POSY: side_hint_t = 5;
pub const CB_HINT_NEGZ: side_hint_t = 4;
pub const CB_HINT_POSZ: side_hint_t = 3;
pub const CB_HINT_NEGX: side_hint_t = 2;
pub const CB_HINT_POSX: side_hint_t = 1;
pub const CB_HINT_NO: side_hint_t = 0;
pub type cubepack_t = cubepack_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cubepack_s {
    pub name: *const libc::c_char,
    pub type_0: *const suffix_t,
}
// just for debug
/*
img_main.c - load & save various image formats
Copyright (C) 2007 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// global image variables
#[no_mangle]
pub static mut image: imglib_t =
    imglib_t{loadformats: 0 as *const loadpixformat_t,
             saveformats: 0 as *const savepixformat_t,
             width: 0,
             height: 0,
             depth: 0,
             num_mips: 0,
             encode: 0,
             type_0: 0,
             flags: 0,
             size: 0,
             ptr: 0,
             bpp: 0,
             rgba: 0 as *const byte as *mut byte,
             source_width: 0,
             source_height: 0,
             source_type: 0,
             num_sides: 0,
             cubemap: 0 as *const byte as *mut byte,
             d_currentpal: 0 as *const uint as *mut uint,
             d_rendermode: 0,
             palette: 0 as *const byte as *mut byte,
             fogParams: [0; 4],
             hint: IL_HINT_NO,
             tempbuffer: 0 as *const byte as *mut byte,
             cmd_flags: 0,
             force_flags: 0,
             custom_palette: false_0,};
static mut skybox_qv1: [suffix_t; 6] =
    [{
         let mut init =
             suffix_s{suf: b"ft\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_FLIP_X as libc::c_int as uint,
                      hint: CB_HINT_POSX,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"bk\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_FLIP_Y as libc::c_int as uint,
                      hint: CB_HINT_NEGX,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"up\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_ROT_90 as libc::c_int as uint,
                      hint: CB_HINT_POSZ,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"dn\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_ROT_90 as libc::c_int as uint,
                      hint: CB_HINT_NEGZ,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"rt\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_ROT_90 as libc::c_int as uint,
                      hint: CB_HINT_POSY,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"lf\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_ROT270 as libc::c_int as uint,
                      hint: CB_HINT_NEGY,};
         init
     }];
static mut skybox_qv2: [suffix_t; 6] =
    [{
         let mut init =
             suffix_s{suf: b"_ft\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_FLIP_X as libc::c_int as uint,
                      hint: CB_HINT_POSX,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"_bk\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_FLIP_Y as libc::c_int as uint,
                      hint: CB_HINT_NEGX,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"_up\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_ROT_90 as libc::c_int as uint,
                      hint: CB_HINT_POSZ,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"_dn\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_ROT_90 as libc::c_int as uint,
                      hint: CB_HINT_NEGZ,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"_rt\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_ROT_90 as libc::c_int as uint,
                      hint: CB_HINT_POSY,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"_lf\x00" as *const u8 as *const libc::c_char,
                      flags: IMAGE_ROT270 as libc::c_int as uint,
                      hint: CB_HINT_NEGY,};
         init
     }];
static mut cubemap_v1: [suffix_t; 6] =
    [{
         let mut init =
             suffix_s{suf: b"px\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as uint,
                      hint: CB_HINT_POSX,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"nx\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as uint,
                      hint: CB_HINT_NEGX,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"py\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as uint,
                      hint: CB_HINT_POSY,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"ny\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as uint,
                      hint: CB_HINT_NEGY,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"pz\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as uint,
                      hint: CB_HINT_POSZ,};
         init
     },
     {
         let mut init =
             suffix_s{suf: b"nz\x00" as *const u8 as *const libc::c_char,
                      flags: 0 as libc::c_int as uint,
                      hint: CB_HINT_NEGZ,};
         init
     }];
static mut load_cubemap: [cubepack_t; 4] =
    unsafe {
        [{
             let mut init =
                 cubepack_s{name:
                                b"3Ds Sky1\x00" as *const u8 as
                                    *const libc::c_char,
                            type_0: skybox_qv1.as_ptr(),};
             init
         },
         {
             let mut init =
                 cubepack_s{name:
                                b"3Ds Sky2\x00" as *const u8 as
                                    *const libc::c_char,
                            type_0: skybox_qv2.as_ptr(),};
             init
         },
         {
             let mut init =
                 cubepack_s{name:
                                b"3Ds Cube\x00" as *const u8 as
                                    *const libc::c_char,
                            type_0: cubemap_v1.as_ptr(),};
             init
         },
         {
             let mut init =
                 cubepack_s{name: 0 as *const libc::c_char,
                            type_0: 0 as *const suffix_t,};
             init
         }]
    };
// soul of ImageLib - table of image format constants
#[no_mangle]
pub static mut PFDesc: [bpc_desc_t; 12] =
    unsafe {
        [{
             let mut init =
                 bpc_desc_s{format: PF_UNKNOWN as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"raw\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x1908 as libc::c_int as uint,
                            bpp: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_INDEXED_24 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"pal 24\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x1908 as libc::c_int as uint,
                            bpp: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_INDEXED_32 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"pal 32\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x1908 as libc::c_int as uint,
                            bpp: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_RGBA_32 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"RGBA 32\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x1908 as libc::c_int as uint,
                            bpp: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_BGRA_32 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"BGRA 32\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x80e1 as libc::c_int as uint,
                            bpp: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_RGB_24 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"RGB 24\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x1908 as libc::c_int as uint,
                            bpp: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_BGR_24 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"BGR 24\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x80e0 as libc::c_int as uint,
                            bpp: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_LUMINANCE as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"LUM 8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x1909 as libc::c_int as uint,
                            bpp: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_DXT1 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"DXT 1\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x83f1 as libc::c_int as uint,
                            bpp: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_DXT3 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"DXT 3\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x83f2 as libc::c_int as uint,
                            bpp: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_DXT5 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"DXT 5\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x83f3 as libc::c_int as uint,
                            bpp: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 bpc_desc_s{format: PF_ATI2 as libc::c_int,
                            name:
                                *::std::mem::transmute::<&[u8; 16],
                                                         &mut [libc::c_char; 16]>(b"ATI 2\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            glFormat: 0x8837 as libc::c_int as uint,
                            bpp: 4 as libc::c_int,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn Image_Reset() {
    // reset global variables
    image.depth = 0 as libc::c_int as word;
    image.height = image.depth;
    image.width = image.height;
    image.source_height = 0 as libc::c_int;
    image.source_width = image.source_height;
    image.num_mips = 0 as libc::c_int as byte;
    image.source_type = image.num_mips as uint;
    image.flags = 0 as libc::c_int as uint;
    image.num_sides = image.flags as libc::c_int;
    image.encode = 0 as libc::c_int as word;
    image.type_0 = PF_UNKNOWN as libc::c_int as uint;
    image.fogParams[0 as libc::c_int as usize] = 0 as libc::c_int as byte;
    image.fogParams[1 as libc::c_int as usize] = 0 as libc::c_int as byte;
    image.fogParams[2 as libc::c_int as usize] = 0 as libc::c_int as byte;
    image.fogParams[3 as libc::c_int as usize] = 0 as libc::c_int as byte;
    // pointers will be saved with prevoius picture struct
	// don't care about it
    image.palette = 0 as *mut byte;
    image.cubemap = 0 as *mut byte;
    image.rgba = 0 as *mut byte;
    image.ptr = 0 as libc::c_int as uint;
    image.size = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ImagePack() -> *mut rgbdata_t {
    let mut pack: *mut rgbdata_t =
        _Mem_Alloc(host.imagepool,
                   ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/imagelib/img_main.c\x00" as *const u8 as
                       *const libc::c_char, 115 as libc::c_int) as
            *mut rgbdata_t;
    // clear any force flags
    image.force_flags = 0 as libc::c_int;
    if !image.cubemap.is_null() && image.num_sides != 6 as libc::c_int {
        // this never be happens, just in case
        FS_FreeImage(pack);
        return 0 as *mut rgbdata_t
    }
    if !image.cubemap.is_null() {
        image.flags |= IMAGE_CUBEMAP as libc::c_int as libc::c_uint;
        (*pack).buffer = image.cubemap;
        (*pack).width = image.source_width as word;
        (*pack).height = image.source_height as word;
        (*pack).type_0 = image.source_type;
        (*pack).size =
            image.size.wrapping_mul(image.num_sides as libc::c_ulong)
    } else {
        (*pack).buffer = image.rgba;
        (*pack).width = image.width;
        (*pack).height = image.height;
        (*pack).depth = image.depth;
        (*pack).type_0 = image.type_0;
        (*pack).size = image.size
    }
    // copy fog params
    (*pack).fogParams[0 as libc::c_int as usize] =
        image.fogParams[0 as libc::c_int as usize];
    (*pack).fogParams[1 as libc::c_int as usize] =
        image.fogParams[1 as libc::c_int as usize];
    (*pack).fogParams[2 as libc::c_int as usize] =
        image.fogParams[2 as libc::c_int as usize];
    (*pack).fogParams[3 as libc::c_int as usize] =
        image.fogParams[3 as libc::c_int as usize];
    (*pack).flags = image.flags;
    (*pack).numMips = image.num_mips;
    (*pack).palette = image.palette;
    (*pack).encode = image.encode;
    return pack;
}
/*
================
FS_AddSideToPack

================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_AddSideToPack(mut name: *const libc::c_char,
                                          mut adjust_flags: libc::c_int)
 -> qboolean {
    let mut out: *mut byte = 0 as *mut byte;
    let mut flipped: *mut byte = 0 as *mut byte;
    let mut resampled: qboolean = false_0;
    // first side set average size for all cubemap sides!
    if image.cubemap.is_null() {
        image.source_width = image.width as libc::c_int;
        image.source_height = image.height as libc::c_int;
        image.source_type = image.type_0
    }
    // keep constant size, render.dll expecting it
    image.size =
        (image.source_width * image.source_height * 4 as libc::c_int) as
            size_t;
    // mixing dds format with any existing ?
    if image.type_0 != image.source_type { return false_0 }
    // flip image if needed
    flipped =
        Image_FlipInternal(image.rgba, &mut image.width, &mut image.height,
                           image.source_type as libc::c_int,
                           adjust_flags); // try to reasmple dxt?
    if flipped.is_null() { return false_0 }
    if flipped != image.rgba { image.rgba = Image_Copy(image.size) }
    // resampling image if needed
    out =
        Image_ResampleInternal(image.rgba as *mut uint as *const libc::c_void,
                               image.width as libc::c_int,
                               image.height as libc::c_int,
                               image.source_width, image.source_height,
                               image.source_type as libc::c_int,
                               &mut resampled); // try to reasmple dxt?
    if out.is_null() { return false_0 } // add new side
    if resampled as u64 != 0 {
        image.rgba = Image_Copy(image.size)
    } // release source buffer
    image.cubemap =
        _Mem_Realloc(host.imagepool, image.cubemap as *mut libc::c_void,
                     (image.ptr as libc::c_ulong).wrapping_add(image.size),
                     true_0,
                     b"../engine/common/imagelib/img_main.c\x00" as *const u8
                         as *const libc::c_char, 196 as libc::c_int) as
            *mut byte; // move to next
    memcpy(image.cubemap.offset(image.ptr as isize) as *mut libc::c_void,
           image.rgba as *const libc::c_void, image.size); // bump sides count
    _Mem_Free(image.rgba as *mut libc::c_void,
              b"../engine/common/imagelib/img_main.c\x00" as *const u8 as
                  *const libc::c_char, 199 as libc::c_int);
    image.ptr =
        (image.ptr as libc::c_ulong).wrapping_add(image.size) as uint as uint;
    image.num_sides += 1;
    return true_0;
}
/*
================
FS_LoadImage

loading and unpack to rgba any known image
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_LoadImage(mut filename: *const libc::c_char,
                                      mut buffer: *const byte,
                                      mut size: size_t) -> *mut rgbdata_t {
    let mut ext: *const libc::c_char =
        COM_FileExtension(filename); // clear old image
    let mut path: string = [0; 256];
    let mut loadname: string = [0; 256];
    let mut sidename: string = [0; 256];
    let mut anyformat: qboolean = true_0;
    let mut i: libc::c_int = 0;
    let mut filesize: fs_offset_t = 0 as libc::c_int as fs_offset_t;
    let mut format: *const loadpixformat_t = 0 as *const loadpixformat_t;
    let mut cmap: *const cubepack_t = 0 as *const cubepack_t;
    let mut f: *mut byte = 0 as *mut byte;
    Q_strncpy(loadname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    Image_Reset();
    if Q_strnicmp(ext, b"\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 {
        // we needs to compare file extension with list of supported formats
		// and be sure what is real extension, not a filename with dot
        format = image.loadformats;
        while !format.is_null() && !(*format).formatstring.is_null() {
            if Q_strnicmp((*format).ext, ext, 99999 as libc::c_int) == 0 {
                COM_StripExtension(loadname.as_mut_ptr());
                anyformat = false_0;
                break ;
            } else { format = format.offset(1) }
        }
    }
    // special mode: skip any checks, load file from buffer
    if !(*filename.offset(0 as libc::c_int as isize) as libc::c_int ==
             '#' as i32 && !buffer.is_null() && size != 0) {
        // now try all the formats in the selected list
        format = image.loadformats; // release buffer
        while !format.is_null() && !(*format).formatstring.is_null() {
            if anyformat as libc::c_uint != 0 ||
                   Q_strnicmp(ext, (*format).ext, 99999 as libc::c_int) == 0 {
                Q_sprintf(path.as_mut_ptr(), (*format).formatstring,
                          loadname.as_mut_ptr(),
                          b"\x00" as *const u8 as *const libc::c_char,
                          (*format).ext);
                image.hint = (*format).hint;
                f = FS_LoadFile(path.as_mut_ptr(), &mut filesize, false_0);
                if !f.is_null() && filesize > 0 as libc::c_int as libc::c_long
                   {
                    if (*format).loadfunc.expect("non-null function pointer")(path.as_mut_ptr(),
                                                                              f,
                                                                              filesize)
                           as u64 != 0 {
                        _Mem_Free(f as *mut libc::c_void,
                                  b"../engine/common/imagelib/img_main.c\x00"
                                      as *const u8 as *const libc::c_char,
                                  259 as libc::c_int);
                        return ImagePack()
                        // loaded
                    } else {
                        _Mem_Free(f as *mut libc::c_void,
                                  b"../engine/common/imagelib/img_main.c\x00"
                                      as *const u8 as *const libc::c_char,
                                  262 as libc::c_int);
                    }
                    // release buffer
                }
            }
            format = format.offset(1)
        }
        // check all cubemap sides with package suffix
        cmap = load_cubemap.as_ptr();
        while !cmap.is_null() && !(*cmap).type_0.is_null() {
            i = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                // for support mixed cubemaps e.g. sky_ft.bmp, sky_rt.tga
			// NOTE: all loaders must keep sides in one format for all
                format = image.loadformats; // side hint
                while !format.is_null() && !(*format).formatstring.is_null() {
                    if anyformat as libc::c_uint != 0 ||
                           Q_strnicmp(ext, (*format).ext,
                                      99999 as libc::c_int) == 0 {
                        Q_sprintf(path.as_mut_ptr(), (*format).formatstring,
                                  loadname.as_mut_ptr(),
                                  (*(*cmap).type_0.offset(i as isize)).suf,
                                  (*format).ext);
                        image.hint =
                            (*(*cmap).type_0.offset(i as isize)).hint as
                                image_hint_t;
                        f =
                            FS_LoadFile(path.as_mut_ptr(), &mut filesize,
                                        false_0);
                        if !f.is_null() &&
                               filesize > 0 as libc::c_int as libc::c_long {
                            // this name will be used only for tell user about problems
                            if (*format).loadfunc.expect("non-null function pointer")(path.as_mut_ptr(),
                                                                                      f,
                                                                                      filesize)
                                   as u64 != 0 {
                                Q_snprintf(sidename.as_mut_ptr(),
                                           ::std::mem::size_of::<string>() as
                                               libc::c_ulong,
                                           b"%s%s.%s\x00" as *const u8 as
                                               *const libc::c_char,
                                           loadname.as_mut_ptr(),
                                           (*(*cmap).type_0.offset(i as
                                                                       isize)).suf,
                                           (*format).ext);
                                if FS_AddSideToPack(sidename.as_mut_ptr(),
                                                    (*(*cmap).type_0.offset(i
                                                                                as
                                                                                isize)).flags
                                                        as libc::c_int) as u64
                                       != 0 {
                                    // process flags to flip some sides
                                    _Mem_Free(f as *mut libc::c_void,
                                              b"../engine/common/imagelib/img_main.c\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              290 as libc::c_int);
                                    break ;
                                    // loaded
                                }
                            }
                            _Mem_Free(f as *mut libc::c_void,
                                      b"../engine/common/imagelib/img_main.c\x00"
                                          as *const u8 as *const libc::c_char,
                                      294 as libc::c_int);
                        }
                    }
                    format = format.offset(1)
                }
                if image.num_sides != i + 1 as libc::c_int {
                    // check side
                    // first side not found, probably it's not cubemap
				// it contain info about image_type and dimensions, don't generate black cubemaps
                    if image.cubemap.is_null() { break ; }
                    // Mem_Alloc already filled memblock with 0x00, no need to do it again
                    image.cubemap =
                        _Mem_Realloc(host.imagepool,
                                     image.cubemap as *mut libc::c_void,
                                     (image.ptr as
                                          libc::c_ulong).wrapping_add(image.size),
                                     true_0,
                                     b"../engine/common/imagelib/img_main.c\x00"
                                         as *const u8 as *const libc::c_char,
                                     305 as libc::c_int) as
                            *mut byte; // move to next
                    image.ptr =
                        (image.ptr as libc::c_ulong).wrapping_add(image.size)
                            as uint as uint;
                    image.num_sides += 1
                    // merge counter
                }
                i += 1
            }
            // make sure what all sides is loaded
            if !(image.num_sides != 6 as libc::c_int) { break ; }
            // unexpected errors ?
            if !image.cubemap.is_null() {
                _Mem_Free(image.cubemap as *mut libc::c_void,
                          b"../engine/common/imagelib/img_main.c\x00" as
                              *const u8 as *const libc::c_char,
                          316 as libc::c_int); // all done
            }
            Image_Reset();
            cmap = cmap.offset(1)
        }
        if !image.cubemap.is_null() { return ImagePack() }
    }
    format = image.loadformats;
    while !format.is_null() && !(*format).formatstring.is_null() {
        if anyformat as libc::c_uint != 0 ||
               Q_strnicmp(ext, (*format).ext, 99999 as libc::c_int) == 0 {
            image.hint = (*format).hint;
            if !buffer.is_null() && size > 0 as libc::c_int as libc::c_ulong {
                if (*format).loadfunc.expect("non-null function pointer")(loadname.as_mut_ptr(),
                                                                          buffer,
                                                                          size
                                                                              as
                                                                              fs_offset_t)
                       as u64 != 0 {
                    return ImagePack()
                }
                // loaded
            }
        }
        format = format.offset(1)
    }
    if *filename.offset(0 as libc::c_int as isize) as libc::c_int !=
           '#' as i32 {
        Con_Reportf(b"^3Warning:^7 FS_LoadImage: couldn\'t load \"%s\"\n\x00"
                        as *const u8 as *const libc::c_char,
                    loadname.as_mut_ptr());
    }
    // clear any force flags
    image.force_flags = 0 as libc::c_int;
    return 0 as *mut rgbdata_t;
}
/*
================
Image_Save

writes image as any known format
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_SaveImage(mut filename: *const libc::c_char,
                                      mut pix: *mut rgbdata_t) -> qboolean {
    let mut ext: *const libc::c_char = COM_FileExtension(filename);
    let mut anyformat: qboolean =
        if Q_strnicmp(ext, b"\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    let mut path: string = [0; 256];
    let mut savename: string = [0; 256];
    let mut format: *const savepixformat_t = 0 as *const savepixformat_t;
    if pix.is_null() || (*pix).buffer.is_null() ||
           anyformat as libc::c_uint != 0 {
        // clear any force flags
        image.force_flags = 0 as libc::c_int; // remove extension if needed
        return false_0
    } // keep real pic size
    Q_strncpy(savename.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as
                  libc::c_ulong); // to avoid corrupt memory on free data
    COM_StripExtension(savename.as_mut_ptr());
    if (*pix).flags &
           (IMAGE_CUBEMAP as libc::c_int | IMAGE_SKYBOX as libc::c_int) as
               libc::c_uint != 0 {
        let mut realSize: size_t = (*pix).size;
        let mut picBuffer: *mut byte = 0 as *mut byte;
        let mut box_0: *const suffix_t = 0 as *const suffix_t;
        let mut i: libc::c_int = 0;
        if (*pix).flags & IMAGE_SKYBOX as libc::c_int as libc::c_uint != 0 {
            box_0 = skybox_qv1.as_ptr()
        } else if (*pix).flags & IMAGE_CUBEMAP as libc::c_int as libc::c_uint
                      != 0 {
            box_0 = cubemap_v1.as_ptr()
        } else {
            // clear any force flags
            image.force_flags = 0 as libc::c_int;
            return false_0
            // do not happens
        } // now set as side size
        (*pix).size =
            ((*pix).size as
                 libc::c_ulong).wrapping_div(6 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        picBuffer = (*pix).buffer;
        // save all sides seperately
        format = image.saveformats;
        while !format.is_null() && !(*format).formatstring.is_null() {
            if Q_strnicmp(ext, (*format).ext, 99999 as libc::c_int) == 0 {
                i = 0 as libc::c_int;
                while i < 6 as libc::c_int {
                    Q_sprintf(path.as_mut_ptr(), (*format).formatstring,
                              savename.as_mut_ptr(),
                              (*box_0.offset(i as isize)).suf, (*format).ext);
                    // move pointer
                    if (*format).savefunc.expect("non-null function pointer")(path.as_mut_ptr(),
                                                                              pix)
                           as u64 == 0 {
                        break ; // there were errors
                    }
                    (*pix).buffer =
                        (*pix).buffer.offset((*pix).size as isize);
                    i += 1
                }
                // restore pointers
                (*pix).size = realSize;
                (*pix).buffer = picBuffer;
                // clear any force flags
                image.force_flags = 0 as libc::c_int;
                return (i == 6 as libc::c_int) as libc::c_int as qboolean
            }
            format = format.offset(1)
        }
    } else {
        format = image.saveformats;
        while !format.is_null() && !(*format).formatstring.is_null() {
            if Q_strnicmp(ext, (*format).ext, 99999 as libc::c_int) == 0 {
                Q_sprintf(path.as_mut_ptr(), (*format).formatstring,
                          savename.as_mut_ptr(),
                          b"\x00" as *const u8 as *const libc::c_char,
                          (*format).ext);
                if (*format).savefunc.expect("non-null function pointer")(path.as_mut_ptr(),
                                                                          pix)
                       as u64 != 0 {
                    // clear any force flags
                    image.force_flags = 0 as libc::c_int;
                    return true_0
                    // saved
                }
            }
            format = format.offset(1)
        }
    }
    // clear any force flags
    image.force_flags = 0 as libc::c_int;
    return false_0;
}
/*
================
Image_FreeImage

free RGBA buffer
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FreeImage(mut pack: *mut rgbdata_t) {
    if pack.is_null() { return }
    if !(*pack).buffer.is_null() {
        _Mem_Free((*pack).buffer as *mut libc::c_void,
                  b"../engine/common/imagelib/img_main.c\x00" as *const u8 as
                      *const libc::c_char, 449 as libc::c_int);
    }
    if !(*pack).palette.is_null() {
        _Mem_Free((*pack).palette as *mut libc::c_void,
                  b"../engine/common/imagelib/img_main.c\x00" as *const u8 as
                      *const libc::c_char, 450 as libc::c_int);
    }
    _Mem_Free(pack as *mut libc::c_void,
              b"../engine/common/imagelib/img_main.c\x00" as *const u8 as
                  *const libc::c_char, 451 as libc::c_int);
}
/*
================
FS_CopyImage

make an image copy
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_CopyImage(mut in_0: *mut rgbdata_t)
 -> *mut rgbdata_t {
    let mut out: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut palSize: libc::c_int = 0 as libc::c_int;
    if in_0.is_null() { return 0 as *mut rgbdata_t }
    out =
        _Mem_Alloc(host.imagepool,
                   ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong,
                   false_0,
                   b"../engine/common/imagelib/img_main.c\x00" as *const u8 as
                       *const libc::c_char, 468 as libc::c_int) as
            *mut rgbdata_t;
    *out = *in_0;
    match (*in_0).type_0 {
        1 => { palSize = 768 as libc::c_int }
        2 => { palSize = 1024 as libc::c_int }
        _ => { }
    }
    if palSize != 0 {
        (*out).palette =
            _Mem_Alloc(host.imagepool, palSize as size_t, false_0,
                       b"../engine/common/imagelib/img_main.c\x00" as
                           *const u8 as *const libc::c_char,
                       483 as libc::c_int) as *mut byte;
        memcpy((*out).palette as *mut libc::c_void,
               (*in_0).palette as *const libc::c_void,
               palSize as libc::c_ulong);
    }
    if (*in_0).size != 0 {
        (*out).buffer =
            _Mem_Alloc(host.imagepool, (*in_0).size, false_0,
                       b"../engine/common/imagelib/img_main.c\x00" as
                           *const u8 as *const libc::c_char,
                       489 as libc::c_int) as *mut byte;
        memcpy((*out).buffer as *mut libc::c_void,
               (*in_0).buffer as *const libc::c_void, (*in_0).size);
    }
    return out;
}
/* XASH_ENGINE_TESTS */
