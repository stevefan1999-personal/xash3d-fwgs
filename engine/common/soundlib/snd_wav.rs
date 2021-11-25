#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    pub type decallist_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
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
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Read(file: *mut file_t, buffer: *mut libc::c_void,
               buffersize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Seek(file: *mut file_t, offset: fs_offset_t, whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Tell(file: *mut file_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Eof(file: *mut file_t) -> qboolean;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut sound: sndlib_t;
    #[no_mangle]
    fn Sound_LoadMPG(name: *const libc::c_char, buffer: *const byte,
                     filesize: fs_offset_t) -> qboolean;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loadwavfmt_s {
    pub formatstring: *const libc::c_char,
    pub ext: *const libc::c_char,
    pub loadfunc: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const byte, _: fs_offset_t)
                             -> qboolean>,
}
pub type loadwavfmt_t = loadwavfmt_s;
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
pub type sndlib_t = sndlib_s;
/*
snd_wav.c - wav format load & save
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
static mut iff_data: *const byte = 0 as *const byte;
static mut iff_dataPtr: *const byte = 0 as *const byte;
static mut iff_end: *const byte = 0 as *const byte;
static mut iff_lastChunk: *const byte = 0 as *const byte;
static mut iff_chunkLen: libc::c_int = 0;
/*
=================
GetLittleShort
=================
*/
unsafe extern "C" fn GetLittleShort() -> libc::c_short {
    let mut val: libc::c_short = 0 as libc::c_int as libc::c_short;
    val =
        (val as libc::c_int +
             ((*iff_dataPtr.offset(0 as libc::c_int as isize) as libc::c_int)
                  << 0 as libc::c_int)) as libc::c_short;
    val =
        (val as libc::c_int +
             ((*iff_dataPtr.offset(1 as libc::c_int as isize) as libc::c_int)
                  << 8 as libc::c_int)) as libc::c_short;
    iff_dataPtr = iff_dataPtr.offset(2 as libc::c_int as isize);
    return val;
}
/*
=================
GetLittleLong
=================
*/
unsafe extern "C" fn GetLittleLong() -> libc::c_int {
    let mut val: libc::c_int = 0 as libc::c_int;
    val +=
        (*iff_dataPtr.offset(0 as libc::c_int as isize) as libc::c_int) <<
            0 as libc::c_int;
    val +=
        (*iff_dataPtr.offset(1 as libc::c_int as isize) as libc::c_int) <<
            8 as libc::c_int;
    val +=
        (*iff_dataPtr.offset(2 as libc::c_int as isize) as libc::c_int) <<
            16 as libc::c_int;
    val +=
        (*iff_dataPtr.offset(3 as libc::c_int as isize) as libc::c_int) <<
            24 as libc::c_int;
    iff_dataPtr = iff_dataPtr.offset(4 as libc::c_int as isize);
    return val;
}
/*
=================
FindNextChunk
=================
*/
unsafe extern "C" fn FindNextChunk(mut name: *const libc::c_char) {
    loop  {
        iff_dataPtr = iff_lastChunk;
        if iff_dataPtr >= iff_end {
            // didn't find the chunk
            iff_dataPtr = 0 as *const byte;
            return
        }
        iff_dataPtr = iff_dataPtr.offset(4 as libc::c_int as isize);
        iff_chunkLen = GetLittleLong();
        if iff_chunkLen < 0 as libc::c_int {
            iff_dataPtr = 0 as *const byte;
            return
        }
        iff_dataPtr = iff_dataPtr.offset(-(8 as libc::c_int as isize));
        iff_lastChunk =
            iff_dataPtr.offset(8 as libc::c_int as
                                   isize).offset((iff_chunkLen +
                                                      1 as libc::c_int &
                                                      !(1 as libc::c_int)) as
                                                     isize);
        if Q_strncmp(iff_dataPtr as *const libc::c_char, name,
                     4 as libc::c_int) == 0 {
            return
        }
    };
}
/*
=================
FindChunk
=================
*/
unsafe extern "C" fn FindChunk(mut name: *const libc::c_char) {
    iff_lastChunk = iff_data;
    FindNextChunk(name);
}
/*
============
StreamFindNextChunk
============
*/
#[no_mangle]
pub unsafe extern "C" fn StreamFindNextChunk(mut file: *mut file_t,
                                             mut name: *const libc::c_char,
                                             mut last_chunk: *mut libc::c_int)
 -> qboolean {
    let mut chunkName: [libc::c_char; 4] = [0; 4]; // didn't find the chunk
    let mut iff_chunk_len: libc::c_int = 0; // didn't find the chunk
    loop  {
        FS_Seek(file, *last_chunk as fs_offset_t, 0 as libc::c_int);
        if FS_Eof(file) as u64 != 0 { return false_0 }
        FS_Seek(file, 4 as libc::c_int as fs_offset_t, 1 as libc::c_int);
        FS_Read(file,
                &mut iff_chunk_len as *mut libc::c_int as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
        if iff_chunk_len < 0 as libc::c_int { return false_0 }
        FS_Seek(file, -(8 as libc::c_int) as fs_offset_t, 1 as libc::c_int);
        *last_chunk =
            (FS_Tell(file) + 8 as libc::c_int as libc::c_long +
                 (iff_chunk_len + 1 as libc::c_int & !(1 as libc::c_int)) as
                     libc::c_long) as libc::c_int;
        FS_Read(file, chunkName.as_mut_ptr() as *mut libc::c_void,
                4 as libc::c_int as size_t);
        if Q_strncmp(chunkName.as_mut_ptr(), name, 4 as libc::c_int) == 0 {
            return true_0
        }
    };
}
/*
=============
Sound_LoadWAV
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Sound_LoadWAV(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut samples: libc::c_int = 0;
    let mut fmt: libc::c_int = 0;
    let mut mpeg_stream: qboolean = false_0;
    if buffer.is_null() || filesize <= 0 as libc::c_int as libc::c_long {
        return false_0
    }
    iff_data = buffer;
    iff_end = buffer.offset(filesize as isize);
    // find "RIFF" chunk
    FindChunk(b"RIFF\x00" as *const u8 as *const libc::c_char);
    if !(!iff_dataPtr.is_null() &&
             Q_strncmp((iff_dataPtr as
                            *const libc::c_char).offset(8 as libc::c_int as
                                                            isize),
                       b"WAVE\x00" as *const u8 as *const libc::c_char,
                       4 as libc::c_int) == 0) {
        Con_DPrintf(b"^1Error:^7 Sound_LoadWAV: %s missing \'RIFF/WAVE\' chunks\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    // get "fmt " chunk
    iff_data = iff_dataPtr.offset(12 as libc::c_int as isize);
    FindChunk(b"fmt \x00" as *const u8 as *const libc::c_char);
    if iff_dataPtr.is_null() {
        Con_DPrintf(b"^1Error:^7 Sound_LoadWAV: %s missing \'fmt \' chunk\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    iff_dataPtr = iff_dataPtr.offset(8 as libc::c_int as isize);
    fmt = GetLittleShort() as libc::c_int;
    if fmt != 1 as libc::c_int {
        if fmt != 85 as libc::c_int {
            Con_DPrintf(b"^1Error:^7 Sound_LoadWAV: %s not a microsoft PCM format\n\x00"
                            as *const u8 as *const libc::c_char, name);
            return false_0
        } else {
            // mpeg stream in wav container
            mpeg_stream = true_0
        }
    } // mp3 always 16bit
    sound.channels = GetLittleShort() as libc::c_int;
    if sound.channels != 1 as libc::c_int &&
           sound.channels != 2 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Sound_LoadWAV: only mono and stereo WAV files supported (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    sound.rate = GetLittleLong();
    iff_dataPtr = iff_dataPtr.offset(6 as libc::c_int as isize);
    sound.width = GetLittleShort() as libc::c_int / 8 as libc::c_int;
    if mpeg_stream as u64 != 0 { sound.width = 2 as libc::c_int }
    if sound.width != 1 as libc::c_int && sound.width != 2 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Sound_LoadWAV: only 8 and 16 bit WAV files supported (%s)\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    // get cue chunk
    FindChunk(b"cue \x00" as *const u8 as
                  *const libc::c_char); // if the next chunk is a LIST chunk, look for a cue length marker
    if !iff_dataPtr.is_null() {
        iff_dataPtr = iff_dataPtr.offset(32 as libc::c_int as isize);
        sound.loopstart = GetLittleLong();
        FindNextChunk(b"LIST\x00" as *const u8 as *const libc::c_char);
        if !iff_dataPtr.is_null() {
            if Q_strncmp((iff_dataPtr as
                              *const libc::c_char).offset(28 as libc::c_int as
                                                              isize),
                         b"mark\x00" as *const u8 as *const libc::c_char,
                         4 as libc::c_int) == 0 {
                // this is not a proper parse, but it works with CoolEdit...
                iff_dataPtr = iff_dataPtr.offset(24 as libc::c_int as isize);
                sound.samples = (sound.loopstart + GetLittleLong()) as uint
                // samples in loop
            }
        }
    } else {
        sound.loopstart = -(1 as libc::c_int);
        sound.samples = 0 as libc::c_int as uint
    }
    // find data chunk
    FindChunk(b"data\x00" as *const u8 as *const libc::c_char);
    if iff_dataPtr.is_null() {
        Con_DPrintf(b"^1Error:^7 Sound_LoadWAV: %s missing \'data\' chunk\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return false_0
    }
    iff_dataPtr = iff_dataPtr.offset(4 as libc::c_int as isize);
    samples = GetLittleLong() / sound.width;
    if sound.samples != 0 {
        if (samples as libc::c_uint) < sound.samples {
            Con_DPrintf(b"^1Error:^7 Sound_LoadWAV: %s has a bad loop length\n\x00"
                            as *const u8 as *const libc::c_char, name);
            return false_0
        }
    } else { sound.samples = samples as uint }
    if sound.samples <= 0 as libc::c_int as libc::c_uint {
        Con_Reportf(b"^1Error:^7 Sound_LoadWAV: file with %i samples (%s)\n\x00"
                        as *const u8 as *const libc::c_char, sound.samples,
                    name);
        return false_0
    }
    sound.type_0 = WF_PCMDATA as libc::c_int;
    sound.samples =
        (sound.samples as
             libc::c_uint).wrapping_div(sound.channels as libc::c_uint) as
            uint as uint;
    // g-cont. get support for mp3 streams packed in wav container
	// e.g. CAd menu sounds
    if mpeg_stream as u64 != 0 {
        let mut hdr_size: libc::c_int =
            iff_dataPtr.wrapping_offset_from(buffer) as libc::c_long as
                libc::c_int;
        if (filesize - hdr_size as libc::c_long) <
               32768 as libc::c_int as libc::c_long {
            sound.tempbuffer =
                _Mem_Realloc(host.soundpool,
                             sound.tempbuffer as *mut libc::c_void,
                             32768 as libc::c_int as size_t, true_0,
                             b"../engine/common/soundlib/snd_wav.c\x00" as
                                 *const u8 as *const libc::c_char,
                             272 as libc::c_int) as *mut byte;
            memcpy(sound.tempbuffer as *mut libc::c_void,
                   buffer.offset(iff_dataPtr.wrapping_offset_from(buffer) as
                                     libc::c_long as isize) as
                       *const libc::c_void,
                   (filesize - hdr_size as libc::c_long) as libc::c_ulong);
            return Sound_LoadMPG(name, sound.tempbuffer,
                                 32768 as libc::c_int as fs_offset_t)
        }
        return Sound_LoadMPG(name, buffer.offset(hdr_size as isize),
                             filesize - hdr_size as libc::c_long)
    }
    // Load the data
    sound.size =
        sound.samples.wrapping_mul(sound.width as
                                       libc::c_uint).wrapping_mul(sound.channels
                                                                      as
                                                                      libc::c_uint)
            as size_t;
    sound.wav =
        _Mem_Alloc(host.soundpool, sound.size, false_0,
                   b"../engine/common/soundlib/snd_wav.c\x00" as *const u8 as
                       *const libc::c_char, 282 as libc::c_int) as *mut byte;
    memcpy(sound.wav as *mut libc::c_void,
           buffer.offset(iff_dataPtr.wrapping_offset_from(buffer) as
                             libc::c_long as isize) as *const libc::c_void,
           sound.size);
    // now convert 8-bit sounds to signed
    if sound.width == 1 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut pData: *mut libc::c_schar = sound.wav as *mut libc::c_schar;
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < sound.samples {
            j = 0 as libc::c_int;
            while j < sound.channels {
                *pData =
                    (*pData as byte as libc::c_int - 128 as libc::c_int) as
                        byte as libc::c_schar;
                pData = pData.offset(1);
                j += 1
            }
            i += 1
        }
    }
    return true_0;
}
/*
=================
Stream_OpenWAV
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_OpenWAV(mut filename: *const libc::c_char)
 -> *mut stream_t {
    let mut stream: *mut stream_t = 0 as *mut stream_t;
    let mut last_chunk: libc::c_int = 0 as libc::c_int;
    let mut chunkName: [libc::c_char; 4] = [0; 4];
    let mut iff_data_0: libc::c_int = 0;
    let mut file: *mut file_t = 0 as *mut file_t;
    let mut t: libc::c_short = 0;
    if filename.is_null() || *filename == 0 { return 0 as *mut stream_t }
    // open
    file =
        FS_Open(filename, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    if file.is_null() { return 0 as *mut stream_t }
    // find "RIFF" chunk
    if StreamFindNextChunk(file,
                           b"RIFF\x00" as *const u8 as *const libc::c_char,
                           &mut last_chunk) as u64 == 0 {
        Con_DPrintf(b"^1Error:^7 Stream_OpenWAV: %s missing RIFF chunk\n\x00"
                        as *const u8 as *const libc::c_char, filename);
        FS_Close(file);
        return 0 as *mut stream_t
    }
    FS_Read(file, chunkName.as_mut_ptr() as *mut libc::c_void,
            4 as libc::c_int as size_t);
    if Q_strncmp(chunkName.as_mut_ptr(),
                 b"WAVE\x00" as *const u8 as *const libc::c_char,
                 4 as libc::c_int) == 0 {
        Con_DPrintf(b"^1Error:^7 Stream_OpenWAV: %s missing WAVE chunk\n\x00"
                        as *const u8 as *const libc::c_char, filename);
        FS_Close(file);
        return 0 as *mut stream_t
    }
    // get "fmt " chunk
    iff_data_0 =
        (FS_Tell(file) + 4 as libc::c_int as libc::c_long) as libc::c_int;
    last_chunk = iff_data_0;
    if StreamFindNextChunk(file,
                           b"fmt \x00" as *const u8 as *const libc::c_char,
                           &mut last_chunk) as u64 == 0 {
        Con_DPrintf(b"^1Error:^7 Stream_OpenWAV: %s missing \'fmt \' chunk\n\x00"
                        as *const u8 as *const libc::c_char, filename);
        FS_Close(file);
        return 0 as *mut stream_t
    }
    FS_Read(file, chunkName.as_mut_ptr() as *mut libc::c_void,
            4 as libc::c_int as size_t);
    FS_Read(file, &mut t as *mut libc::c_short as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong);
    if t as libc::c_int != 1 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Stream_OpenWAV: %s not a microsoft PCM format\n\x00"
                        as *const u8 as *const libc::c_char, filename);
        FS_Close(file);
        return 0 as *mut stream_t
    }
    FS_Read(file, &mut t as *mut libc::c_short as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong);
    sound.channels = t as libc::c_int;
    FS_Read(file, &mut sound.rate as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Seek(file, 6 as libc::c_int as fs_offset_t, 1 as libc::c_int);
    FS_Read(file, &mut t as *mut libc::c_short as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong);
    sound.width = t as libc::c_int / 8 as libc::c_int;
    sound.loopstart = 0 as libc::c_int;
    // find data chunk
    last_chunk = iff_data_0;
    if StreamFindNextChunk(file,
                           b"data\x00" as *const u8 as *const libc::c_char,
                           &mut last_chunk) as u64 == 0 {
        Con_DPrintf(b"^1Error:^7 Stream_OpenWAV: %s missing \'data\' chunk\n\x00"
                        as *const u8 as *const libc::c_char, filename);
        FS_Close(file);
        return 0 as *mut stream_t
    }
    FS_Read(file, &mut sound.samples as *mut uint as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    sound.samples =
        sound.samples.wrapping_div(sound.width as
                                       libc::c_uint).wrapping_div(sound.channels
                                                                      as
                                                                      libc::c_uint);
    // at this point we have valid stream
    stream =
        _Mem_Alloc(host.soundpool,
                   ::std::mem::size_of::<stream_t>() as libc::c_ulong, true_0,
                   b"../engine/common/soundlib/snd_wav.c\x00" as *const u8 as
                       *const libc::c_char, 387 as libc::c_int) as
            *mut stream_t; // header length
    (*stream).file = file;
    (*stream).size =
        sound.samples.wrapping_mul(sound.width as
                                       libc::c_uint).wrapping_mul(sound.channels
                                                                      as
                                                                      libc::c_uint)
            as size_t;
    (*stream).buffsize = FS_Tell(file) as libc::c_int;
    (*stream).channels = sound.channels;
    (*stream).width = sound.width;
    (*stream).rate = sound.rate;
    (*stream).type_0 = WF_PCMDATA as libc::c_int;
    return stream;
}
/*
=================
Stream_ReadWAV

assume stream is valid
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_ReadWAV(mut stream: *mut stream_t,
                                        mut bytes: libc::c_int,
                                        mut buffer: *mut libc::c_void)
 -> libc::c_int {
    let mut remaining: libc::c_int = 0; // invalid file
    if (*stream).file.is_null() { return 0 as libc::c_int }
    remaining = (*stream).size.wrapping_sub((*stream).pos) as libc::c_int;
    if remaining <= 0 as libc::c_int { return 0 as libc::c_int }
    if bytes > remaining { bytes = remaining }
    (*stream).pos =
        ((*stream).pos as libc::c_ulong).wrapping_add(bytes as libc::c_ulong)
            as size_t as size_t;
    FS_Read((*stream).file, buffer, bytes as size_t);
    return bytes;
}
/*
=================
Stream_SetPosWAV

assume stream is valid
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_SetPosWAV(mut stream: *mut stream_t,
                                          mut newpos: libc::c_int)
 -> libc::c_int {
    // NOTE: stream->pos it's real file position without header size
    if FS_Seek((*stream).file, ((*stream).buffsize + newpos) as fs_offset_t,
               0 as libc::c_int) != -(1 as libc::c_int) {
        (*stream).pos = newpos as size_t;
        return true_0 as libc::c_int
    }
    return false_0 as libc::c_int;
}
/*
=================
Stream_GetPosWAV

assume stream is valid
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_GetPosWAV(mut stream: *mut stream_t)
 -> libc::c_int {
    return (*stream).pos as libc::c_int;
}
/*
=================
Stream_FreeWAV

assume stream is valid
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_FreeWAV(mut stream: *mut stream_t) {
    if !(*stream).file.is_null() { FS_Close((*stream).file); }
    _Mem_Free(stream as *mut libc::c_void,
              b"../engine/common/soundlib/snd_wav.c\x00" as *const u8 as
                  *const libc::c_char, 464 as libc::c_int);
}
