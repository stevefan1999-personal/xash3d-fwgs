#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type decallist_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Image_ValidSize(name: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Image_CheckFlag(bit: libc::c_int) -> qboolean;
    #[no_mangle]
    static mut image: imglib_t;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IL_OVERVIEW: C2RustUnnamed_0 = 64;
pub const IL_LOAD_DECAL: C2RustUnnamed_0 = 32;
pub const IL_DDS_HARDWARE: C2RustUnnamed_0 = 16;
pub const IL_DONTFLIP_TGA: C2RustUnnamed_0 = 8;
pub const IL_ALLOW_OVERWRITE: C2RustUnnamed_0 = 4;
pub const IL_KEEP_8BIT: C2RustUnnamed_0 = 2;
pub const IL_USE_LERPING: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IMAGE_REMAP: C2RustUnnamed_1 = 134217728;
pub const IMAGE_LIGHTGAMMA: C2RustUnnamed_1 = 67108864;
pub const IMAGE_QUANTIZE: C2RustUnnamed_1 = 33554432;
pub const IMAGE_MAKE_LUMA: C2RustUnnamed_1 = 16777216;
pub const IMAGE_FORCE_RGBA: C2RustUnnamed_1 = 8388608;
pub const IMAGE_RESAMPLE: C2RustUnnamed_1 = 1048576;
pub const IMAGE_EMBOSS: C2RustUnnamed_1 = 524288;
pub const IMAGE_ROT270: C2RustUnnamed_1 = 458752;
pub const IMAGE_ROT180: C2RustUnnamed_1 = 196608;
pub const IMAGE_ROT_90: C2RustUnnamed_1 = 262144;
pub const IMAGE_FLIP_Y: C2RustUnnamed_1 = 131072;
pub const IMAGE_FLIP_X: C2RustUnnamed_1 = 65536;
pub const IMAGE_QUAKEPAL: C2RustUnnamed_1 = 1024;
pub const IMAGE_ONEBIT_ALPHA: C2RustUnnamed_1 = 512;
pub const IMAGE_MULTILAYER: C2RustUnnamed_1 = 256;
pub const IMAGE_DDS_FORMAT: C2RustUnnamed_1 = 128;
pub const IMAGE_QUAKESKY: C2RustUnnamed_1 = 64;
pub const IMAGE_SKYBOX: C2RustUnnamed_1 = 32;
pub const IMAGE_HAS_LUMA: C2RustUnnamed_1 = 16;
pub const IMAGE_COLORINDEX: C2RustUnnamed_1 = 8;
pub const IMAGE_HAS_COLOR: C2RustUnnamed_1 = 4;
pub const IMAGE_HAS_ALPHA: C2RustUnnamed_1 = 2;
pub const IMAGE_CUBEMAP: C2RustUnnamed_1 = 1;
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
pub type dds_t = dds_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dds_s {
    pub dwIdent: uint32_t,
    pub dwSize: uint32_t,
    pub dwFlags: uint32_t,
    pub dwHeight: uint32_t,
    pub dwWidth: uint32_t,
    pub dwLinearSize: uint32_t,
    pub dwDepth: uint32_t,
    pub dwMipMapCount: uint32_t,
    pub dwAlphaBitDepth: uint32_t,
    pub dwReserved1: [uint32_t; 10],
    pub dsPixelFormat: dds_pixf_t,
    pub dsCaps: dds_caps_t,
    pub dwTextureStage: uint32_t,
}
pub type dds_caps_t = dds_caps_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dds_caps_s {
    pub dwCaps1: uint32_t,
    pub dwCaps2: uint32_t,
    pub dwCaps3: uint32_t,
    pub dwCaps4: uint32_t,
}
pub type dds_pixf_t = dds_pf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dds_pf_s {
    pub dwSize: uint32_t,
    pub dwFlags: uint32_t,
    pub dwFourCC: uint32_t,
    pub dwRGBBitCount: uint32_t,
    pub dwRBitMask: uint32_t,
    pub dwGBitMask: uint32_t,
    pub dwBBitMask: uint32_t,
    pub dwABitMask: uint32_t,
}
/*
img_dds.c - dds format load
Copyright (C) 2015 Uncle Mike

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
pub unsafe extern "C" fn Image_CheckDXT3Alpha(mut hdr: *mut dds_t,
                                              mut fin: *mut byte)
 -> qboolean {
    let mut sAlpha: word = 0;
    let mut alpha: *mut byte = 0 as *mut byte;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    y = 0 as libc::c_int;
    while (y as libc::c_uint) < (*hdr).dwHeight {
        x = 0 as libc::c_int;
        while (x as libc::c_uint) < (*hdr).dwWidth {
            alpha = fin.offset(8 as libc::c_int as isize);
            fin = fin.offset(16 as libc::c_int as isize);
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                sAlpha =
                    (*alpha.offset((2 as libc::c_int * j) as isize) as
                         libc::c_int +
                         256 as libc::c_int *
                             *alpha.offset((2 as libc::c_int * j +
                                                1 as libc::c_int) as isize) as
                                 libc::c_int) as word;
                i = 0 as libc::c_int;
                while i < 4 as libc::c_int {
                    if ((x + i) as libc::c_uint) < (*hdr).dwWidth &&
                           ((y + j) as libc::c_uint) < (*hdr).dwHeight {
                        if sAlpha as libc::c_int == 0 as libc::c_int {
                            return true_0
                        }
                    }
                    sAlpha =
                        (sAlpha as libc::c_int >> 4 as libc::c_int) as word;
                    i += 1
                }
                j += 1
            }
            x += 4 as libc::c_int
        }
        y += 4 as libc::c_int
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Image_CheckDXT5Alpha(mut hdr: *mut dds_t,
                                              mut fin: *mut byte)
 -> qboolean {
    let mut bits: uint = 0;
    let mut bitmask: uint = 0;
    let mut alphamask: *mut byte = 0 as *mut byte;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    y = 0 as libc::c_int;
    while (y as libc::c_uint) < (*hdr).dwHeight {
        x = 0 as libc::c_int;
        while (x as libc::c_uint) < (*hdr).dwWidth {
            if y as libc::c_uint >= (*hdr).dwHeight ||
                   x as libc::c_uint >= (*hdr).dwWidth {
                break ;
            }
            alphamask = fin.offset(2 as libc::c_int as isize);
            fin = fin.offset(8 as libc::c_int as isize);
            bitmask = *(fin as *mut uint).offset(1 as libc::c_int as isize);
            fin = fin.offset(8 as libc::c_int as isize);
            // last three bytes
            bits =
                (*alphamask.offset(3 as libc::c_int as isize) as libc::c_int |
                     (*alphamask.offset(4 as libc::c_int as isize) as
                          libc::c_int) << 8 as libc::c_int |
                     (*alphamask.offset(5 as libc::c_int as isize) as
                          libc::c_int) << 16 as libc::c_int) as uint;
            j = 2 as libc::c_int;
            while j < 4 as libc::c_int {
                i = 0 as libc::c_int;
                while i < 4 as libc::c_int {
                    // only put pixels out < width or height
                    if ((x + i) as libc::c_uint) < (*hdr).dwWidth &&
                           ((y + j) as libc::c_uint) < (*hdr).dwHeight {
                        if bits & 0x7 as libc::c_int as libc::c_uint != 0 {
                            return true_0
                        }
                    } // alpha is already premultiplied by color
                    bits >>=
                        3 as
                            libc::c_int; // alpha is already premultiplied by color
                    i += 1
                } // assume error
                j += 1
            }
            x += 4 as libc::c_int
        }
        y += 4 as libc::c_int
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Image_DXTGetPixelFormat(mut hdr: *mut dds_t) {
    let mut bits: uint = (*hdr).dsPixelFormat.dwRGBBitCount;
    if (*hdr).dsCaps.dwCaps2 as libc::c_long & 0x200000 as libc::c_long == 0 {
        (*hdr).dwDepth = 1 as libc::c_int as uint32_t
    }
    if (*hdr).dsPixelFormat.dwFlags as libc::c_long & 0x4 as libc::c_long != 0
       {
        let mut current_block_8: u64;
        match (*hdr).dsPixelFormat.dwFourCC {
            827611204 => {
                image.type_0 = PF_DXT1 as libc::c_int as uint;
                current_block_8 = 8236137900636309791;
            }
            844388420 => {
                image.flags &=
                    !(IMAGE_HAS_ALPHA as libc::c_int) as libc::c_uint;
                current_block_8 = 6760737790439395503;
            }
            861165636 => { current_block_8 = 6760737790439395503; }
            877942852 => {
                image.flags &=
                    !(IMAGE_HAS_ALPHA as libc::c_int) as libc::c_uint;
                current_block_8 = 15652205675372273526;
            }
            894720068 => { current_block_8 = 15652205675372273526; }
            843666497 => {
                image.type_0 = PF_ATI2 as libc::c_int as uint;
                current_block_8 = 8236137900636309791;
            }
            _ => {
                image.type_0 = PF_UNKNOWN as libc::c_int as uint;
                current_block_8 = 8236137900636309791;
            }
        }
        match current_block_8 {
            6760737790439395503 =>
            // intentionally fallthrough
            {
                image.type_0 = PF_DXT3 as libc::c_int as uint
            }
            15652205675372273526 =>
            // intentionally fallthrough
            {
                image.type_0 = PF_DXT5 as libc::c_int as uint
            }
            _ => { }
        }
    } else if (*hdr).dsPixelFormat.dwFlags as libc::c_long &
                  0x80000 as libc::c_long != 0 {
        image.type_0 = PF_UNKNOWN as libc::c_int as uint
        // this dds texture isn't compressed so write out ARGB or luminance format
        // assume error
    } else if (*hdr).dsPixelFormat.dwFlags as libc::c_long &
                  0x20000 as libc::c_long != 0 {
        image.type_0 = PF_UNKNOWN as libc::c_int as uint
        // assume error
    } else {
        match bits {
            32 => { image.type_0 = PF_BGRA_32 as libc::c_int as uint }
            24 => { image.type_0 = PF_BGR_24 as libc::c_int as uint }
            8 => { image.type_0 = PF_LUMINANCE as libc::c_int as uint }
            _ => { image.type_0 = PF_UNKNOWN as libc::c_int as uint }
        }
    }
    // setup additional flags
    if (*hdr).dsCaps.dwCaps1 as libc::c_long & 0x8 as libc::c_long != 0 &&
           (*hdr).dsCaps.dwCaps2 as libc::c_long & 0x200 as libc::c_long != 0
       {
        image.flags |= IMAGE_CUBEMAP as libc::c_int as libc::c_uint
    }
    if (*hdr).dwFlags as libc::c_long & 0x20000 as libc::c_long != 0 {
        image.num_mips = (*hdr).dwMipMapCount as byte
    };
    // get actual mip count
}
#[no_mangle]
pub unsafe extern "C" fn Image_DXTGetLinearSize(mut type_0: libc::c_int,
                                                mut width: libc::c_int,
                                                mut height: libc::c_int,
                                                mut depth: libc::c_int)
 -> size_t {
    match type_0 {
        8 => {
            return ((width + 3 as libc::c_int) / 4 as libc::c_int *
                        ((height + 3 as libc::c_int) / 4 as libc::c_int) *
                        depth * 8 as libc::c_int) as size_t
        }
        9 | 10 | 11 => {
            return ((width + 3 as libc::c_int) / 4 as libc::c_int *
                        ((height + 3 as libc::c_int) / 4 as libc::c_int) *
                        depth * 16 as libc::c_int) as size_t
        }
        7 => { return (width * height * depth) as size_t }
        6 | 5 => {
            return (width * height * depth * 3 as libc::c_int) as size_t
        }
        4 | 3 => {
            return (width * height * depth * 4 as libc::c_int) as size_t
        }
        _ => { }
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn Image_DXTCalcMipmapSize(mut hdr: *mut dds_t)
 -> size_t {
    let mut buffsize: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    // now correct buffer size
    i = 0 as libc::c_int;
    while (i as libc::c_uint) <
              (if 1 as libc::c_int as libc::c_uint > (*hdr).dwMipMapCount {
                   1 as libc::c_int as libc::c_uint
               } else { (*hdr).dwMipMapCount }) {
        width =
            if 1 as libc::c_int as libc::c_uint > (*hdr).dwWidth >> i {
                1 as libc::c_int as libc::c_uint
            } else { ((*hdr).dwWidth) >> i } as libc::c_int;
        height =
            if 1 as libc::c_int as libc::c_uint > (*hdr).dwHeight >> i {
                1 as libc::c_int as libc::c_uint
            } else { ((*hdr).dwHeight) >> i } as libc::c_int;
        buffsize =
            (buffsize as
                 libc::c_ulong).wrapping_add(Image_DXTGetLinearSize(image.type_0
                                                                        as
                                                                        libc::c_int,
                                                                    width,
                                                                    height,
                                                                    image.depth
                                                                        as
                                                                        libc::c_int))
                as size_t as size_t;
        i += 1
    }
    return buffsize;
}
#[no_mangle]
pub unsafe extern "C" fn Image_DXTCalcSize(mut name: *const libc::c_char,
                                           mut hdr: *mut dds_t,
                                           mut filesize: size_t) -> uint {
    let mut buffsize: size_t = 0 as libc::c_int as size_t;
    let mut w: libc::c_int = image.width as libc::c_int;
    let mut h: libc::c_int = image.height as libc::c_int;
    let mut d: libc::c_int = image.depth as libc::c_int;
    if (*hdr).dsCaps.dwCaps2 as libc::c_long & 0x200 as libc::c_long != 0 {
        // cubemap w*h always match for all sides
        buffsize =
            Image_DXTCalcMipmapSize(hdr).wrapping_mul(6 as libc::c_int as
                                                          libc::c_ulong)
    } else if (*hdr).dwFlags as libc::c_long & 0x20000 as libc::c_long != 0 {
        // if mipcount > 1
        buffsize = Image_DXTCalcMipmapSize(hdr)
    } else if (*hdr).dwFlags as libc::c_long &
                  (0x80000 as libc::c_long | 0x8 as libc::c_long) != 0 {
        // just in case (no need, really)
        buffsize = (*hdr).dwLinearSize as size_t
    } else {
        // pretty solution for microsoft bug
        buffsize = Image_DXTCalcMipmapSize(hdr)
    }
    if filesize != buffsize {
        // main check
        Con_DPrintf(b"^3Warning:^7 Image_LoadDDS: (%s) probably corrupted (%i should be %lu)\n\x00"
                        as *const u8 as *const libc::c_char, name, buffsize,
                    filesize);
        if buffsize > filesize { return false_0 as libc::c_int as uint }
    }
    return buffsize as uint;
}
#[no_mangle]
pub unsafe extern "C" fn Image_DXTAdjustVolume(mut hdr: *mut dds_t) {
    if (*hdr).dwDepth <= 1 as libc::c_int as libc::c_uint { return }
    (*hdr).dwLinearSize =
        Image_DXTGetLinearSize(image.type_0 as libc::c_int,
                               (*hdr).dwWidth as libc::c_int,
                               (*hdr).dwHeight as libc::c_int,
                               (*hdr).dwDepth as libc::c_int) as uint32_t;
    (*hdr).dwFlags =
        ((*hdr).dwFlags as libc::c_long | 0x80000 as libc::c_long) as
            uint32_t;
}
/*
=============
Image_LoadDDS
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadDDS(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut header: dds_t =
        dds_t{dwIdent: 0,
              dwSize: 0,
              dwFlags: 0,
              dwHeight: 0,
              dwWidth: 0,
              dwLinearSize: 0,
              dwDepth: 0,
              dwMipMapCount: 0,
              dwAlphaBitDepth: 0,
              dwReserved1: [0; 10],
              dsPixelFormat:
                  dds_pixf_t{dwSize: 0,
                             dwFlags: 0,
                             dwFourCC: 0,
                             dwRGBBitCount: 0,
                             dwRBitMask: 0,
                             dwGBitMask: 0,
                             dwBBitMask: 0,
                             dwABitMask: 0,},
              dsCaps:
                  dds_caps_t{dwCaps1: 0, dwCaps2: 0, dwCaps3: 0, dwCaps4: 0,},
              dwTextureStage: 0,}; // it's not a dds file, just skip it
    let mut fin: *mut byte = 0 as *mut byte;
    if (filesize as libc::c_ulong) <
           ::std::mem::size_of::<dds_t>() as libc::c_ulong {
        return false_0
    }
    memcpy(&mut header as *mut dds_t as *mut libc::c_void,
           buffer as *const libc::c_void,
           ::std::mem::size_of::<dds_t>() as libc::c_ulong);
    if header.dwIdent !=
           (((' ' as i32) << 24 as libc::c_int) +
                (('S' as i32) << 16 as libc::c_int) +
                (('D' as i32) << 8 as libc::c_int) + 'D' as i32) as
               libc::c_uint {
        return false_0
    }
    if header.dwSize as libc::c_ulong !=
           (::std::mem::size_of::<dds_t>() as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<uint>() as
                                                libc::c_ulong) {
        // size of the structure (minus MagicNum)
        Con_DPrintf(b"^1Error:^7 Image_LoadDDS: (%s) have corrupted header\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    if header.dsPixelFormat.dwSize as libc::c_ulong !=
           ::std::mem::size_of::<dds_pixf_t>() as libc::c_ulong {
        // size of the structure
        Con_DPrintf(b"^1Error:^7 Image_LoadDDS: (%s) have corrupt pixelformat header\n\x00"
                        as *const u8 as *const libc::c_char,
                    name); // and image type too :)
        return false_0
    } // silently rejected
    image.width = header.dwWidth as word; // just in case
    image.height = header.dwHeight as word;
    if header.dwFlags as libc::c_long & 0x800000 as libc::c_long != 0 {
        image.depth = header.dwDepth as word
    } else { image.depth = 1 as libc::c_int as word }
    if Image_ValidSize(name) as u64 == 0 { return false_0 }
    Image_DXTGetPixelFormat(&mut header);
    Image_DXTAdjustVolume(&mut header);
    if Image_CheckFlag(IL_DDS_HARDWARE as libc::c_int) as u64 == 0 &&
           (image.type_0 == PF_DXT1 as libc::c_int as libc::c_uint ||
                image.type_0 == PF_DXT3 as libc::c_int as libc::c_uint ||
                image.type_0 == PF_DXT5 as libc::c_int as libc::c_uint ||
                image.type_0 == PF_ATI2 as libc::c_int as libc::c_uint) {
        return false_0
    }
    if image.type_0 == PF_UNKNOWN as libc::c_int as libc::c_uint {
        Con_DPrintf(b"^1Error:^7 Image_LoadDDS: (%s) has unrecognized type\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    image.size =
        Image_DXTCalcSize(name, &mut header,
                          (filesize - 128 as libc::c_int as libc::c_long) as
                              size_t) as size_t;
    if image.size == 0 as libc::c_int as libc::c_ulong { return false_0 }
    fin =
        buffer.offset(::std::mem::size_of::<dds_t>() as libc::c_ulong as
                          isize) as *mut byte;
    // copy an encode method
    image.encode = header.dwReserved1[0 as libc::c_int as usize] as word;
    match image.encode as libc::c_int {
        6657 => {
            image.flags =
                image.flags | IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
        }
        6661 | 6662 | 6663 | 6664 | 6665 => {
            image.flags =
                image.flags | IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
        }
        _ => {
            // check for real alpha-pixels
            if image.type_0 == PF_DXT3 as libc::c_int as libc::c_uint &&
                   Image_CheckDXT3Alpha(&mut header, fin) as libc::c_uint != 0
               {
                image.flags =
                    image.flags |
                        IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
            } else if image.type_0 == PF_DXT5 as libc::c_int as libc::c_uint
                          &&
                          Image_CheckDXT5Alpha(&mut header, fin) as
                              libc::c_uint != 0 {
                image.flags =
                    image.flags |
                        IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
            }
            if header.dsPixelFormat.dwFlags as libc::c_long &
                   0x20000 as libc::c_long == 0 {
                image.flags =
                    image.flags |
                        IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
            }
        }
    }
    if image.type_0 == PF_LUMINANCE as libc::c_int as libc::c_uint {
        image.flags =
            image.flags &
                !(IMAGE_HAS_COLOR as libc::c_int |
                      IMAGE_HAS_ALPHA as libc::c_int) as libc::c_uint
    }
    if header.dwReserved1[1 as libc::c_int as usize] !=
           0 as libc::c_int as libc::c_uint {
        // store texture reflectivity
        image.fogParams[0 as libc::c_int as usize] =
            ((header.dwReserved1[1 as libc::c_int as usize] &
                  0xff as libc::c_int as libc::c_uint) >> 0 as libc::c_int) as
                byte;
        image.fogParams[1 as libc::c_int as usize] =
            ((header.dwReserved1[1 as libc::c_int as usize] &
                  0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
                as byte;
        image.fogParams[2 as libc::c_int as usize] =
            ((header.dwReserved1[1 as libc::c_int as usize] &
                  0xff0000 as libc::c_int as libc::c_uint) >>
                 16 as libc::c_int) as byte;
        image.fogParams[3 as libc::c_int as usize] =
            ((header.dwReserved1[1 as libc::c_int as usize] &
                  0xff000000 as libc::c_uint) >> 24 as libc::c_int) as byte
    }
    // dds files will be uncompressed on a render. requires minimal of info for set this
    image.rgba =
        _Mem_Alloc(host.imagepool, image.size, false_0,
                   b"../engine/common/imagelib/img_dds.c\x00" as *const u8 as
                       *const libc::c_char, 341 as libc::c_int) as *mut byte;
    memcpy(image.rgba as *mut libc::c_void, fin as *const libc::c_void,
           image.size);
    image.flags =
        image.flags | IMAGE_DDS_FORMAT as libc::c_int as libc::c_uint;
    return true_0;
}
