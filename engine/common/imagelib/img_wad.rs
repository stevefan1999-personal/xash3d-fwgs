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
    fn Q_strrchr(s: *const libc::c_char, c: libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_stristr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut image: imglib_t;
    #[no_mangle]
    fn Image_AddIndexedImageToPack(in_0: *const byte, width: libc::c_int,
                                   height: libc::c_int) -> qboolean;
    #[no_mangle]
    fn Image_GetPaletteLMP(pal: *const byte, rendermode: libc::c_int);
    #[no_mangle]
    fn Image_ComparePalette(pal: *const byte) -> libc::c_int;
    #[no_mangle]
    fn Image_CopyPalette32bit();
    #[no_mangle]
    fn Image_GetPaletteQ1();
    #[no_mangle]
    fn Image_CheckFlag(bit: libc::c_int) -> qboolean;
    #[no_mangle]
    fn Image_ValidSize(name: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Image_LumpValidSize(name: *const libc::c_char) -> qboolean;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const LUMP_QUAKE1: C2RustUnnamed_2 = 5;
pub const LUMP_HALFLIFE: C2RustUnnamed_2 = 4;
pub const LUMP_EXTENDED: C2RustUnnamed_2 = 3;
pub const LUMP_GRADIENT: C2RustUnnamed_2 = 2;
pub const LUMP_MASKED: C2RustUnnamed_2 = 1;
pub const LUMP_NORMAL: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_int;
pub const PAL_HALFLIFE: C2RustUnnamed_3 = 2;
pub const PAL_QUAKE1: C2RustUnnamed_3 = 1;
pub const PAL_CUSTOM: C2RustUnnamed_3 = 0;
pub const PAL_INVALID: C2RustUnnamed_3 = -1;
pub type mip_t = mip_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mip_s {
    pub name: [libc::c_char; 16],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub offsets: [libc::c_uint; 4],
}
pub type mstudiotexture_t = mstudiotex_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dspriteframe_t {
    pub origin: [libc::c_int; 2],
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type qfont_t = qfont_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qfont_s {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub rowcount: libc::c_int,
    pub rowheight: libc::c_int,
    pub fontinfo: [charinfo; 256],
    pub data: [byte; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charinfo {
    pub startoffset: libc::c_short,
    pub charwidth: libc::c_short,
}
pub type lmp_t = lmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lmp_s {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
}
/*
img_mip.c - hl1 and q1 image mips
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
============
Image_LoadPAL
============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadPAL(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut rendermode: libc::c_int = LUMP_NORMAL as libc::c_int;
    if filesize != 768 as libc::c_int as libc::c_long {
        Con_DPrintf(b"^1Error:^7 Image_LoadPAL: (%s) have invalid size (%ld should be %d)\n\x00"
                        as *const u8 as *const libc::c_char, name, filesize,
                    768 as libc::c_int);
        return false_0
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        // using palette name as rendermode
        if !Q_stristr(name,
                      b"normal\x00" as *const u8 as
                          *const libc::c_char).is_null() {
            rendermode = LUMP_NORMAL as libc::c_int
        } else if !Q_stristr(name,
                             b"masked\x00" as *const u8 as
                                 *const libc::c_char).is_null() {
            rendermode = LUMP_MASKED as libc::c_int
        } else if !Q_stristr(name,
                             b"gradient\x00" as *const u8 as
                                 *const libc::c_char).is_null() {
            rendermode = LUMP_GRADIENT as libc::c_int
        } else if !Q_stristr(name,
                             b"valve\x00" as *const u8 as
                                 *const libc::c_char).is_null() {
            rendermode = LUMP_HALFLIFE as libc::c_int;
            buffer = 0 as *const byte
            // force to get HL palette
        } else if !Q_stristr(name,
                             b"id\x00" as *const u8 as
                                 *const libc::c_char).is_null() {
            rendermode = LUMP_QUAKE1 as libc::c_int;
            buffer = 0 as *const byte
            // force to get Q1 palette
        }
    }
    // NOTE: image.d_currentpal not cleared with Image_Reset()
	// and stay valid any time before new call of Image_SetPalette
    Image_GetPaletteLMP(buffer, rendermode); // only palette, not real image
    Image_CopyPalette32bit(); // expanded palette
    image.rgba = 0 as *mut byte;
    image.size = 1024 as libc::c_int as size_t;
    image.height = 0 as libc::c_int as word;
    image.width = image.height;
    image.depth = 1 as libc::c_int as word;
    return true_0;
}
/*
============
Image_LoadFNT
============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadFNT(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut font: qfont_t =
        qfont_t{width: 0,
                height: 0,
                rowcount: 0,
                rowheight: 0,
                fontinfo: [charinfo{startoffset: 0, charwidth: 0,}; 256],
                data: [0; 4],}; // Quake1 doesn't have qfonts
    let mut pal: *const byte = 0 as *const byte;
    let mut fin: *const byte = 0 as *const byte;
    let mut size: size_t = 0;
    let mut numcolors: libc::c_int = 0;
    if image.hint as libc::c_uint == IL_HINT_Q1 as libc::c_int as libc::c_uint
       {
        return false_0
    }
    if (filesize as libc::c_ulong) <
           ::std::mem::size_of::<qfont_t>() as libc::c_ulong {
        return false_0
    }
    memcpy(&mut font as *mut qfont_t as *mut libc::c_void,
           buffer as *const libc::c_void,
           ::std::mem::size_of::<qfont_t>() as libc::c_ulong);
    // last sixty four bytes - what the hell ????
    size =
        (::std::mem::size_of::<qfont_t>() as
             libc::c_ulong).wrapping_sub(4 as libc::c_int as
                                             libc::c_ulong).wrapping_add((font.height
                                                                              *
                                                                              font.width
                                                                              *
                                                                              16
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_short>()
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_add(768
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_ulong).wrapping_add(64
                                                                                                                                                                             as
                                                                                                                                                                             libc::c_int
                                                                                                                                                                             as
                                                                                                                                                                             libc::c_ulong);
    if size != filesize as libc::c_ulong {
        // oldstyle font: "conchars" or "creditsfont"
        image.width = 256 as libc::c_int as word; // hardcoded
        image.height = font.height as word
    } else {
        // Half-Life 1.1.0.0 font style (qfont_t)
        image.width = (font.width * 16 as libc::c_int) as word;
        image.height = font.height as word
    }
    if Image_LumpValidSize(name) as u64 == 0 { return false_0 }
    fin =
        buffer.offset(::std::mem::size_of::<qfont_t>() as libc::c_ulong as
                          isize).offset(-(4 as libc::c_int as isize));
    pal =
        fin.offset((image.width as libc::c_int * image.height as libc::c_int)
                       as isize);
    numcolors = *(pal as *mut libc::c_short) as libc::c_int;
    pal =
        pal.offset(::std::mem::size_of::<libc::c_short>() as libc::c_ulong as
                       isize);
    if numcolors == 768 as libc::c_int || numcolors == 256 as libc::c_int {
        // g-cont. make sure that is didn't hit anything
        Image_GetPaletteLMP(pal, LUMP_MASKED as libc::c_int);
        image.flags |= IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
        // fonts always have transparency
    } else { return false_0 } // 32-bit palette
    image.type_0 = PF_INDEXED_32 as libc::c_int as uint;
    image.depth = 1 as libc::c_int as word;
    return Image_AddIndexedImageToPack(fin, image.width as libc::c_int,
                                       image.height as libc::c_int);
}
/*
======================
Image_SetMDLPointer

Transfer buffer pointer before Image_LoadMDL
======================
*/
static mut g_mdltexdata: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn Image_SetMDLPointer(mut p: *mut byte) {
    g_mdltexdata = p as *mut libc::c_void;
}
/*
============
Image_LoadMDL
============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadMDL(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut fin: *mut byte = 0 as *mut byte;
    let mut pixels: size_t = 0;
    let mut pin: *mut mstudiotexture_t = 0 as *mut mstudiotexture_t;
    let mut flags: libc::c_int = 0;
    pin = buffer as *mut mstudiotexture_t;
    flags = (*pin).flags as libc::c_int;
    image.width = (*pin).width as word;
    image.height = (*pin).height as word;
    pixels =
        (image.width as libc::c_int * image.height as libc::c_int) as size_t;
    fin = g_mdltexdata as *mut byte;
    if fin.is_null() {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/common/imagelib/img_wad.c\x00" as *const u8 as
                      *const libc::c_char, 164 as libc::c_int);
    }
    g_mdltexdata = 0 as *mut libc::c_void;
    if Image_ValidSize(name) as u64 == 0 { return false_0 }
    if image.hint as libc::c_uint == IL_HINT_HL as libc::c_int as libc::c_uint
       {
        if (filesize as libc::c_ulong) <
               (::std::mem::size_of::<mstudiotexture_t>() as
                    libc::c_ulong).wrapping_add(pixels).wrapping_add(768 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
           {
            return false_0
        }
        if flags & 0x40 as libc::c_int != 0 {
            let mut pal: *mut byte = fin.offset(pixels as isize);
            Image_GetPaletteLMP(pal, LUMP_MASKED as libc::c_int);
            image.flags |=
                (IMAGE_HAS_ALPHA as libc::c_int |
                     IMAGE_ONEBIT_ALPHA as libc::c_int) as libc::c_uint
        } else {
            Image_GetPaletteLMP(fin.offset(pixels as isize),
                                LUMP_NORMAL as libc::c_int);
        }
    } else {
        return false_0
        // unknown or unsupported mode rejected
    } // 32-bit palete
    image.type_0 = PF_INDEXED_32 as libc::c_int as uint;
    image.depth = 1 as libc::c_int as word;
    return Image_AddIndexedImageToPack(fin, image.width as libc::c_int,
                                       image.height as libc::c_int);
}
/*
============
Image_LoadSPR
============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadSPR(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut pin: dspriteframe_t =
        dspriteframe_t{origin: [0; 2],
                       width: 0,
                       height: 0,}; // identical for q1\hl sprites
    let mut truecolor: qboolean = false_0;
    let mut fin: *mut byte = 0 as *mut byte;
    if image.hint as libc::c_uint == IL_HINT_HL as libc::c_int as libc::c_uint
       {
        if image.d_currentpal.is_null() { return false_0 }
    } else if image.hint as libc::c_uint ==
                  IL_HINT_Q1 as libc::c_int as libc::c_uint {
        Image_GetPaletteQ1();
    } else {
        // unknown mode rejected
        return false_0
    }
    memcpy(&mut pin as *mut dspriteframe_t as *mut libc::c_void,
           buffer as *const libc::c_void,
           ::std::mem::size_of::<dspriteframe_t>() as libc::c_ulong);
    image.width = pin.width as word;
    image.height = pin.height as word;
    if filesize <
           (image.width as libc::c_int * image.height as libc::c_int) as
               libc::c_long {
        return false_0
    }
    if filesize ==
           (image.width as libc::c_int * image.height as libc::c_int *
                4 as libc::c_int) as libc::c_long {
        truecolor = true_0
    }
    // sorry, can't validate palette rendermode
    if Image_LumpValidSize(name) as u64 == 0 {
        return false_0
    } // 32-bit palete
    image.type_0 =
        if truecolor as libc::c_uint != 0 {
            PF_RGBA_32 as libc::c_int
        } else { PF_INDEXED_32 as libc::c_int } as uint;
    image.depth = 1 as libc::c_int as word;
    let mut current_block_21: u64;
    // detect alpha-channel by palette type
    match image.d_rendermode {
        1 => {
            image.flags =
                image.flags |
                    IMAGE_ONEBIT_ALPHA as libc::c_int as libc::c_uint;
            current_block_21 = 9351520129929712126;
        }
        2 | 5 => { current_block_21 = 9351520129929712126; }
        _ => { current_block_21 = 1109700713171191020; }
    }
    match current_block_21 {
        9351520129929712126 =>
        // intentionally fallthrough
        {
            image.flags =
                image.flags | IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
        }
        _ => { }
    }
    fin =
        buffer.offset(::std::mem::size_of::<dspriteframe_t>() as libc::c_ulong
                          as isize) as *mut byte;
    if truecolor as u64 != 0 {
        // spr32 support
        image.size =
            (image.width as libc::c_int * image.height as libc::c_int *
                 4 as libc::c_int) as size_t; // Color. True Color!
        image.rgba =
            _Mem_Alloc(host.imagepool, image.size, false_0,
                       b"../engine/common/imagelib/img_wad.c\x00" as *const u8
                           as *const libc::c_char, 254 as libc::c_int) as
                *mut byte;
        memcpy(image.rgba as *mut libc::c_void, fin as *const libc::c_void,
               image.size);
        image.flags =
            image.flags | IMAGE_HAS_COLOR as libc::c_int as libc::c_uint;
        return true_0
    }
    return Image_AddIndexedImageToPack(fin, image.width as libc::c_int,
                                       image.height as libc::c_int);
}
/*
============
Image_LoadLMP
============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadLMP(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut lmp: lmp_t = lmp_t{width: 0, height: 0,};
    let mut fin: *mut byte = 0 as *mut byte;
    let mut pal: *mut byte = 0 as *mut byte;
    let mut rendermode: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pixels: libc::c_int = 0;
    if (filesize as libc::c_ulong) <
           ::std::mem::size_of::<lmp_t>() as libc::c_ulong {
        return false_0
    }
    // valve software trick (particle palette)
    if !Q_stristr(name,
                  b"palette.lmp\x00" as *const u8 as
                      *const libc::c_char).is_null() {
        return Image_LoadPAL(name, buffer, filesize)
    }
    // id software trick (image without header)
    if !Q_stristr(name,
                  b"conchars\x00" as *const u8 as
                      *const libc::c_char).is_null() &&
           filesize == 16384 as libc::c_int as libc::c_long {
        image.height = 128 as libc::c_int as word;
        image.width = image.height;
        rendermode = LUMP_QUAKE1 as libc::c_int;
        filesize =
            (filesize as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<lmp_t>() as
                                                 libc::c_ulong) as fs_offset_t
                as fs_offset_t;
        fin = buffer as *mut byte;
        // need to remap transparent color from first to last entry
        i = 0 as libc::c_int; // corrupted lump ?
        while i < 16384 as libc::c_int {
            if *fin.offset(i as isize) == 0 {
                *fin.offset(i as isize) = 0xff as libc::c_int as byte
            }
            i += 1
        }
    } else {
        fin = buffer as *mut byte;
        memcpy(&mut lmp as *mut lmp_t as *mut libc::c_void,
               fin as *const libc::c_void,
               ::std::mem::size_of::<lmp_t>() as libc::c_ulong);
        image.width = lmp.width as word;
        image.height = lmp.height as word;
        rendermode = LUMP_NORMAL as libc::c_int;
        fin =
            fin.offset(::std::mem::size_of::<lmp_t>() as libc::c_ulong as
                           isize)
    }
    pixels = image.width as libc::c_int * image.height as libc::c_int;
    if (filesize as libc::c_ulong) <
           (::std::mem::size_of::<lmp_t>() as
                libc::c_ulong).wrapping_add(pixels as libc::c_ulong) {
        return false_0
    }
    if Image_ValidSize(name) as u64 == 0 { return false_0 }
    if image.hint as libc::c_uint != IL_HINT_Q1 as libc::c_int as libc::c_uint
           &&
           filesize >
               (::std::mem::size_of::<lmp_t>() as libc::c_ulong as libc::c_int
                    + pixels) as libc::c_long {
        let mut numcolors: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < pixels {
            if *fin.offset(i as isize) as libc::c_int == 255 as libc::c_int {
                image.flags |= IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint;
                rendermode = LUMP_MASKED as libc::c_int;
                break ;
            } else { i += 1 }
        }
        pal = fin.offset(pixels as isize);
        numcolors = *(pal as *mut libc::c_short) as libc::c_int;
        if numcolors != 256 as libc::c_int {
            pal = 0 as *mut byte
        } else {
            pal =
                pal.offset(::std::mem::size_of::<libc::c_short>() as
                               libc::c_ulong as isize)
        }
    } else if image.hint as libc::c_uint !=
                  IL_HINT_HL as libc::c_int as libc::c_uint {
        image.flags |= IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint;
        rendermode = LUMP_QUAKE1 as libc::c_int;
        pal = 0 as *mut byte
    } else {
        // unknown mode rejected
        return false_0
    } // 32-bit palete
    Image_GetPaletteLMP(pal, rendermode);
    image.type_0 = PF_INDEXED_32 as libc::c_int as uint;
    image.depth = 1 as libc::c_int as word;
    return Image_AddIndexedImageToPack(fin, image.width as libc::c_int,
                                       image.height as libc::c_int);
}
/*
=============
Image_LoadMIP
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadMIP(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut mip: mip_t =
        mip_t{name: [0; 16], width: 0, height: 0, offsets: [0; 4],};
    let mut hl_texture: qboolean = false_0;
    let mut fin: *mut byte = 0 as *mut byte;
    let mut pal: *mut byte = 0 as *mut byte;
    let mut ofs: [libc::c_int; 4] = [0; 4];
    let mut rendermode: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pixels: libc::c_int = 0;
    let mut numcolors: libc::c_int = 0;
    let mut reflectivity: [libc::c_int; 3] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
    if (filesize as libc::c_ulong) <
           ::std::mem::size_of::<mip_t>() as libc::c_ulong {
        return false_0
    }
    memcpy(&mut mip as *mut mip_t as *mut libc::c_void,
           buffer as *const libc::c_void,
           ::std::mem::size_of::<mip_t>() as libc::c_ulong);
    image.width = mip.width as word;
    image.height = mip.height as word;
    if Image_ValidSize(name) as u64 == 0 { return false_0 }
    memcpy(ofs.as_mut_ptr() as *mut libc::c_void,
           mip.offsets.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong);
    pixels = image.width as libc::c_int * image.height as libc::c_int;
    if image.hint as libc::c_uint != IL_HINT_Q1 as libc::c_int as libc::c_uint
           &&
           filesize as libc::c_ulong >=
               ((::std::mem::size_of::<mip_t>() as libc::c_ulong as
                     libc::c_int +
                     (pixels * 85 as libc::c_int >> 6 as libc::c_int)) as
                    libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_short>()
                                                    as
                                                    libc::c_ulong).wrapping_add(768
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
       {
        // half-life 1.0.0.1 mip version with palette
        fin =
            (buffer as
                 *mut byte).offset(mip.offsets[0 as libc::c_int as usize] as
                                       isize); // skip colorsize
        pal =
            (buffer as
                 *mut byte).offset(mip.offsets[0 as libc::c_int as usize] as
                                       isize).offset((image.width as
                                                          libc::c_int *
                                                          image.height as
                                                              libc::c_int *
                                                          85 as libc::c_int >>
                                                          6 as libc::c_int) as
                                                         isize); // corrupted mip ?
        numcolors = *(pal as *mut libc::c_short) as libc::c_int;
        if numcolors != 256 as libc::c_int {
            pal = 0 as *mut byte
        } else {
            pal =
                pal.offset(::std::mem::size_of::<libc::c_short>() as
                               libc::c_ulong as isize)
        }
        hl_texture = true_0;
        // setup rendermode
        if !Q_strrchr(name, '{' as i32 as libc::c_char).is_null() {
            // NOTE: decals with 'blue base' can be interpret as colored decals
            if Image_CheckFlag(IL_LOAD_DECAL as libc::c_int) as u64 == 0 ||
                   *pal.offset(765 as libc::c_int as isize) as libc::c_int ==
                       0 as libc::c_int &&
                       *pal.offset(766 as libc::c_int as isize) as libc::c_int
                           == 0 as libc::c_int &&
                       *pal.offset(767 as libc::c_int as isize) as libc::c_int
                           == 255 as libc::c_int {
                image.flags =
                    image.flags |
                        IMAGE_ONEBIT_ALPHA as libc::c_int as libc::c_uint;
                rendermode = LUMP_MASKED as libc::c_int
            } else {
                // classic gradient decals
                image.flags =
                    image.flags |
                        IMAGE_COLORINDEX as libc::c_int as libc::c_uint;
                rendermode = LUMP_GRADIENT as libc::c_int
            }
            image.flags =
                image.flags | IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
        } else {
            let mut pal_type: libc::c_int = 0;
            // NOTE: we can have luma-pixels if quake1 texture
			// converted into the hl texture but palette leave unchanged
			// this is a good reason for using fullbright pixels
            pal_type = Image_ComparePalette(pal);
            // check for luma pixels (but ignore liquid textures because they have no lightmap)
            if mip.name[0 as libc::c_int as usize] as libc::c_int !=
                   '*' as i32 &&
                   mip.name[0 as libc::c_int as usize] as libc::c_int !=
                       '!' as i32 && pal_type == PAL_QUAKE1 as libc::c_int {
                i = 0 as libc::c_int;
                while i <
                          image.width as libc::c_int *
                              image.height as libc::c_int {
                    if *fin.offset(i as isize) as libc::c_int >
                           224 as libc::c_int {
                        image.flags |=
                            IMAGE_HAS_LUMA as libc::c_int as libc::c_uint;
                        break ;
                    } else { i += 1 }
                }
            }
            if pal_type == PAL_QUAKE1 as libc::c_int {
                image.flags =
                    image.flags |
                        IMAGE_QUAKEPAL as libc::c_int as libc::c_uint
            }
            rendermode = LUMP_NORMAL as libc::c_int
        }
        Image_GetPaletteLMP(pal, rendermode);
        let ref mut fresh0 =
            *image.d_currentpal.offset(255 as libc::c_int as isize);
        *fresh0 &= 0xffffff as libc::c_int as libc::c_uint
    } else if image.hint as libc::c_uint !=
                  IL_HINT_HL as libc::c_int as libc::c_uint &&
                  filesize >=
                      (::std::mem::size_of::<mip_t>() as libc::c_ulong as
                           libc::c_int +
                           (pixels * 85 as libc::c_int >> 6 as libc::c_int))
                          as libc::c_long {
        // quake1 1.01 mip version without palette
        fin =
            (buffer as
                 *mut byte).offset(mip.offsets[0 as libc::c_int as usize] as
                                       isize); // clear palette
        pal = 0 as *mut byte;
        rendermode = LUMP_NORMAL as libc::c_int;
        hl_texture = false_0;
        // check for luma and alpha pixels
        if image.custom_palette as u64 == 0 {
            i = 0 as libc::c_int;
            while i < image.width as libc::c_int * image.height as libc::c_int
                  {
                if *fin.offset(i as isize) as libc::c_int > 224 as libc::c_int
                       &&
                       *fin.offset(i as isize) as libc::c_int !=
                           255 as libc::c_int {
                    // don't apply luma to water surfaces because they have no lightmap
                    if mip.name[0 as libc::c_int as usize] as libc::c_int !=
                           '*' as i32 &&
                           mip.name[0 as libc::c_int as usize] as libc::c_int
                               != '!' as i32 {
                        image.flags |=
                            IMAGE_HAS_LUMA as libc::c_int as libc::c_uint
                    }
                    break ;
                } else { i += 1 }
            }
        }
        // Arcane Dimensions has the transparent textures
        if !Q_strrchr(name, '{' as i32 as libc::c_char).is_null() {
            i = 0 as libc::c_int;
            while i < image.width as libc::c_int * image.height as libc::c_int
                  {
                if *fin.offset(i as isize) as libc::c_int ==
                       255 as libc::c_int {
                    // don't set ONEBIT_ALPHA flag for some reasons
                    image.flags |=
                        IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint;
                    break ;
                } else { i += 1 }
            }
        }
        image.flags =
            image.flags | IMAGE_QUAKEPAL as libc::c_int as libc::c_uint;
        Image_GetPaletteQ1();
    } else {
        return false_0
        // unknown or unsupported mode rejected
    }
    // check for quake-sky texture
    if Q_strncmp(mip.name.as_mut_ptr(),
                 b"sky\x00" as *const u8 as *const libc::c_char,
                 3 as libc::c_int) == 0 &&
           image.width as libc::c_int ==
               image.height as libc::c_int * 2 as libc::c_int {
        // g-cont: we need to run additional checks for palette type and colors ?
        image.flags |= IMAGE_QUAKESKY as libc::c_int as libc::c_uint
    }
    // check for half-life water texture
    if hl_texture as libc::c_uint != 0 &&
           (mip.name[0 as libc::c_int as usize] as libc::c_int == '!' as i32
                ||
                Q_strnicmp(mip.name.as_mut_ptr(),
                           b"water\x00" as *const u8 as *const libc::c_char,
                           5 as libc::c_int) == 0) {
        // grab the fog color
        image.fogParams[0 as libc::c_int as usize] =
            *pal.offset((3 as libc::c_int * 3 as libc::c_int +
                             0 as libc::c_int) as isize);
        image.fogParams[1 as libc::c_int as usize] =
            *pal.offset((3 as libc::c_int * 3 as libc::c_int +
                             1 as libc::c_int) as isize);
        image.fogParams[2 as libc::c_int as usize] =
            *pal.offset((3 as libc::c_int * 3 as libc::c_int +
                             2 as libc::c_int) as isize);
        // grab the fog density
        image.fogParams[3 as libc::c_int as usize] =
            *pal.offset((4 as libc::c_int * 3 as libc::c_int +
                             0 as libc::c_int) as isize)
    } else if hl_texture as libc::c_uint != 0 &&
                  rendermode == LUMP_GRADIENT as libc::c_int {
        // grab the decal color
        image.fogParams[0 as libc::c_int as usize] =
            *pal.offset((255 as libc::c_int * 3 as libc::c_int +
                             0 as libc::c_int) as isize);
        image.fogParams[1 as libc::c_int as usize] =
            *pal.offset((255 as libc::c_int * 3 as libc::c_int +
                             1 as libc::c_int) as isize);
        image.fogParams[2 as libc::c_int as usize] =
            *pal.offset((255 as libc::c_int * 3 as libc::c_int +
                             2 as libc::c_int) as isize);
        // calc the decal reflectivity
        image.fogParams[3 as libc::c_int as usize] =
            ((image.fogParams[0 as libc::c_int as usize] as libc::c_int +
                  image.fogParams[1 as libc::c_int as usize] as libc::c_int +
                  image.fogParams[2 as libc::c_int as usize] as libc::c_int) /
                 3 as libc::c_int) as byte
    } else if !pal.is_null() {
        // calc texture reflectivity
        i = 0 as libc::c_int; // 32-bit palete
        while i < 256 as libc::c_int {
            reflectivity[0 as libc::c_int as usize] +=
                *pal.offset((i * 3 as libc::c_int + 0 as libc::c_int) as
                                isize) as libc::c_int;
            reflectivity[1 as libc::c_int as usize] +=
                *pal.offset((i * 3 as libc::c_int + 1 as libc::c_int) as
                                isize) as libc::c_int;
            reflectivity[2 as libc::c_int as usize] +=
                *pal.offset((i * 3 as libc::c_int + 2 as libc::c_int) as
                                isize) as libc::c_int;
            i += 1
        }
        image.fogParams[0 as libc::c_int as usize] =
            (reflectivity[0 as libc::c_int as usize] as libc::c_float *
                 (1.0f32 / 256 as libc::c_int as libc::c_float)) as byte;
        image.fogParams[1 as libc::c_int as usize] =
            (reflectivity[1 as libc::c_int as usize] as libc::c_float *
                 (1.0f32 / 256 as libc::c_int as libc::c_float)) as byte;
        image.fogParams[2 as libc::c_int as usize] =
            (reflectivity[2 as libc::c_int as usize] as libc::c_float *
                 (1.0f32 / 256 as libc::c_int as libc::c_float)) as byte
    }
    image.type_0 = PF_INDEXED_32 as libc::c_int as uint;
    image.depth = 1 as libc::c_int as word;
    return Image_AddIndexedImageToPack(fin, image.width as libc::c_int,
                                       image.height as libc::c_int);
}
