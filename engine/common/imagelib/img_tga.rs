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
    static mut host: host_parm_t;
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type tga_t = tga_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct tga_s {
    pub id_length: uint8_t,
    pub colormap_type: uint8_t,
    pub image_type: uint8_t,
    pub colormap_index: uint16_t,
    pub colormap_length: uint16_t,
    pub colormap_size: uint8_t,
    pub x_origin: uint16_t,
    pub y_origin: uint16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub pixel_size: uint8_t,
    pub attributes: uint8_t,
}
/*
img_tga.c - tga format load & save
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
/*
=============
Image_LoadTGA
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadTGA(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut i: libc::c_int = 0; // skip TARGA image comment
    let mut columns: libc::c_int = 0;
    let mut rows: libc::c_int = 0;
    let mut row_inc: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut buf_p: *mut byte = 0 as *mut byte;
    let mut pixbuf: *mut byte = 0 as *mut byte;
    let mut targa_rgba: *mut byte = 0 as *mut byte;
    let mut palette: [rgba_t; 256] = [[0; 4]; 256];
    let mut red: byte = 0 as libc::c_int as byte;
    let mut green: byte = 0 as libc::c_int as byte;
    let mut blue: byte = 0 as libc::c_int as byte;
    let mut alpha: byte = 0 as libc::c_int as byte;
    let mut readpixelcount: libc::c_int = 0;
    let mut pixelcount: libc::c_int = 0;
    let mut reflectivity: [libc::c_int; 3] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
    let mut compressed: qboolean = false_0;
    let mut targa_header: tga_t =
        tga_t{id_length: 0,
              colormap_type: 0,
              image_type: 0,
              colormap_index: 0,
              colormap_length: 0,
              colormap_size: 0,
              x_origin: 0,
              y_origin: 0,
              width: 0,
              height: 0,
              pixel_size: 0,
              attributes: 0,};
    if (filesize as libc::c_ulong) <
           ::std::mem::size_of::<tga_t>() as libc::c_ulong {
        return false_0
    }
    buf_p = buffer as *mut byte;
    let fresh0 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.id_length = *fresh0;
    let fresh1 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.colormap_type = *fresh1;
    let fresh2 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.image_type = *fresh2;
    targa_header.colormap_index =
        (*buf_p.offset(0 as libc::c_int as isize) as libc::c_int +
             *buf_p.offset(1 as libc::c_int as isize) as libc::c_int *
                 256 as libc::c_int) as uint16_t;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header.colormap_length =
        (*buf_p.offset(0 as libc::c_int as isize) as libc::c_int +
             *buf_p.offset(1 as libc::c_int as isize) as libc::c_int *
                 256 as libc::c_int) as uint16_t;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header.colormap_size = *buf_p;
    buf_p = buf_p.offset(1 as libc::c_int as isize);
    targa_header.x_origin = *(buf_p as *mut libc::c_short) as uint16_t;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header.y_origin = *(buf_p as *mut libc::c_short) as uint16_t;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    image.width = *(buf_p as *mut libc::c_short) as word;
    targa_header.width = image.width;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    image.height = *(buf_p as *mut libc::c_short) as word;
    targa_header.height = image.height;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    let fresh3 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.pixel_size = *fresh3;
    let fresh4 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.attributes = *fresh4;
    if targa_header.id_length as libc::c_int != 0 as libc::c_int {
        buf_p = buf_p.offset(targa_header.id_length as libc::c_int as isize)
    }
    // check for tga file
    if Image_ValidSize(name) as u64 == 0 {
        return false_0
    } // always exctracted to 32-bit buffer
    image.type_0 = PF_RGBA_32 as libc::c_int as uint;
    if targa_header.image_type as libc::c_int == 1 as libc::c_int ||
           targa_header.image_type as libc::c_int == 9 as libc::c_int {
        // uncompressed colormapped image
        if targa_header.pixel_size as libc::c_int != 8 as libc::c_int {
            Con_DPrintf(b"^1Error:^7 Image_LoadTGA: (%s) Only 8 bit images supported for type 1 and 9\n\x00"
                            as *const u8 as *const libc::c_char, name);
            return false_0
        }
        if targa_header.colormap_length as libc::c_int != 256 as libc::c_int {
            Con_DPrintf(b"^1Error:^7 Image_LoadTGA: (%s) Only 8 bit colormaps are supported for type 1 and 9\n\x00"
                            as *const u8 as *const libc::c_char, name);
            return false_0
        }
        if targa_header.colormap_index != 0 {
            Con_DPrintf(b"^1Error:^7 Image_LoadTGA: (%s) colormap_index is not supported for type 1 and 9\n\x00"
                            as *const u8 as *const libc::c_char, name);
            return false_0
        }
        if targa_header.colormap_size as libc::c_int == 24 as libc::c_int {
            i = 0 as libc::c_int;
            while i < targa_header.colormap_length as libc::c_int {
                let fresh5 = buf_p;
                buf_p = buf_p.offset(1);
                palette[i as usize][2 as libc::c_int as usize] = *fresh5;
                let fresh6 = buf_p;
                buf_p = buf_p.offset(1);
                palette[i as usize][1 as libc::c_int as usize] = *fresh6;
                let fresh7 = buf_p;
                buf_p = buf_p.offset(1);
                palette[i as usize][0 as libc::c_int as usize] = *fresh7;
                palette[i as usize][3 as libc::c_int as usize] =
                    255 as libc::c_int as byte;
                i += 1
            }
        } else if targa_header.colormap_size as libc::c_int ==
                      32 as libc::c_int {
            i = 0 as libc::c_int;
            while i < targa_header.colormap_length as libc::c_int {
                let fresh8 = buf_p;
                buf_p = buf_p.offset(1);
                palette[i as usize][2 as libc::c_int as usize] = *fresh8;
                let fresh9 = buf_p;
                buf_p = buf_p.offset(1);
                palette[i as usize][1 as libc::c_int as usize] = *fresh9;
                let fresh10 = buf_p;
                buf_p = buf_p.offset(1);
                palette[i as usize][0 as libc::c_int as usize] = *fresh10;
                let fresh11 = buf_p;
                buf_p = buf_p.offset(1);
                palette[i as usize][3 as libc::c_int as usize] = *fresh11;
                i += 1
            }
        } else {
            Con_DPrintf(b"^1Error:^7 Image_LoadTGA: (%s) only 24 and 32 bit colormaps are supported for type 1 and 9\n\x00"
                            as *const u8 as *const libc::c_char, name);
            return false_0
        }
    } else if targa_header.image_type as libc::c_int == 2 as libc::c_int ||
                  targa_header.image_type as libc::c_int == 10 as libc::c_int
     {
        // uncompressed or RLE compressed RGB
        if targa_header.pixel_size as libc::c_int != 32 as libc::c_int &&
               targa_header.pixel_size as libc::c_int != 24 as libc::c_int {
            Con_DPrintf(b"^1Error:^7 Image_LoadTGA: (%s) Only 32 or 24 bit images supported for type 2 and 10\n\x00"
                            as *const u8 as *const libc::c_char, name);
            return false_0
        }
    } else if targa_header.image_type as libc::c_int == 3 as libc::c_int ||
                  targa_header.image_type as libc::c_int == 11 as libc::c_int
     {
        // uncompressed greyscale
        if targa_header.pixel_size as libc::c_int != 8 as libc::c_int &&
               targa_header.pixel_size as libc::c_int != 16 as libc::c_int {
            Con_DPrintf(b"^1Error:^7 Image_LoadTGA: (%s) Only 8 bit images supported for type 3 and 11\n\x00"
                            as *const u8 as *const libc::c_char, name);
            return false_0
        }
    }
    columns = targa_header.width as libc::c_int;
    rows = targa_header.height as libc::c_int;
    image.size =
        (image.width as libc::c_int * image.height as libc::c_int *
             4 as libc::c_int) as size_t;
    image.rgba =
        _Mem_Alloc(host.imagepool, image.size, false_0,
                   b"../engine/common/imagelib/img_tga.c\x00" as *const u8 as
                       *const libc::c_char, 127 as libc::c_int) as *mut byte;
    targa_rgba = image.rgba;
    // if bit 5 of attributes isn't set, the image has been stored from bottom to top
    if Image_CheckFlag(IL_DONTFLIP_TGA as libc::c_int) as u64 == 0 &&
           targa_header.attributes as libc::c_int & 0x20 as libc::c_int != 0 {
        pixbuf = targa_rgba;
        row_inc = 0 as libc::c_int
    } else {
        pixbuf =
            targa_rgba.offset(((rows - 1 as libc::c_int) * columns *
                                   4 as libc::c_int) as isize);
        row_inc = -columns * 4 as libc::c_int * 2 as libc::c_int
    }
    compressed =
        (targa_header.image_type as libc::c_int == 9 as libc::c_int ||
             targa_header.image_type as libc::c_int == 10 as libc::c_int ||
             targa_header.image_type as libc::c_int == 11 as libc::c_int) as
            libc::c_int as qboolean;
    col = 0 as libc::c_int;
    row = col;
    while row < rows {
        pixelcount = 0x10000 as libc::c_int;
        readpixelcount = 0x10000 as libc::c_int;
        if compressed as u64 != 0 {
            let fresh12 = buf_p;
            buf_p = buf_p.offset(1);
            pixelcount = *fresh12 as libc::c_int;
            if pixelcount & 0x80 as libc::c_int != 0 {
                // run-length packet
                readpixelcount = 1 as libc::c_int
            }
            pixelcount = 1 as libc::c_int + (pixelcount & 0x7f as libc::c_int)
        }
        loop  {
            let fresh13 = pixelcount;
            pixelcount = pixelcount - 1;
            if !(fresh13 != 0 && row < rows) { break ; }
            let fresh14 = readpixelcount;
            readpixelcount = readpixelcount - 1;
            if fresh14 > 0 as libc::c_int {
                match targa_header.image_type as libc::c_int {
                    1 | 9 => {
                        // colormapped image
                        let fresh15 = buf_p;
                        buf_p = buf_p.offset(1);
                        blue = *fresh15;
                        if (blue as libc::c_int) <
                               targa_header.colormap_length as libc::c_int {
                            red =
                                palette[blue as
                                            usize][0 as libc::c_int as usize];
                            green =
                                palette[blue as
                                            usize][1 as libc::c_int as usize];
                            alpha =
                                palette[blue as
                                            usize][3 as libc::c_int as usize];
                            blue =
                                palette[blue as
                                            usize][2 as libc::c_int as usize];
                            if alpha as libc::c_int != 255 as libc::c_int {
                                image.flags |=
                                    IMAGE_HAS_ALPHA as libc::c_int as
                                        libc::c_uint
                            }
                        }
                    }
                    2 | 10 => {
                        // 24 or 32 bit image
                        let fresh16 = buf_p;
                        buf_p = buf_p.offset(1);
                        blue = *fresh16;
                        let fresh17 = buf_p;
                        buf_p = buf_p.offset(1);
                        green = *fresh17;
                        let fresh18 = buf_p;
                        buf_p = buf_p.offset(1);
                        red = *fresh18;
                        alpha = 255 as libc::c_int as byte;
                        if targa_header.pixel_size as libc::c_int ==
                               32 as libc::c_int {
                            let fresh19 = buf_p;
                            buf_p = buf_p.offset(1);
                            alpha = *fresh19;
                            if alpha as libc::c_int != 255 as libc::c_int {
                                image.flags |=
                                    IMAGE_HAS_ALPHA as libc::c_int as
                                        libc::c_uint
                            }
                        }
                    }
                    3 | 11 => {
                        // greyscale image
                        let fresh20 = buf_p;
                        buf_p = buf_p.offset(1);
                        red = *fresh20;
                        green = red;
                        blue = green;
                        if targa_header.pixel_size as libc::c_int ==
                               16 as libc::c_int {
                            let fresh21 = buf_p;
                            buf_p = buf_p.offset(1);
                            alpha = *fresh21;
                            if alpha as libc::c_int != 255 as libc::c_int {
                                image.flags |=
                                    IMAGE_HAS_ALPHA as libc::c_int as
                                        libc::c_uint
                            }
                        } else { alpha = 255 as libc::c_int as byte }
                    }
                    _ => { }
                }
            }
            if red as libc::c_int != green as libc::c_int ||
                   green as libc::c_int != blue as libc::c_int {
                image.flags |= IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
            }
            reflectivity[0 as libc::c_int as usize] += red as libc::c_int;
            reflectivity[1 as libc::c_int as usize] += green as libc::c_int;
            reflectivity[2 as libc::c_int as usize] += blue as libc::c_int;
            let fresh22 = pixbuf;
            pixbuf = pixbuf.offset(1);
            *fresh22 = red;
            let fresh23 = pixbuf;
            pixbuf = pixbuf.offset(1);
            *fresh23 = green;
            let fresh24 = pixbuf;
            pixbuf = pixbuf.offset(1);
            *fresh24 = blue;
            let fresh25 = pixbuf;
            pixbuf = pixbuf.offset(1);
            *fresh25 = alpha;
            col += 1;
            if col == columns {
                // run spans across rows
                row += 1;
                col = 0 as libc::c_int;
                pixbuf = pixbuf.offset(row_inc as isize)
            }
        }
    }
    image.fogParams[0 as libc::c_int as usize] =
        (reflectivity[0 as libc::c_int as usize] as libc::c_float *
             (1.0f32 /
                  (image.width as libc::c_int * image.height as libc::c_int)
                      as libc::c_float)) as byte;
    image.fogParams[1 as libc::c_int as usize] =
        (reflectivity[1 as libc::c_int as usize] as libc::c_float *
             (1.0f32 /
                  (image.width as libc::c_int * image.height as libc::c_int)
                      as libc::c_float)) as byte;
    image.fogParams[2 as libc::c_int as usize] =
        (reflectivity[2 as libc::c_int as usize] as libc::c_float *
             (1.0f32 /
                  (image.width as libc::c_int * image.height as libc::c_int)
                      as libc::c_float)) as byte;
    image.depth = 1 as libc::c_int as word;
    return true_0;
}
/*
=============
Image_SaveTGA
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_SaveTGA(mut name: *const libc::c_char,
                                       mut pix: *mut rgbdata_t) -> qboolean {
    let mut y: libc::c_int = 0; // already existed
    let mut outsize: libc::c_int = 0;
    let mut pixel_size: libc::c_int = 0;
    let mut bufend: *const uint8_t = 0 as *const uint8_t;
    let mut in_0: *const uint8_t = 0 as *const uint8_t;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut out: *mut uint8_t = 0 as *mut uint8_t;
    let mut targa_header: tga_t =
        {
            let mut init =
                tga_s{id_length: 0 as libc::c_int as uint8_t,
                      colormap_type: 0,
                      image_type: 0,
                      colormap_index: 0,
                      colormap_length: 0,
                      colormap_size: 0,
                      x_origin: 0,
                      y_origin: 0,
                      width: 0,
                      height: 0,
                      pixel_size: 0,
                      attributes: 0,};
            init
        };
    let comment: [libc::c_char; 27] =
        *::std::mem::transmute::<&[u8; 27],
                                 &[libc::c_char; 27]>(b"Generated by Xash ImageLib\x00");
    if FS_FileExists(name, false_0 as libc::c_int) != 0 &&
           Image_CheckFlag(IL_ALLOW_OVERWRITE as libc::c_int) as u64 == 0 {
        return false_0
    }
    // bogus parameter check
    if (*pix).buffer.is_null() { return false_0 }
    // get image description
    match (*pix).type_0 {
        5 | 6 => { pixel_size = 3 as libc::c_int }
        3 | 4 => { pixel_size = 4 as libc::c_int }
        _ => { return false_0 }
    }
    outsize =
        (*pix).width as libc::c_int * (*pix).height as libc::c_int *
            pixel_size;
    outsize =
        (outsize as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<tga_t>() as
                                             libc::c_ulong) as libc::c_int as
            libc::c_int;
    outsize =
        (outsize as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<[libc::c_char; 27]>()
                                              as
                                              libc::c_ulong).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_int as libc::c_int;
    buffer =
        _Mem_Alloc(host.imagepool, outsize as size_t, false_0,
                   b"../engine/common/imagelib/img_tga.c\x00" as *const u8 as
                       *const libc::c_char, 266 as libc::c_int) as
            *mut uint8_t;
    // prepare header
    targa_header.id_length =
        (::std::mem::size_of::<[libc::c_char; 27]>() as
             libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            uint8_t; // tga comment length
    targa_header.image_type =
        2 as libc::c_int as uint8_t; // uncompressed type
    targa_header.width = (*pix).width;
    targa_header.height = (*pix).height;
    if (*pix).flags & IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint != 0 {
        targa_header.pixel_size = 32 as libc::c_int as uint8_t;
        targa_header.attributes = 8 as libc::c_int as uint8_t
        // 8 bits of alpha
    } else {
        targa_header.pixel_size = 24 as libc::c_int as uint8_t;
        targa_header.attributes = 0 as libc::c_int as uint8_t
    }
    out = buffer;
    memcpy(out as *mut libc::c_void,
           &mut targa_header as *mut tga_t as *const libc::c_void,
           ::std::mem::size_of::<tga_t>() as libc::c_ulong);
    out =
        out.offset(::std::mem::size_of::<tga_t>() as libc::c_ulong as isize);
    memcpy(out as *mut libc::c_void, comment.as_ptr() as *const libc::c_void,
           (::std::mem::size_of::<[libc::c_char; 27]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                libc::c_ulong));
    out =
        out.offset((::std::mem::size_of::<[libc::c_char; 27]>() as
                        libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong) as
                       isize);
    match (*pix).type_0 {
        5 | 3 => {
            // swap rgba to bgra and flip upside down
            y = (*pix).height as libc::c_int - 1 as libc::c_int;
            while y >= 0 as libc::c_int {
                in_0 =
                    (*pix).buffer.offset((y * (*pix).width as libc::c_int *
                                              pixel_size) as isize);
                bufend =
                    in_0.offset(((*pix).width as libc::c_int * pixel_size) as
                                    isize);
                while in_0 < bufend {
                    let fresh26 = out;
                    out = out.offset(1);
                    *fresh26 = *in_0.offset(2 as libc::c_int as isize);
                    let fresh27 = out;
                    out = out.offset(1);
                    *fresh27 = *in_0.offset(1 as libc::c_int as isize);
                    let fresh28 = out;
                    out = out.offset(1);
                    *fresh28 = *in_0.offset(0 as libc::c_int as isize);
                    if (*pix).flags &
                           IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint != 0
                       {
                        let fresh29 = out;
                        out = out.offset(1);
                        *fresh29 = *in_0.offset(3 as libc::c_int as isize)
                    }
                    in_0 = in_0.offset(pixel_size as isize)
                }
                y -= 1
            }
        }
        6 | 4 => {
            // flip upside down
            y = (*pix).height as libc::c_int - 1 as libc::c_int;
            while y >= 0 as libc::c_int {
                in_0 =
                    (*pix).buffer.offset((y * (*pix).width as libc::c_int *
                                              pixel_size) as isize);
                bufend =
                    in_0.offset(((*pix).width as libc::c_int * pixel_size) as
                                    isize);
                while in_0 < bufend {
                    let fresh30 = out;
                    out = out.offset(1);
                    *fresh30 = *in_0.offset(0 as libc::c_int as isize);
                    let fresh31 = out;
                    out = out.offset(1);
                    *fresh31 = *in_0.offset(1 as libc::c_int as isize);
                    let fresh32 = out;
                    out = out.offset(1);
                    *fresh32 = *in_0.offset(2 as libc::c_int as isize);
                    if (*pix).flags &
                           IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint != 0
                       {
                        let fresh33 = out;
                        out = out.offset(1);
                        *fresh33 = *in_0.offset(3 as libc::c_int as isize)
                    }
                    in_0 = in_0.offset(pixel_size as isize)
                }
                y -= 1
            }
        }
        _ => { }
    }
    FS_WriteFile(name, buffer as *const libc::c_void, outsize as fs_offset_t);
    _Mem_Free(buffer as *mut libc::c_void,
              b"../engine/common/imagelib/img_tga.c\x00" as *const u8 as
                  *const libc::c_char, 333 as libc::c_int);
    return true_0;
}
