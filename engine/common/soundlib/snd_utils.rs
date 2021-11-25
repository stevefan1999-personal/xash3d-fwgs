#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type file_s;
    pub type decallist_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Sound_LoadWAV(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Sound_LoadMPG(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn Stream_OpenMPG(filename: *const libc::c_char) -> *mut stream_t;
    #[no_mangle]
    fn Stream_ReadMPG(stream: *mut stream_t, bytes: libc::c_int,
                      buffer: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn Stream_SetPosMPG(stream: *mut stream_t, newpos: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Stream_GetPosMPG(stream: *mut stream_t) -> libc::c_int;
    #[no_mangle]
    fn Stream_FreeMPG(stream: *mut stream_t);
    #[no_mangle]
    fn Stream_OpenWAV(filename: *const libc::c_char) -> *mut stream_t;
    #[no_mangle]
    fn Stream_ReadWAV(stream: *mut stream_t, bytes: libc::c_int,
                      buffer: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn Stream_SetPosWAV(stream: *mut stream_t, newpos: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Stream_GetPosWAV(stream: *mut stream_t) -> libc::c_int;
    #[no_mangle]
    fn Stream_FreeWAV(stream: *mut stream_t);
    #[no_mangle]
    static mut sound: sndlib_t;
    #[no_mangle]
    fn FS_FileLength(f: *mut file_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn FS_Read(file: *mut file_t, buffer: *mut libc::c_void,
               buffersize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn _Mem_Check(filename: *const libc::c_char, fileline: libc::c_int);
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn _Mem_FreePool(poolptr: *mut poolhandle_t,
                     filename: *const libc::c_char, fileline: libc::c_int);
    #[no_mangle]
    fn _Mem_AllocPool(name: *const libc::c_char,
                      filename: *const libc::c_char, fileline: libc::c_int)
     -> poolhandle_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Realloc(poolptr: poolhandle_t, memptr: *mut libc::c_void,
                    size: size_t, clear: qboolean,
                    filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut host: host_parm_t;
}
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const HOST_DEDICATED: C2RustUnnamed = 1;
pub const HOST_NORMAL: C2RustUnnamed = 0;
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
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type file_t = file_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_s {
    pub format: *const streamfmt_t,
    pub file: *mut file_t,
    pub width: libc::c_int,
    pub rate: libc::c_int,
    pub channels: libc::c_int,
    pub type_0: libc::c_int,
    pub size: size_t,
    pub ptr: *mut libc::c_void,
    pub temp: [libc::c_char; 8192],
    pub pos: size_t,
    pub buffsize: libc::c_int,
}
pub type streamfmt_t = streamfmt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamfmt_s {
    pub formatstring: *const libc::c_char,
    pub ext: *const libc::c_char,
    pub openfunc: Option<unsafe extern "C" fn(_: *const libc::c_char)
                             -> *mut stream_t>,
    pub readfunc: Option<unsafe extern "C" fn(_: *mut stream_t,
                                              _: libc::c_int,
                                              _: *mut libc::c_void)
                             -> libc::c_int>,
    pub setposfunc: Option<unsafe extern "C" fn(_: *mut stream_t,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub getposfunc: Option<unsafe extern "C" fn(_: *mut stream_t)
                               -> libc::c_int>,
    pub freefunc: Option<unsafe extern "C" fn(_: *mut stream_t) -> ()>,
}
pub type stream_t = stream_s;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SOUND_CONVERT16BIT: C2RustUnnamed_0 = 8192;
pub const SOUND_RESAMPLE: C2RustUnnamed_0 = 4096;
pub const SOUND_STREAM: C2RustUnnamed_0 = 2;
pub const SOUND_LOOPED: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavdata_t {
    pub rate: word,
    pub width: byte,
    pub channels: byte,
    pub loopStart: libc::c_int,
    pub samples: libc::c_int,
    pub type_0: uint,
    pub flags: uint,
    pub buffer: *mut byte,
    pub size: size_t,
}
pub type sndlib_t = sndlib_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sndlib_s {
    pub loadformats: *const loadwavfmt_t,
    pub streamformat: *const streamfmt_t,
    pub type_0: libc::c_int,
    pub rate: libc::c_int,
    pub width: libc::c_int,
    pub channels: libc::c_int,
    pub loopstart: libc::c_int,
    pub samples: uint,
    pub flags: uint,
    pub size: size_t,
    pub wav: *mut byte,
    pub tempbuffer: *mut byte,
    pub cmd_flags: libc::c_int,
}
pub type loadwavfmt_t = loadwavfmt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loadwavfmt_s {
    pub formatstring: *const libc::c_char,
    pub ext: *const libc::c_char,
    pub loadfunc: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const byte, _: fs_offset_t)
                             -> qboolean>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavehdr_t {
    pub riff_id: int32_t,
    pub rLen: int32_t,
    pub wave_id: int32_t,
    pub fmt_id: int32_t,
    pub pcm_header_len: int32_t,
    pub wFormatTag: int16_t,
    pub nChannels: int16_t,
    pub nSamplesPerSec: int32_t,
    pub nAvgBytesPerSec: int32_t,
    pub nBlockAlign: int16_t,
    pub nBitsPerSample: int16_t,
}
/*
snd_utils.c - sound common tools
Copyright (C) 2010 Uncle Mike

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
=============================================================================

	XASH3D LOAD SOUND FORMATS

=============================================================================
*/
// stub
static mut load_null: [loadwavfmt_t; 1] =
    [{
         let mut init =
             loadwavfmt_s{formatstring: 0 as *const libc::c_char,
                          ext: 0 as *const libc::c_char,
                          loadfunc: None,};
         init
     }];
static mut load_game: [loadwavfmt_t; 5] =
    unsafe {
        [{
             let mut init =
                 loadwavfmt_s{formatstring:
                                  b"sound/%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"wav\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Sound_LoadWAV as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),};
             init
         },
         {
             let mut init =
                 loadwavfmt_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"wav\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Sound_LoadWAV as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),};
             init
         },
         {
             let mut init =
                 loadwavfmt_s{formatstring:
                                  b"sound/%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"mp3\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Sound_LoadMPG as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),};
             init
         },
         {
             let mut init =
                 loadwavfmt_s{formatstring:
                                  b"%s%s.%s\x00" as *const u8 as
                                      *const libc::c_char,
                              ext:
                                  b"mp3\x00" as *const u8 as
                                      *const libc::c_char,
                              loadfunc:
                                  Some(Sound_LoadMPG as
                                           unsafe extern "C" fn(_:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const byte,
                                                                _:
                                                                    fs_offset_t)
                                               -> qboolean),};
             init
         },
         {
             let mut init =
                 loadwavfmt_s{formatstring: 0 as *const libc::c_char,
                              ext: 0 as *const libc::c_char,
                              loadfunc: None,};
             init
         }]
    };
/*
=============================================================================

	XASH3D PROCESS STREAM FORMATS

=============================================================================
*/
// stub
static mut stream_null: [streamfmt_t; 1] =
    [{
         let mut init =
             streamfmt_s{formatstring: 0 as *const libc::c_char,
                         ext: 0 as *const libc::c_char,
                         openfunc: None,
                         readfunc: None,
                         setposfunc: None,
                         getposfunc: None,
                         freefunc: None,};
         init
     }];
static mut stream_game: [streamfmt_t; 3] =
    unsafe {
        [{
             let mut init =
                 streamfmt_s{formatstring:
                                 b"%s%s.%s\x00" as *const u8 as
                                     *const libc::c_char,
                             ext:
                                 b"mp3\x00" as *const u8 as
                                     *const libc::c_char,
                             openfunc:
                                 Some(Stream_OpenMPG as
                                          unsafe extern "C" fn(_:
                                                                   *const libc::c_char)
                                              -> *mut stream_t),
                             readfunc:
                                 Some(Stream_ReadMPG as
                                          unsafe extern "C" fn(_:
                                                                   *mut stream_t,
                                                               _: libc::c_int,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> libc::c_int),
                             setposfunc:
                                 Some(Stream_SetPosMPG as
                                          unsafe extern "C" fn(_:
                                                                   *mut stream_t,
                                                               _: libc::c_int)
                                              -> libc::c_int),
                             getposfunc:
                                 Some(Stream_GetPosMPG as
                                          unsafe extern "C" fn(_:
                                                                   *mut stream_t)
                                              -> libc::c_int),
                             freefunc:
                                 Some(Stream_FreeMPG as
                                          unsafe extern "C" fn(_:
                                                                   *mut stream_t)
                                              -> ()),};
             init
         },
         {
             let mut init =
                 streamfmt_s{formatstring:
                                 b"%s%s.%s\x00" as *const u8 as
                                     *const libc::c_char,
                             ext:
                                 b"wav\x00" as *const u8 as
                                     *const libc::c_char,
                             openfunc:
                                 Some(Stream_OpenWAV as
                                          unsafe extern "C" fn(_:
                                                                   *const libc::c_char)
                                              -> *mut stream_t),
                             readfunc:
                                 Some(Stream_ReadWAV as
                                          unsafe extern "C" fn(_:
                                                                   *mut stream_t,
                                                               _: libc::c_int,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> libc::c_int),
                             setposfunc:
                                 Some(Stream_SetPosWAV as
                                          unsafe extern "C" fn(_:
                                                                   *mut stream_t,
                                                               _: libc::c_int)
                                              -> libc::c_int),
                             getposfunc:
                                 Some(Stream_GetPosWAV as
                                          unsafe extern "C" fn(_:
                                                                   *mut stream_t)
                                              -> libc::c_int),
                             freefunc:
                                 Some(Stream_FreeWAV as
                                          unsafe extern "C" fn(_:
                                                                   *mut stream_t)
                                              -> ()),};
             init
         },
         {
             let mut init =
                 streamfmt_s{formatstring: 0 as *const libc::c_char,
                             ext: 0 as *const libc::c_char,
                             openfunc: None,
                             readfunc: None,
                             setposfunc: None,
                             getposfunc: None,
                             freefunc: None,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn Sound_Init() {
    // init pools
    host.soundpool =
        _Mem_AllocPool(b"SoundLib Pool\x00" as *const u8 as
                           *const libc::c_char,
                       b"../engine/common/soundlib/snd_utils.c\x00" as
                           *const u8 as *const libc::c_char,
                       63 as libc::c_int);
    // install image formats (can be re-install later by Sound_Setup)
    match host.type_0 {
        0 => {
            sound.loadformats = load_game.as_ptr();
            sound.streamformat = stream_game.as_ptr()
        }
        _ => {
            // all other instances not using soundlib or will be reinstalling later
            sound.loadformats = load_null.as_ptr(); // check for leaks
            sound.streamformat = stream_null.as_ptr()
        }
    } // magic number from GoldSrc, seems to be header size
    sound.tempbuffer = 0 as *mut byte;
}
#[no_mangle]
pub unsafe extern "C" fn Sound_Shutdown() {
    _Mem_Check(b"../engine/common/soundlib/snd_utils.c\x00" as *const u8 as
                   *const libc::c_char, 82 as libc::c_int);
    _Mem_FreePool(&mut host.soundpool,
                  b"../engine/common/soundlib/snd_utils.c\x00" as *const u8 as
                      *const libc::c_char, 83 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Sound_Copy(mut size: size_t) -> *mut byte {
    let mut out: *mut byte = 0 as *mut byte;
    out =
        _Mem_Alloc(host.soundpool, size, false_0,
                   b"../engine/common/soundlib/snd_utils.c\x00" as *const u8
                       as *const libc::c_char, 90 as libc::c_int) as
            *mut byte;
    memcpy(out as *mut libc::c_void, sound.tempbuffer as *const libc::c_void,
           size);
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn Sound_GetApproxWavePlayLen(mut filepath:
                                                        *const libc::c_char)
 -> uint {
    let mut f: *mut file_t = 0 as *mut file_t;
    let mut wav: wavehdr_t =
        wavehdr_t{riff_id: 0,
                  rLen: 0,
                  wave_id: 0,
                  fmt_id: 0,
                  pcm_header_len: 0,
                  wFormatTag: 0,
                  nChannels: 0,
                  nSamplesPerSec: 0,
                  nAvgBytesPerSec: 0,
                  nBlockAlign: 0,
                  nBitsPerSample: 0,};
    let mut filesize: size_t = 0;
    let mut msecs: uint = 0;
    f =
        FS_Open(filepath, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    if f.is_null() { return 0 as libc::c_int as uint }
    if FS_Read(f, &mut wav as *mut wavehdr_t as *mut libc::c_void,
               ::std::mem::size_of::<wavehdr_t>() as libc::c_ulong) as
           libc::c_ulong !=
           ::std::mem::size_of::<wavehdr_t>() as libc::c_ulong {
        FS_Close(f);
        return 0 as libc::c_int as uint
    }
    filesize = FS_FileLength(f) as size_t;
    filesize =
        (filesize as
             libc::c_ulong).wrapping_sub(128 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    FS_Close(f);
    // is real wav file ?
    if wav.riff_id !=
           (('F' as i32) << 24 as libc::c_int) +
               (('F' as i32) << 16 as libc::c_int) +
               (('I' as i32) << 8 as libc::c_int) + 'R' as i32 ||
           wav.wave_id !=
               (('E' as i32) << 24 as libc::c_int) +
                   (('V' as i32) << 16 as libc::c_int) +
                   (('A' as i32) << 8 as libc::c_int) + 'W' as i32 ||
           wav.fmt_id !=
               ((' ' as i32) << 24 as libc::c_int) +
                   (('t' as i32) << 16 as libc::c_int) +
                   (('m' as i32) << 8 as libc::c_int) + 'f' as i32 {
        return 0 as libc::c_int as uint
    }
    if wav.nAvgBytesPerSec >= 1000 as libc::c_int {
        msecs =
            (filesize as libc::c_float /
                 (wav.nAvgBytesPerSec as libc::c_float / 1000.0f32)) as uint
    } else {
        msecs =
            (filesize as libc::c_float / wav.nAvgBytesPerSec as libc::c_float
                 * 1000.0f32) as uint
    }
    return msecs;
}
/*
================
Sound_ConvertToSigned

Convert unsigned data to signed
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sound_ConvertToSigned(mut data: *const byte,
                                               mut channels: libc::c_int,
                                               mut samples: libc::c_int) {
    let mut i: libc::c_int = 0;
    if channels == 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < samples {
            *(sound.tempbuffer as
                  *mut libc::c_schar).offset((i * 2 as libc::c_int +
                                                  0 as libc::c_int) as isize)
                =
                (*data.offset((i * 2 as libc::c_int + 0 as libc::c_int) as
                                  isize) as libc::c_int - 128 as libc::c_int)
                    as libc::c_schar;
            *(sound.tempbuffer as
                  *mut libc::c_schar).offset((i * 2 as libc::c_int +
                                                  1 as libc::c_int) as isize)
                =
                (*data.offset((i * 2 as libc::c_int + 1 as libc::c_int) as
                                  isize) as libc::c_int - 128 as libc::c_int)
                    as libc::c_schar;
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < samples {
            *(sound.tempbuffer as *mut libc::c_schar).offset(i as isize) =
                (*data.offset(i as isize) as libc::c_int - 128 as libc::c_int)
                    as libc::c_schar;
            i += 1
        }
    };
}
/*
================
Sound_ResampleInternal

We need convert sound to signed even if nothing to resample
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sound_ResampleInternal(mut sc: *mut wavdata_t,
                                                mut inrate: libc::c_int,
                                                mut inwidth: libc::c_int,
                                                mut outrate: libc::c_int,
                                                mut outwidth: libc::c_int)
 -> qboolean {
    let mut stepscale: libc::c_float = 0.; // this is usually 0.5, 1, or 2
    let mut outcount: libc::c_int = 0;
    let mut srcsample: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut sample: libc::c_int = 0;
    let mut sample2: libc::c_int = 0;
    let mut samplefrac: libc::c_int = 0;
    let mut fracstep: libc::c_int = 0;
    let mut data: *mut byte = 0 as *mut byte;
    data = (*sc).buffer;
    stepscale = inrate as libc::c_float / outrate as libc::c_float;
    outcount = ((*sc).samples as libc::c_float / stepscale) as libc::c_int;
    (*sc).size =
        (outcount * outwidth * (*sc).channels as libc::c_int) as size_t;
    sound.tempbuffer =
        _Mem_Realloc(host.soundpool, sound.tempbuffer as *mut libc::c_void,
                     (*sc).size, true_0,
                     b"../engine/common/soundlib/snd_utils.c\x00" as *const u8
                         as *const libc::c_char, 174 as libc::c_int) as
            *mut byte;
    (*sc).samples = outcount;
    if (*sc).loopStart != -(1 as libc::c_int) {
        (*sc).loopStart =
            ((*sc).loopStart as libc::c_float / stepscale) as libc::c_int
    }
    // resample / decimate to the current source rate
    if stepscale == 1.0f32 && inwidth == 1 as libc::c_int &&
           outwidth == 1 as libc::c_int {
        Sound_ConvertToSigned(data, (*sc).channels as libc::c_int, outcount);
    } else {
        // general case
        samplefrac = 0 as libc::c_int;
        fracstep =
            (stepscale * 256 as libc::c_int as libc::c_float) as libc::c_int;
        if (*sc).channels as libc::c_int == 2 as libc::c_int {
            i = 0 as libc::c_int;
            while i < outcount {
                srcsample = samplefrac >> 8 as libc::c_int;
                samplefrac += fracstep;
                if inwidth == 2 as libc::c_int {
                    sample =
                        *(data as
                              *mut libc::c_short).offset((srcsample *
                                                              2 as libc::c_int
                                                              +
                                                              0 as
                                                                  libc::c_int)
                                                             as isize) as
                            libc::c_int;
                    sample2 =
                        *(data as
                              *mut libc::c_short).offset((srcsample *
                                                              2 as libc::c_int
                                                              +
                                                              1 as
                                                                  libc::c_int)
                                                             as isize) as
                            libc::c_int
                } else {
                    sample =
                        (*data.offset((srcsample * 2 as libc::c_int +
                                           0 as libc::c_int) as isize) as
                             libc::c_char as libc::c_int) << 8 as libc::c_int;
                    sample2 =
                        (*data.offset((srcsample * 2 as libc::c_int +
                                           1 as libc::c_int) as isize) as
                             libc::c_char as libc::c_int) << 8 as libc::c_int
                }
                if outwidth == 2 as libc::c_int {
                    *(sound.tempbuffer as
                          *mut libc::c_short).offset((i * 2 as libc::c_int +
                                                          0 as libc::c_int) as
                                                         isize) =
                        sample as libc::c_short;
                    *(sound.tempbuffer as
                          *mut libc::c_short).offset((i * 2 as libc::c_int +
                                                          1 as libc::c_int) as
                                                         isize) =
                        sample2 as libc::c_short
                } else {
                    *(sound.tempbuffer as
                          *mut libc::c_schar).offset((i * 2 as libc::c_int +
                                                          0 as libc::c_int) as
                                                         isize) =
                        (sample >> 8 as libc::c_int) as libc::c_schar;
                    *(sound.tempbuffer as
                          *mut libc::c_schar).offset((i * 2 as libc::c_int +
                                                          1 as libc::c_int) as
                                                         isize) =
                        (sample2 >> 8 as libc::c_int) as libc::c_schar
                }
                i += 1
            }
        } else {
            i = 0 as libc::c_int;
            while i < outcount {
                srcsample = samplefrac >> 8 as libc::c_int;
                samplefrac += fracstep;
                if inwidth == 2 as libc::c_int {
                    sample =
                        *(data as
                              *mut libc::c_short).offset(srcsample as isize)
                            as libc::c_int
                } else {
                    sample =
                        (*data.offset(srcsample as isize) as libc::c_char as
                             libc::c_int) << 8 as libc::c_int
                }
                if outwidth == 2 as libc::c_int {
                    *(sound.tempbuffer as
                          *mut libc::c_short).offset(i as isize) =
                        sample as libc::c_short
                } else {
                    *(sound.tempbuffer as
                          *mut libc::c_schar).offset(i as isize) =
                        (sample >> 8 as libc::c_int) as libc::c_schar
                }
                i += 1
            }
        }
        Con_Reportf(b"Sound_Resample: from[%d bit %d kHz] to [%d bit %d kHz]\n\x00"
                        as *const u8 as *const libc::c_char,
                    inwidth * 8 as libc::c_int, inrate,
                    outwidth * 8 as libc::c_int, outrate);
    }
    (*sc).rate = outrate as word;
    (*sc).width = outwidth as byte;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Sound_Process(mut wav: *mut *mut wavdata_t,
                                       mut rate: libc::c_int,
                                       mut width: libc::c_int,
                                       mut flags: uint) -> qboolean {
    let mut snd: *mut wavdata_t = *wav;
    let mut result: qboolean = true_0;
    // check for buffers
    if snd.is_null() || (*snd).buffer.is_null() {
        return false_0
    } // free original image buffer
    if flags & SOUND_RESAMPLE as libc::c_int as libc::c_uint != 0 &&
           (width > 0 as libc::c_int || rate > 0 as libc::c_int) {
        if Sound_ResampleInternal(snd, (*snd).rate as libc::c_int,
                                  (*snd).width as libc::c_int, rate, width) as
               u64 != 0 {
            _Mem_Free((*snd).buffer as *mut libc::c_void,
                      b"../engine/common/soundlib/snd_utils.c\x00" as
                          *const u8 as *const libc::c_char,
                      258 as libc::c_int);
            (*snd).buffer = Sound_Copy((*snd).size)
            // unzone buffer (don't touch image.tempbuffer)
        } else {
            // not resampled
            result = false_0
        }
    }
    *wav = snd;
    return false_0;
}
