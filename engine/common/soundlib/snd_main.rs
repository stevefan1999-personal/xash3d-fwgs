#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type decallist_s;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
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
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
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
pub type C2RustUnnamed = libc::c_uint;
pub const WF_TOTALCOUNT: C2RustUnnamed = 3;
pub const WF_MPGDATA: C2RustUnnamed = 2;
pub const WF_PCMDATA: C2RustUnnamed = 1;
pub const WF_UNKNOWN: C2RustUnnamed = 0;
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
/*
snd_main.c - load & save various sound formats
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
// global sound variables
#[no_mangle]
pub static mut sound: sndlib_t =
    sndlib_t{loadformats: 0 as *const loadwavfmt_t,
             streamformat: 0 as *const streamfmt_t,
             type_0: 0,
             rate: 0,
             width: 0,
             channels: 0,
             loopstart: 0,
             samples: 0,
             flags: 0,
             size: 0,
             wav: 0 as *const byte as *mut byte,
             tempbuffer: 0 as *const byte as *mut byte,
             cmd_flags: 0,};
#[no_mangle]
pub unsafe extern "C" fn Sound_Reset() {
    // reset global variables
    sound.rate = 0 as libc::c_int;
    sound.width = sound.rate;
    sound.loopstart = 0 as libc::c_int;
    sound.channels = sound.loopstart;
    sound.flags = 0 as libc::c_int as uint;
    sound.samples = sound.flags;
    sound.type_0 = WF_UNKNOWN as libc::c_int;
    sound.wav = 0 as *mut byte;
    sound.size = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn SoundPack() -> *mut wavdata_t {
    let mut pack: *mut wavdata_t =
        _Mem_Alloc(host.soundpool,
                   ::std::mem::size_of::<wavdata_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/common/soundlib/snd_main.c\x00" as *const u8 as
                       *const libc::c_char, 35 as libc::c_int) as
            *mut wavdata_t;
    (*pack).buffer = sound.wav;
    (*pack).width = sound.width as byte;
    (*pack).rate = sound.rate as word;
    (*pack).type_0 = sound.type_0 as uint;
    (*pack).size = sound.size;
    (*pack).loopStart = sound.loopstart;
    (*pack).samples = sound.samples as libc::c_int;
    (*pack).channels = sound.channels as byte;
    (*pack).flags = sound.flags;
    return pack;
}
/*
================
FS_LoadSound

loading and unpack to wav any known sound
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_LoadSound(mut filename: *const libc::c_char,
                                      mut buffer: *const byte,
                                      mut size: size_t) -> *mut wavdata_t {
    let mut ext: *const libc::c_char =
        COM_FileExtension(filename); // clear old sounddata
    let mut path: string = [0; 256];
    let mut loadname: string = [0; 256];
    let mut anyformat: qboolean = true_0;
    let mut filesize: fs_offset_t = 0 as libc::c_int as fs_offset_t;
    let mut format: *const loadwavfmt_t = 0 as *const loadwavfmt_t;
    let mut f: *mut byte = 0 as *mut byte;
    Sound_Reset();
    Q_strncpy(loadname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    if Q_strnicmp(ext, b"\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 {
        // we needs to compare file extension with list of supported formats
		// and be sure what is real extension, not a filename with dot
        format = sound.loadformats;
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
        format = sound.loadformats; // release buffer
        while !format.is_null() && !(*format).formatstring.is_null() {
            if anyformat as libc::c_uint != 0 ||
                   Q_strnicmp(ext, (*format).ext, 99999 as libc::c_int) == 0 {
                Q_sprintf(path.as_mut_ptr(), (*format).formatstring,
                          loadname.as_mut_ptr(),
                          b"\x00" as *const u8 as *const libc::c_char,
                          (*format).ext);
                f = FS_LoadFile(path.as_mut_ptr(), &mut filesize, false_0);
                if !f.is_null() && filesize > 0 as libc::c_int as libc::c_long
                   {
                    if (*format).loadfunc.expect("non-null function pointer")(path.as_mut_ptr(),
                                                                              f,
                                                                              filesize)
                           as u64 != 0 {
                        _Mem_Free(f as *mut libc::c_void,
                                  b"../engine/common/soundlib/snd_main.c\x00"
                                      as *const u8 as *const libc::c_char,
                                  99 as libc::c_int);
                        return SoundPack()
                        // loaded
                    } else {
                        _Mem_Free(f as *mut libc::c_void,
                                  b"../engine/common/soundlib/snd_main.c\x00"
                                      as *const u8 as *const libc::c_char,
                                  102 as libc::c_int);
                    }
                    // release buffer
                }
            }
            format = format.offset(1)
        }
    }
    format = sound.loadformats;
    while !format.is_null() && !(*format).formatstring.is_null() {
        if anyformat as libc::c_uint != 0 ||
               Q_strnicmp(ext, (*format).ext, 99999 as libc::c_int) == 0 {
            if !buffer.is_null() && size > 0 as libc::c_int as libc::c_ulong {
                if (*format).loadfunc.expect("non-null function pointer")(loadname.as_mut_ptr(),
                                                                          buffer,
                                                                          size
                                                                              as
                                                                              fs_offset_t)
                       as u64 != 0 {
                    return SoundPack()
                }
                // loaded
            }
        }
        format = format.offset(1)
    }
    if *filename.offset(0 as libc::c_int as isize) as libc::c_int !=
           '#' as i32 {
        Con_DPrintf(b"^3Warning:^7 FS_LoadSound: couldn\'t load \"%s\"\n\x00"
                        as *const u8 as *const libc::c_char,
                    loadname.as_mut_ptr());
    }
    return 0 as *mut wavdata_t;
}
/*
================
Sound_FreeSound

free WAV buffer
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FreeSound(mut pack: *mut wavdata_t) {
    if pack.is_null() { return }
    if !(*pack).buffer.is_null() {
        _Mem_Free((*pack).buffer as *mut libc::c_void,
                  b"../engine/common/soundlib/snd_main.c\x00" as *const u8 as
                      *const libc::c_char, 136 as libc::c_int);
    }
    _Mem_Free(pack as *mut libc::c_void,
              b"../engine/common/soundlib/snd_main.c\x00" as *const u8 as
                  *const libc::c_char, 137 as libc::c_int);
}
/*
================
FS_OpenStream

open and reading basic info from sound stream
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_OpenStream(mut filename: *const libc::c_char)
 -> *mut stream_t {
    let mut ext: *const libc::c_char =
        COM_FileExtension(filename); // clear old streaminfo
    let mut path: string = [0; 256];
    let mut loadname: string = [0; 256];
    let mut anyformat: qboolean = true_0;
    let mut format: *const streamfmt_t = 0 as *const streamfmt_t;
    let mut stream: *mut stream_t = 0 as *mut stream_t;
    Sound_Reset();
    Q_strncpy(loadname.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    if Q_strnicmp(ext, b"\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 {
        // we needs to compare file extension with list of supported formats
		// and be sure what is real extension, not a filename with dot
        format = sound.streamformat;
        while !format.is_null() && !(*format).formatstring.is_null() {
            if Q_strnicmp((*format).ext, ext, 99999 as libc::c_int) == 0 {
                COM_StripExtension(loadname.as_mut_ptr());
                anyformat = false_0;
                break ;
            } else { format = format.offset(1) }
        }
    }
    // now try all the formats in the selected list
    format = sound.streamformat;
    while !format.is_null() && !(*format).formatstring.is_null() {
        if anyformat as libc::c_uint != 0 ||
               Q_strnicmp(ext, (*format).ext, 99999 as libc::c_int) == 0 {
            Q_sprintf(path.as_mut_ptr(), (*format).formatstring,
                      loadname.as_mut_ptr(),
                      b"\x00" as *const u8 as *const libc::c_char,
                      (*format).ext);
            stream =
                (*format).openfunc.expect("non-null function pointer")(path.as_mut_ptr());
            if !stream.is_null() {
                (*stream).format = format;
                return stream
                // done
            }
        }
        format = format.offset(1)
    }
    Con_Reportf(b"FS_OpenStream: couldn\'t open \"%s\"\n\x00" as *const u8 as
                    *const libc::c_char, loadname.as_mut_ptr());
    return 0 as *mut stream_t;
}
/*
================
FS_StreamInfo

get basic stream info
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_StreamInfo(mut stream: *mut stream_t)
 -> *mut wavdata_t {
    static mut info: wavdata_t =
        wavdata_t{rate: 0,
                  width: 0,
                  channels: 0,
                  loopStart: 0,
                  samples: 0,
                  type_0: 0,
                  flags: 0,
                  buffer: 0 as *const byte as *mut byte,
                  size: 0,};
    if stream.is_null() { return 0 as *mut wavdata_t }
    // fill structure
    info.loopStart = -(1 as libc::c_int); // not actual for streams
    info.rate = (*stream).rate as word;
    info.width = (*stream).width as byte;
    info.channels = (*stream).channels as byte;
    info.flags = SOUND_STREAM as libc::c_int as uint;
    info.size = (*stream).size;
    info.buffer = 0 as *mut byte;
    info.samples = 0 as libc::c_int;
    info.type_0 = (*stream).type_0 as uint;
    return &mut info;
}
/*
================
FS_ReadStream

extract stream as wav-data and put into buffer, move file pointer
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_ReadStream(mut stream: *mut stream_t,
                                       mut bytes: libc::c_int,
                                       mut buffer: *mut libc::c_void)
 -> libc::c_int {
    if stream.is_null() || (*stream).format.is_null() ||
           (*(*stream).format).readfunc.is_none() {
        return 0 as libc::c_int
    }
    if bytes <= 0 as libc::c_int || buffer.is_null() {
        return 0 as libc::c_int
    }
    return (*(*stream).format).readfunc.expect("non-null function pointer")(stream,
                                                                            bytes,
                                                                            buffer);
}
/*
================
FS_GetStreamPos

get stream position (in bytes)
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_GetStreamPos(mut stream: *mut stream_t)
 -> libc::c_int {
    if stream.is_null() || (*stream).format.is_null() ||
           (*(*stream).format).getposfunc.is_none() {
        return -(1 as libc::c_int)
    }
    return (*(*stream).format).getposfunc.expect("non-null function pointer")(stream);
}
/*
================
FS_SetStreamPos

set stream position (in bytes)
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_SetStreamPos(mut stream: *mut stream_t,
                                         mut newpos: libc::c_int)
 -> libc::c_int {
    if stream.is_null() || (*stream).format.is_null() ||
           (*(*stream).format).setposfunc.is_none() {
        return -(1 as libc::c_int)
    }
    return (*(*stream).format).setposfunc.expect("non-null function pointer")(stream,
                                                                              newpos);
}
/*
================
FS_FreeStream

close sound stream
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FreeStream(mut stream: *mut stream_t) {
    if stream.is_null() || (*stream).format.is_null() ||
           (*(*stream).format).freefunc.is_none() {
        return
    }
    (*(*stream).format).freefunc.expect("non-null function pointer")(stream);
}
