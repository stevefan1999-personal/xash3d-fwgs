#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    pub type decallist_s;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn Image_ValidSize(name: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Image_CheckFlag(bit: libc::c_int) -> qboolean;
    #[no_mangle]
    fn Image_GetPaletteBMP(pal: *const byte);
    #[no_mangle]
    static mut image: imglib_t;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
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
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Write(file: *mut file_t, data: *const libc::c_void,
                datasize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
}
pub type __int8_t = libc::c_schar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type file_t = file_s;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct bmp_t {
    pub id: [int8_t; 2],
    pub fileSize: uint32_t,
    pub reserved0: uint32_t,
    pub bitmapDataOffset: uint32_t,
    pub bitmapHeaderSize: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub planes: uint16_t,
    pub bitsPerPixel: uint16_t,
    pub compression: uint32_t,
    pub bitmapDataSize: uint32_t,
    pub hRes: uint32_t,
    pub vRes: uint32_t,
    pub colors: uint32_t,
    pub importantColors: uint32_t,
}
/*
img_bmp.c - bmp format load & save
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
Image_LoadBMP
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Image_LoadBMP(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut buf_p: *mut byte = 0 as *mut byte;
    let mut pixbuf: *mut byte = 0 as *mut byte;
    let mut palette: [rgba_t; 256] = [[0; 4]; 256];
    let mut i: libc::c_int = 0;
    let mut columns: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut rows: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut bpp: libc::c_int = 1 as libc::c_int;
    let mut cbPalBytes: libc::c_int = 0 as libc::c_int;
    let mut padSize: libc::c_int = 0 as libc::c_int;
    let mut bps: libc::c_int = 0 as libc::c_int;
    let mut reflectivity: [libc::c_int; 3] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
    let mut load_qfont: qboolean = false_0;
    let mut bhdr: bmp_t =
        bmp_t{id: [0; 2],
              fileSize: 0,
              reserved0: 0,
              bitmapDataOffset: 0,
              bitmapHeaderSize: 0,
              width: 0,
              height: 0,
              planes: 0,
              bitsPerPixel: 0,
              compression: 0,
              bitmapDataSize: 0,
              hRes: 0,
              vRes: 0,
              colors: 0,
              importantColors: 0,};
    let mut estimatedSize: fs_offset_t = 0;
    if (filesize as libc::c_ulong) <
           ::std::mem::size_of::<bmp_t>() as libc::c_ulong {
        Con_Reportf(b"^1Error:^7 Image_LoadBMP: %s have incorrect file size %li should be greater than %li (header)\n\x00"
                        as *const u8 as *const libc::c_char, name, filesize,
                    ::std::mem::size_of::<bmp_t>() as libc::c_ulong);
        return false_0
    }
    buf_p = buffer as *mut byte;
    memcpy(&mut bhdr as *mut bmp_t as *mut libc::c_void,
           buf_p as *const libc::c_void,
           ::std::mem::size_of::<bmp_t>() as libc::c_ulong);
    buf_p =
        buf_p.offset(::std::mem::size_of::<bmp_t>() as libc::c_ulong as
                         isize);
    // bogus file header check
    if bhdr.reserved0 != 0 as libc::c_int as libc::c_uint { return false_0 }
    if bhdr.planes as libc::c_int != 1 as libc::c_int { return false_0 }
    if memcmp(bhdr.id.as_mut_ptr() as *const libc::c_void,
              b"BM\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void, 2 as libc::c_int as libc::c_ulong) != 0
       {
        Con_DPrintf(b"^1Error:^7 Image_LoadBMP: only Windows-style BMP files supported (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    if bhdr.bitmapHeaderSize != 0x28 as libc::c_int as libc::c_uint {
        Con_DPrintf(b"^1Error:^7 Image_LoadBMP: invalid header size %i\n\x00"
                        as *const u8 as *const libc::c_char,
                    bhdr.bitmapHeaderSize);
        return false_0
    }
    // bogus info header check
    if bhdr.fileSize as libc::c_long != filesize {
        // Sweet Half-Life issues. splash.bmp have bogus filesize
        Con_Reportf(b"^3Warning:^7 Image_LoadBMP: %s have incorrect file size %li should be %i\n\x00"
                        as *const u8 as *const libc::c_char, name, filesize,
                    bhdr.fileSize);
    }
    // bogus compression?  Only non-compressed supported.
    if bhdr.compression != 0 as libc::c_int as libc::c_uint {
        Con_DPrintf(b"^1Error:^7 Image_LoadBMP: only uncompressed BMP files supported (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    columns = bhdr.width;
    image.width = columns as word;
    rows = abs(bhdr.height);
    image.height = rows as word;
    if Image_ValidSize(name) as u64 == 0 { return false_0 }
    // special case for loading qfont (menu font)
    if Q_strncmp(name,
                 b"#XASH_SYSTEMFONT_001\x00" as *const u8 as
                     *const libc::c_char, 20 as libc::c_int) == 0 {
        // NOTE: same as system font we can use 4-bit bmps only
		// step1: move main layer into alpha-channel (give grayscale from RED channel)
		// step2: fill main layer with 255 255 255 color (white)
		// step3: ????
		// step4: PROFIT!!! (economy up to 150 kb for menu.dll final size)
        image.flags |= IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint;
        load_qfont = true_0
    }
    if bhdr.bitsPerPixel as libc::c_int <= 8 as libc::c_int {
        // figure out how many entries are actually in the table
        if bhdr.colors == 0 as libc::c_int as libc::c_uint {
            bhdr.colors = 256 as libc::c_int as uint32_t;
            cbPalBytes =
                (((1 as libc::c_int) << bhdr.bitsPerPixel as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<rgba_t>()
                                                     as libc::c_ulong) as
                    libc::c_int
        } else {
            cbPalBytes =
                (bhdr.colors as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<rgba_t>()
                                                     as libc::c_ulong) as
                    libc::c_int
        }
    }
    estimatedSize =
        buf_p.wrapping_offset_from(buffer) as libc::c_long +
            cbPalBytes as libc::c_long;
    if filesize < estimatedSize {
        Con_Reportf(b"^1Error:^7 Image_LoadBMP: %s have incorrect file size %li should be greater than %li (palette)\n\x00"
                        as *const u8 as *const libc::c_char, name, filesize,
                    estimatedSize);
        return false_0
    }
    memcpy(palette.as_mut_ptr() as *mut libc::c_void,
           buf_p as *const libc::c_void, cbPalBytes as libc::c_ulong);
    // setup gradient alpha for player decal
    if Q_strncmp(name, b"#logo\x00" as *const u8 as *const libc::c_char,
                 5 as libc::c_int) == 0 {
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < bhdr.colors {
            palette[i as usize][3 as libc::c_int as usize] = i as byte;
            i += 1
        }
        image.flags |= IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
    }
    if Image_CheckFlag(IL_OVERVIEW as libc::c_int) as libc::c_uint != 0 &&
           bhdr.bitsPerPixel as libc::c_int == 8 as libc::c_int {
        // convert green background into alpha-layer, make opacity for all other entries
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < bhdr.colors {
            if palette[i as usize][0 as libc::c_int as usize] as libc::c_int
                   == 0 as libc::c_int &&
                   palette[i as usize][1 as libc::c_int as usize] as
                       libc::c_int == 255 as libc::c_int &&
                   palette[i as usize][2 as libc::c_int as usize] as
                       libc::c_int == 0 as libc::c_int {
                palette[i as usize][3 as libc::c_int as usize] =
                    0 as libc::c_int as byte;
                palette[i as usize][2 as libc::c_int as usize] =
                    palette[i as usize][3 as libc::c_int as usize];
                palette[i as usize][1 as libc::c_int as usize] =
                    palette[i as usize][2 as libc::c_int as usize];
                palette[i as usize][0 as libc::c_int as usize] =
                    palette[i as usize][1 as libc::c_int as usize];
                image.flags |= IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
            } else {
                palette[i as usize][3 as libc::c_int as usize] =
                    255 as libc::c_int as byte
            }
            i += 1
        }
    }
    if Image_CheckFlag(IL_KEEP_8BIT as libc::c_int) as libc::c_uint != 0 &&
           bhdr.bitsPerPixel as libc::c_int == 8 as libc::c_int {
        image.palette =
            _Mem_Alloc(host.imagepool, 1024 as libc::c_int as size_t, false_0,
                       b"../engine/common/imagelib/img_bmp.c\x00" as *const u8
                           as *const libc::c_char, 138 as libc::c_int) as
                *mut byte;
        pixbuf = image.palette;
        // 32 bit palette
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < bhdr.colors {
            let fresh0 = pixbuf;
            pixbuf = pixbuf.offset(1);
            *fresh0 = palette[i as usize][2 as libc::c_int as usize];
            let fresh1 = pixbuf;
            pixbuf = pixbuf.offset(1);
            *fresh1 = palette[i as usize][1 as libc::c_int as usize];
            let fresh2 = pixbuf;
            pixbuf = pixbuf.offset(1);
            *fresh2 = palette[i as usize][0 as libc::c_int as usize];
            let fresh3 = pixbuf;
            pixbuf = pixbuf.offset(1);
            *fresh3 = palette[i as usize][3 as libc::c_int as usize];
            i += 1
        }
        image.type_0 = PF_INDEXED_32 as libc::c_int as uint
    } else {
        image.palette = 0 as *mut byte;
        image.type_0 = PF_RGBA_32 as libc::c_int as uint;
        bpp = 4 as libc::c_int
    }
    buf_p = buf_p.offset(cbPalBytes as isize);
    bps =
        image.width as libc::c_int *
            (bhdr.bitsPerPixel as libc::c_int >> 3 as libc::c_int);
    match bhdr.bitsPerPixel as libc::c_int {
        1 => {
            padSize =
                (32 as libc::c_int - bhdr.width % 32 as libc::c_int) /
                    8 as libc::c_int % 4 as libc::c_int
        }
        4 => {
            padSize =
                (8 as libc::c_int - bhdr.width % 8 as libc::c_int) /
                    2 as libc::c_int % 4 as libc::c_int
        }
        16 => {
            padSize =
                (4 as libc::c_int -
                     image.width as libc::c_int * 2 as libc::c_int %
                         4 as libc::c_int) % 4 as libc::c_int
        }
        8 | 24 => {
            padSize =
                (4 as libc::c_int - bps % 4 as libc::c_int) % 4 as libc::c_int
        }
        _ => { }
    }
    estimatedSize =
        buf_p.wrapping_offset_from(buffer) as libc::c_long +
            ((image.width as libc::c_int + padSize) *
                 image.height as libc::c_int *
                 (bhdr.bitsPerPixel as libc::c_int >> 3 as libc::c_int)) as
                libc::c_long;
    if filesize < estimatedSize {
        if !image.palette.is_null() {
            _Mem_Free(image.palette as *mut libc::c_void,
                      b"../engine/common/imagelib/img_bmp.c\x00" as *const u8
                          as *const libc::c_char, 182 as libc::c_int);
            image.palette = 0 as *mut byte
        }
        Con_Reportf(b"^1Error:^7 Image_LoadBMP: %s have incorrect file size %li should be greater than %li (pixels)\n\x00"
                        as *const u8 as *const libc::c_char, name, filesize,
                    estimatedSize);
        return false_0
    }
    image.size =
        (image.width as libc::c_int * image.height as libc::c_int * bpp) as
            size_t;
    image.rgba =
        _Mem_Alloc(host.imagepool, image.size, false_0,
                   b"../engine/common/imagelib/img_bmp.c\x00" as *const u8 as
                       *const libc::c_char, 191 as libc::c_int) as *mut byte;
    row = rows - 1 as libc::c_int;
    while row >= 0 as libc::c_int {
        pixbuf = image.rgba.offset((row * columns * bpp) as isize);
        column = 0 as libc::c_int;
        while column < columns {
            let mut red: byte = 0;
            let mut green: byte = 0;
            let mut blue: byte = 0;
            let mut alpha: byte = 0;
            let mut shortPixel: word = 0;
            let mut c: libc::c_int = 0;
            let mut k: libc::c_int = 0;
            let mut palIndex: libc::c_int = 0;
            match bhdr.bitsPerPixel as libc::c_int {
                1 => {
                    let fresh4 = buf_p;
                    buf_p = buf_p.offset(1);
                    alpha = *fresh4;
                    // bmp have a reversed palette colors
                    // actual only for 4-bit bmps
                    column -= 1; // ingnore main iterations
                    c = 0 as libc::c_int; // already existed
                    k = 128 as libc::c_int;
                    while c < 8 as libc::c_int {
                        blue =
                            if (alpha as libc::c_int & k != 0) as libc::c_int
                                   == 1 as libc::c_int {
                                0xff as libc::c_int
                            } else { 0 as libc::c_int } as byte;
                        green = blue;
                        red = green;
                        let fresh5 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh5 = red;
                        let fresh6 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh6 = green;
                        let fresh7 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh7 = blue;
                        let fresh8 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh8 = 0 as libc::c_int as byte;
                        column += 1;
                        if column == columns { break ; }
                        c += 1;
                        k >>= 1 as libc::c_int
                    }
                }
                4 => {
                    let fresh9 = buf_p;
                    buf_p = buf_p.offset(1);
                    alpha = *fresh9;
                    palIndex = alpha as libc::c_int >> 4 as libc::c_int;
                    if load_qfont as u64 != 0 {
                        red = 255 as libc::c_int as byte;
                        let fresh10 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh10 = red;
                        green = 255 as libc::c_int as byte;
                        let fresh11 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh11 = green;
                        blue = 255 as libc::c_int as byte;
                        let fresh12 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh12 = blue;
                        let fresh13 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh13 =
                            palette[palIndex as
                                        usize][2 as libc::c_int as usize]
                    } else {
                        red =
                            palette[palIndex as
                                        usize][2 as libc::c_int as usize];
                        let fresh14 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh14 = red;
                        green =
                            palette[palIndex as
                                        usize][1 as libc::c_int as usize];
                        let fresh15 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh15 = green;
                        blue =
                            palette[palIndex as
                                        usize][0 as libc::c_int as usize];
                        let fresh16 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh16 = blue;
                        let fresh17 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh17 =
                            palette[palIndex as
                                        usize][3 as libc::c_int as usize]
                    }
                    column += 1;
                    if !(column == columns) {
                        palIndex = alpha as libc::c_int & 0xf as libc::c_int;
                        if load_qfont as u64 != 0 {
                            red = 255 as libc::c_int as byte;
                            let fresh18 = pixbuf;
                            pixbuf = pixbuf.offset(1);
                            *fresh18 = red;
                            green = 255 as libc::c_int as byte;
                            let fresh19 = pixbuf;
                            pixbuf = pixbuf.offset(1);
                            *fresh19 = green;
                            blue = 255 as libc::c_int as byte;
                            let fresh20 = pixbuf;
                            pixbuf = pixbuf.offset(1);
                            *fresh20 = blue;
                            let fresh21 = pixbuf;
                            pixbuf = pixbuf.offset(1);
                            *fresh21 =
                                palette[palIndex as
                                            usize][2 as libc::c_int as usize]
                        } else {
                            red =
                                palette[palIndex as
                                            usize][2 as libc::c_int as usize];
                            let fresh22 = pixbuf;
                            pixbuf = pixbuf.offset(1);
                            *fresh22 = red;
                            green =
                                palette[palIndex as
                                            usize][1 as libc::c_int as usize];
                            let fresh23 = pixbuf;
                            pixbuf = pixbuf.offset(1);
                            *fresh23 = green;
                            blue =
                                palette[palIndex as
                                            usize][0 as libc::c_int as usize];
                            let fresh24 = pixbuf;
                            pixbuf = pixbuf.offset(1);
                            *fresh24 = blue;
                            let fresh25 = pixbuf;
                            pixbuf = pixbuf.offset(1);
                            *fresh25 =
                                palette[palIndex as
                                            usize][3 as libc::c_int as usize]
                        }
                    }
                }
                8 => {
                    let fresh26 = buf_p;
                    buf_p = buf_p.offset(1);
                    palIndex = *fresh26 as libc::c_int;
                    red =
                        palette[palIndex as usize][2 as libc::c_int as usize];
                    green =
                        palette[palIndex as usize][1 as libc::c_int as usize];
                    blue =
                        palette[palIndex as usize][0 as libc::c_int as usize];
                    alpha =
                        palette[palIndex as usize][3 as libc::c_int as usize];
                    if Image_CheckFlag(IL_KEEP_8BIT as libc::c_int) as u64 !=
                           0 {
                        let fresh27 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh27 = palIndex as byte
                    } else {
                        let fresh28 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh28 = red;
                        let fresh29 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh29 = green;
                        let fresh30 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh30 = blue;
                        let fresh31 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh31 = alpha
                    }
                }
                16 => {
                    shortPixel = *(buf_p as *mut word);
                    buf_p = buf_p.offset(2 as libc::c_int as isize);
                    blue =
                        ((shortPixel as libc::c_int &
                              (31 as libc::c_int) << 10 as libc::c_int) >>
                             7 as libc::c_int) as byte;
                    let fresh32 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh32 = blue;
                    green =
                        ((shortPixel as libc::c_int &
                              (31 as libc::c_int) << 5 as libc::c_int) >>
                             2 as libc::c_int) as byte;
                    let fresh33 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh33 = green;
                    red =
                        ((shortPixel as libc::c_int & 31 as libc::c_int) <<
                             3 as libc::c_int) as byte;
                    let fresh34 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh34 = red;
                    let fresh35 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh35 = 0xff as libc::c_int as byte
                }
                24 => {
                    let fresh36 = buf_p;
                    buf_p = buf_p.offset(1);
                    blue = *fresh36;
                    let fresh37 = buf_p;
                    buf_p = buf_p.offset(1);
                    green = *fresh37;
                    let fresh38 = buf_p;
                    buf_p = buf_p.offset(1);
                    red = *fresh38;
                    let fresh39 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh39 = red;
                    let fresh40 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh40 = green;
                    let fresh41 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh41 = blue;
                    let fresh42 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh42 = 0xff as libc::c_int as byte
                }
                32 => {
                    let fresh43 = buf_p;
                    buf_p = buf_p.offset(1);
                    blue = *fresh43;
                    let fresh44 = buf_p;
                    buf_p = buf_p.offset(1);
                    green = *fresh44;
                    let fresh45 = buf_p;
                    buf_p = buf_p.offset(1);
                    red = *fresh45;
                    let fresh46 = buf_p;
                    buf_p = buf_p.offset(1);
                    alpha = *fresh46;
                    let fresh47 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh47 = red;
                    let fresh48 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh48 = green;
                    let fresh49 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh49 = blue;
                    let fresh50 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh50 = alpha;
                    if alpha as libc::c_int != 255 as libc::c_int {
                        image.flags |=
                            IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
                    }
                }
                _ => {
                    _Mem_Free(image.palette as *mut libc::c_void,
                              b"../engine/common/imagelib/img_bmp.c\x00" as
                                  *const u8 as *const libc::c_char,
                              300 as libc::c_int);
                    _Mem_Free(image.rgba as *mut libc::c_void,
                              b"../engine/common/imagelib/img_bmp.c\x00" as
                                  *const u8 as *const libc::c_char,
                              301 as libc::c_int);
                    return false_0
                }
            }
            if red as libc::c_int != green as libc::c_int ||
                   green as libc::c_int != blue as libc::c_int {
                image.flags |= IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
            }
            reflectivity[0 as libc::c_int as usize] += red as libc::c_int;
            reflectivity[1 as libc::c_int as usize] += green as libc::c_int;
            reflectivity[2 as libc::c_int as usize] += blue as libc::c_int;
            column += 1
        }
        buf_p = buf_p.offset(padSize as isize);
        row -= 1
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
    if !image.palette.is_null() { Image_GetPaletteBMP(image.palette); }
    image.depth = 1 as libc::c_int as word;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Image_SaveBMP(mut name: *const libc::c_char,
                                       mut pix: *mut rgbdata_t) -> qboolean {
    let mut pfile: *mut file_t = 0 as *mut file_t;
    let mut total_size: size_t = 0;
    let mut cur_size: size_t = 0;
    let mut rgrgbPalette: [rgba_t; 256] = [[0; 4]; 256];
    let mut cbBmpBits: dword = 0;
    let mut clipbuf: *mut byte = 0 as *mut byte;
    let mut pb: *mut byte = 0 as *mut byte;
    let mut pbBmpBits: *mut byte = 0 as *mut byte;
    let mut cbPalBytes: dword = 0;
    let mut biTrueWidth: dword = 0;
    let mut pixel_size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut hdr: bmp_t =
        bmp_t{id: [0; 2],
              fileSize: 0,
              reserved0: 0,
              bitmapDataOffset: 0,
              bitmapHeaderSize: 0,
              width: 0,
              height: 0,
              planes: 0,
              bitsPerPixel: 0,
              compression: 0,
              bitmapDataSize: 0,
              hRes: 0,
              vRes: 0,
              colors: 0,
              importantColors: 0,};
    if FS_FileExists(name, false_0 as libc::c_int) != 0 &&
           Image_CheckFlag(IL_ALLOW_OVERWRITE as libc::c_int) as u64 == 0 {
        return false_0
    }
    // bogus parameter check
    if (*pix).buffer.is_null() { return false_0 }
    // get image description
    match (*pix).type_0 {
        1 | 2 => { pixel_size = 1 as libc::c_int }
        5 => { pixel_size = 3 as libc::c_int }
        3 => { pixel_size = 4 as libc::c_int }
        _ => { return false_0 }
    }
    pfile =
        FS_Open(name, b"wb\x00" as *const u8 as *const libc::c_char, false_0);
    if pfile.is_null() { return false_0 }
    // NOTE: align transparency column will sucessfully removed
	// after create sprite or lump image, it's just standard requiriments
    biTrueWidth =
        ((*pix).width as libc::c_int + 3 as libc::c_int & !(3 as libc::c_int))
            as dword;
    cbBmpBits =
        biTrueWidth.wrapping_mul((*pix).height as
                                     libc::c_uint).wrapping_mul(pixel_size as
                                                                    libc::c_uint);
    cbPalBytes =
        if pixel_size == 1 as libc::c_int {
            (256 as libc::c_int as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<rgba_t>()
                                                 as libc::c_ulong)
        } else { 0 as libc::c_int as libc::c_ulong } as dword;
    // Bogus file header check
    hdr.id[0 as libc::c_int as usize] = 'B' as i32 as int8_t;
    hdr.id[1 as libc::c_int as usize] = 'M' as i32 as int8_t;
    hdr.fileSize =
        (::std::mem::size_of::<bmp_t>() as
             libc::c_ulong).wrapping_add(cbBmpBits as
                                             libc::c_ulong).wrapping_add(cbPalBytes
                                                                             as
                                                                             libc::c_ulong)
            as uint32_t;
    hdr.reserved0 = 0 as libc::c_int as uint32_t;
    hdr.bitmapDataOffset =
        (::std::mem::size_of::<bmp_t>() as
             libc::c_ulong).wrapping_add(cbPalBytes as libc::c_ulong) as
            uint32_t;
    hdr.bitmapHeaderSize = 40 as libc::c_int as uint32_t;
    hdr.width = biTrueWidth as int32_t;
    hdr.height = (*pix).height as int32_t;
    hdr.planes = 1 as libc::c_int as uint16_t;
    hdr.bitsPerPixel = (pixel_size * 8 as libc::c_int) as uint16_t;
    hdr.compression = 0 as libc::c_int as uint32_t;
    hdr.bitmapDataSize = cbBmpBits;
    hdr.hRes = 0 as libc::c_int as uint32_t;
    hdr.vRes = 0 as libc::c_int as uint32_t;
    hdr.colors =
        if pixel_size == 1 as libc::c_int {
            256 as libc::c_int
        } else { 0 as libc::c_int } as uint32_t;
    hdr.importantColors = 0 as libc::c_int as uint32_t;
    FS_Write(pfile, &mut hdr as *mut bmp_t as *const libc::c_void,
             ::std::mem::size_of::<bmp_t>() as libc::c_ulong);
    pbBmpBits =
        _Mem_Alloc(host.imagepool, cbBmpBits as size_t, false_0,
                   b"../engine/common/imagelib/img_bmp.c\x00" as *const u8 as
                       *const libc::c_char, 389 as libc::c_int) as *mut byte;
    if pixel_size == 1 as libc::c_int {
        pb = (*pix).palette;
        // copy over used entries
        i = 0 as libc::c_int;
        while i < hdr.colors as libc::c_int {
            let fresh51 = pb;
            pb = pb.offset(1);
            rgrgbPalette[i as usize][2 as libc::c_int as usize] = *fresh51;
            let fresh52 = pb;
            pb = pb.offset(1);
            rgrgbPalette[i as usize][1 as libc::c_int as usize] = *fresh52;
            let fresh53 = pb;
            pb = pb.offset(1);
            rgrgbPalette[i as usize][0 as libc::c_int as usize] = *fresh53;
            // bmp feature - can store 32-bit palette if present
			// some viewers e.g. fimg.exe can show alpha-chanell for it
            if (*pix).type_0 == PF_INDEXED_32 as libc::c_int as libc::c_uint {
                let fresh54 = pb;
                pb = pb.offset(1);
                rgrgbPalette[i as usize][3 as libc::c_int as usize] = *fresh54
            } else {
                rgrgbPalette[i as usize][3 as libc::c_int as usize] =
                    0 as libc::c_int as byte
            }
            i += 1
        }
        // write palette
        FS_Write(pfile, rgrgbPalette.as_mut_ptr() as *const libc::c_void,
                 cbPalBytes as size_t);
    }
    pb = (*pix).buffer;
    y = 0 as libc::c_int;
    while y < hdr.height {
        i = (hdr.height - 1 as libc::c_int - y) * hdr.width;
        x = 0 as libc::c_int;
        while x < (*pix).width as libc::c_int {
            if pixel_size == 1 as libc::c_int {
                // 8-bit
                *pbBmpBits.offset(i as isize) = *pb.offset(x as isize)
            } else {
                // 24 bit
                *pbBmpBits.offset((i * pixel_size + 0 as libc::c_int) as
                                      isize) =
                    *pb.offset((x * pixel_size + 2 as libc::c_int) as isize);
                *pbBmpBits.offset((i * pixel_size + 1 as libc::c_int) as
                                      isize) =
                    *pb.offset((x * pixel_size + 1 as libc::c_int) as isize);
                *pbBmpBits.offset((i * pixel_size + 2 as libc::c_int) as
                                      isize) =
                    *pb.offset((x * pixel_size + 0 as libc::c_int) as isize)
            }
            if pixel_size == 4 as libc::c_int {
                // write alpha channel
                *pbBmpBits.offset((i * pixel_size + 3 as libc::c_int) as
                                      isize) =
                    *pb.offset((x * pixel_size + 3 as libc::c_int) as isize)
            }
            i += 1;
            x += 1
        }
        pb = pb.offset(((*pix).width as libc::c_int * pixel_size) as isize);
        y += 1
    }
    // write bitmap bits (remainder of file)
    FS_Write(pfile, pbBmpBits as *const libc::c_void, cbBmpBits as size_t);
    FS_Close(pfile);
    _Mem_Free(pbBmpBits as *mut libc::c_void,
              b"../engine/common/imagelib/img_bmp.c\x00" as *const u8 as
                  *const libc::c_char, 446 as libc::c_int);
    return true_0;
}
