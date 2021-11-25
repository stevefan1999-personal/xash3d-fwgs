#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type decallist_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    fn FS_Read(file: *mut file_t, buffer: *mut libc::c_void,
               buffersize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Seek(file: *mut file_t, offset: fs_offset_t, whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut sound: sndlib_t;
    #[no_mangle]
    fn create_decoder(error: *mut libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn feed_mpeg_header(mpg: *mut libc::c_void, data: *const byte,
                        bufsize: libc::c_long, streamsize: libc::c_long,
                        sc: *mut wavinfo_t) -> libc::c_int;
    #[no_mangle]
    fn feed_mpeg_stream(mpg: *mut libc::c_void, data: *const byte,
                        bufsize: libc::c_long, outbuf: *mut byte,
                        outsize: *mut size_t) -> libc::c_int;
    #[no_mangle]
    fn open_mpeg_stream(mpg: *mut libc::c_void, file: *mut libc::c_void,
                        f_read: pfread, f_seek: pfseek, sc: *mut wavinfo_t)
     -> libc::c_int;
    #[no_mangle]
    fn read_mpeg_stream(mpg: *mut libc::c_void, outbuf: *mut byte,
                        outsize: *mut size_t) -> libc::c_int;
    #[no_mangle]
    fn get_stream_pos(mpg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn set_stream_pos(mpg: *mut libc::c_void, curpos: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn close_decoder(mpg: *mut libc::c_void);
    #[no_mangle]
    fn get_error(mpeg: *mut libc::c_void) -> *const libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavinfo_t {
    pub rate: libc::c_int,
    pub channels: libc::c_int,
    pub playtime: libc::c_int,
}
pub type pfseek
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_long,
                                _: libc::c_int) -> libc::c_long>;
pub type pfread
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: size_t) -> libc::c_long>;
/*
snd_mp3.c - mp3 format loading and streaming
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
=================================================================

	MPEG decompression

=================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Sound_LoadMPG(mut name: *const libc::c_char,
                                       mut buffer: *const byte,
                                       mut filesize: fs_offset_t)
 -> qboolean {
    let mut mpeg: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut bytesWrite: size_t = 0 as libc::c_int as size_t;
    let mut out: [byte; 8192] = [0; 8192];
    let mut outsize: size_t = 0;
    let mut padsize: size_t = 0;
    let mut ret: libc::c_int = 0;
    let mut sc: wavinfo_t = wavinfo_t{rate: 0, channels: 0, playtime: 0,};
    // load the file
    if buffer.is_null() || filesize < 32768 as libc::c_int as libc::c_long {
        return false_0
    }
    // couldn't create decoder
    mpeg = create_decoder(&mut ret);
    if mpeg.is_null() { return false_0 }
    if ret != 0 {
        Con_DPrintf(b"^1Error:^7 %s\n\x00" as *const u8 as
                        *const libc::c_char, get_error(mpeg));
    }
    // trying to read header
    if feed_mpeg_header(mpeg, buffer, 32768 as libc::c_int as libc::c_long,
                        filesize, &mut sc) == 0 {
        Con_DPrintf(b"^1Error:^7 Sound_LoadMPG: failed to load (%s): %s\n\x00"
                        as *const u8 as *const libc::c_char, name,
                    get_error(mpeg)); // always 16-bit PCM
        close_decoder(mpeg); // in bytes
        return false_0
    } // evaluate pos
    sound.channels = sc.channels;
    sound.rate = sc.rate;
    sound.width = 2 as libc::c_int;
    sound.loopstart = -(1 as libc::c_int);
    sound.size =
        (sound.channels * sound.rate * sound.width *
             (sc.playtime / 1000 as libc::c_int)) as size_t;
    padsize = sound.size.wrapping_rem(32768 as libc::c_int as libc::c_ulong);
    pos =
        (pos as
             libc::c_ulong).wrapping_add(32768 as libc::c_int as
                                             libc::c_ulong) as size_t as
            size_t;
    if sound.size == 0 {
        // bad mpeg file ?
        Con_DPrintf(b"^1Error:^7 Sound_LoadMPG: (%s) is probably corrupted\n\x00"
                        as *const u8 as *const libc::c_char, name);
        close_decoder(mpeg);
        return false_0
    }
    // add sentinel make sure we not overrun
    sound.wav =
        _Mem_Alloc(host.soundpool, sound.size.wrapping_add(padsize), true_0,
                   b"../engine/common/soundlib/snd_mp3.c\x00" as *const u8 as
                       *const libc::c_char, 71 as libc::c_int) as *mut byte;
    sound.type_0 = WF_PCMDATA as libc::c_int;
    // decompress mpg into pcm wav format
    while bytesWrite < sound.size {
        let mut size: libc::c_int = 0;
        if feed_mpeg_stream(mpeg, 0 as *const byte,
                            0 as libc::c_int as libc::c_long,
                            out.as_mut_ptr(), &mut outsize) !=
               0 as libc::c_int &&
               outsize <= 0 as libc::c_int as libc::c_ulong {
            let mut data: *const byte = buffer.offset(pos as isize);
            let mut bufsize: libc::c_int = 0;
            // there was end of the stream
            // if there are no bytes remainig so we can decompress the new frame
            if pos.wrapping_add(32768 as libc::c_int as libc::c_ulong) >
                   filesize as libc::c_ulong {
                bufsize =
                    (filesize as libc::c_ulong).wrapping_sub(pos) as
                        libc::c_int
            } else { bufsize = 32768 as libc::c_int }
            pos =
                (pos as libc::c_ulong).wrapping_add(bufsize as libc::c_ulong)
                    as size_t as size_t;
            if feed_mpeg_stream(mpeg, data, bufsize as libc::c_long,
                                out.as_mut_ptr(), &mut outsize) !=
                   0 as libc::c_int {
                break ;
            }
        }
        if bytesWrite.wrapping_add(outsize) > sound.size {
            size = sound.size.wrapping_sub(bytesWrite) as libc::c_int
        } else { size = outsize as libc::c_int }
        memcpy(&mut *sound.wav.offset(bytesWrite as isize) as *mut byte as
                   *mut libc::c_void, out.as_mut_ptr() as *const libc::c_void,
               size as libc::c_ulong);
        bytesWrite =
            (bytesWrite as libc::c_ulong).wrapping_add(size as libc::c_ulong)
                as size_t as size_t
    }
    sound.samples =
        bytesWrite.wrapping_div((sound.width * sound.channels) as
                                    libc::c_ulong) as uint;
    close_decoder(mpeg);
    return true_0;
}
/*
=================
Stream_OpenMPG
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_OpenMPG(mut filename: *const libc::c_char)
 -> *mut stream_t {
    let mut stream: *mut stream_t = 0 as *mut stream_t;
    let mut mpeg: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut file: *mut file_t = 0 as *mut file_t;
    let mut ret: libc::c_int = 0;
    let mut sc: wavinfo_t = wavinfo_t{rate: 0, channels: 0, playtime: 0,};
    file =
        FS_Open(filename, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    if file.is_null() { return 0 as *mut stream_t }
    // at this point we have valid stream
    stream =
        _Mem_Alloc(host.soundpool,
                   ::std::mem::size_of::<stream_t>() as libc::c_ulong, true_0,
                   b"../engine/common/soundlib/snd_mp3.c\x00" as *const u8 as
                       *const libc::c_char, 125 as libc::c_int) as
            *mut stream_t;
    (*stream).file = file;
    (*stream).pos = 0 as libc::c_int as size_t;
    // couldn't create decoder
    mpeg = create_decoder(&mut ret);
    if mpeg.is_null() {
        Con_DPrintf(b"^1Error:^7 Stream_OpenMPG: couldn\'t create decoder: %s\n\x00"
                        as *const u8 as *const libc::c_char, get_error(mpeg));
        _Mem_Free(stream as *mut libc::c_void,
                  b"../engine/common/soundlib/snd_mp3.c\x00" as *const u8 as
                      *const libc::c_char, 133 as libc::c_int);
        FS_Close(file);
        return 0 as *mut stream_t
    }
    if ret != 0 {
        Con_DPrintf(b"^1Error:^7 %s\n\x00" as *const u8 as
                        *const libc::c_char, get_error(mpeg));
    }
    // trying to open stream and read header
    if open_mpeg_stream(mpeg, file as *mut libc::c_void,
                        ::std::mem::transmute::<*mut libc::c_void,
                                                pfread>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                *mut file_t,
                                                                                                            _:
                                                                                                                *mut libc::c_void,
                                                                                                            _:
                                                                                                                size_t)
                                                                                           ->
                                                                                               fs_offset_t>,
                                                                                *mut libc::c_void>(Some(FS_Read
                                                                                                            as
                                                                                                            unsafe extern "C" fn(_:
                                                                                                                                     *mut file_t,
                                                                                                                                 _:
                                                                                                                                     *mut libc::c_void,
                                                                                                                                 _:
                                                                                                                                     size_t)
                                                                                                                ->
                                                                                                                    fs_offset_t))),
                        ::std::mem::transmute::<*mut libc::c_void,
                                                pfseek>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                *mut file_t,
                                                                                                            _:
                                                                                                                fs_offset_t,
                                                                                                            _:
                                                                                                                libc::c_int)
                                                                                           ->
                                                                                               libc::c_int>,
                                                                                *mut libc::c_void>(Some(FS_Seek
                                                                                                            as
                                                                                                            unsafe extern "C" fn(_:
                                                                                                                                     *mut file_t,
                                                                                                                                 _:
                                                                                                                                     fs_offset_t,
                                                                                                                                 _:
                                                                                                                                     libc::c_int)
                                                                                                                ->
                                                                                                                    libc::c_int))),
                        &mut sc) == 0 {
        Con_DPrintf(b"^1Error:^7 Stream_OpenMPG: failed to load (%s): %s\n\x00"
                        as *const u8 as *const libc::c_char, filename,
                    get_error(mpeg)); // how many samples left from previous frame
        close_decoder(mpeg); // always 16 bit
        _Mem_Free(stream as *mut libc::c_void,
                  b"../engine/common/soundlib/snd_mp3.c\x00" as *const u8 as
                      *const libc::c_char, 145 as libc::c_int);
        FS_Close(file);
        return 0 as *mut stream_t
    }
    (*stream).buffsize = 0 as libc::c_int;
    (*stream).channels = sc.channels;
    (*stream).rate = sc.rate;
    (*stream).width = 2 as libc::c_int;
    (*stream).ptr = mpeg;
    (*stream).type_0 = WF_MPGDATA as libc::c_int;
    return stream;
}
/*
=================
Stream_ReadMPG

assume stream is valid
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_ReadMPG(mut stream: *mut stream_t,
                                        mut needBytes: libc::c_int,
                                        mut buffer: *mut libc::c_void)
 -> libc::c_int {
    // buffer handling
    let mut bytesWritten: libc::c_int = 0 as libc::c_int;
    let mut mpg: *mut libc::c_void = 0 as *mut libc::c_void;
    mpg = (*stream).ptr;
    loop  {
        let mut data: *mut byte = 0 as *mut byte;
        let mut outsize: libc::c_int = 0;
        if (*stream).buffsize == 0 {
            if read_mpeg_stream(mpg, (*stream).temp.as_mut_ptr() as *mut byte,
                                &mut (*stream).pos) != 0 as libc::c_int {
                break ;
                // no bytes remaining
                // there was end of the stream
            }
        }
        // check remaining size
        if (bytesWritten as libc::c_ulong).wrapping_add((*stream).pos) >
               needBytes as libc::c_ulong {
            outsize = needBytes - bytesWritten
        } else { outsize = (*stream).pos as libc::c_int }
        // copy raw sample to output buffer
        data = (buffer as *mut byte).offset(bytesWritten as isize);
        memcpy(data as *mut libc::c_void,
               &mut *(*stream).temp.as_mut_ptr().offset((*stream).buffsize as
                                                            isize) as
                   *mut libc::c_char as *const libc::c_void,
               outsize as libc::c_ulong);
        bytesWritten += outsize;
        (*stream).pos =
            ((*stream).pos as
                 libc::c_ulong).wrapping_sub(outsize as libc::c_ulong) as
                size_t as size_t;
        (*stream).buffsize += outsize;
        // continue from this sample on a next call
        if bytesWritten >= needBytes { return bytesWritten }
        (*stream).buffsize = 0 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
=================
Stream_SetPosMPG

assume stream is valid
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_SetPosMPG(mut stream: *mut stream_t,
                                          mut newpos: libc::c_int)
 -> libc::c_int {
    if set_stream_pos((*stream).ptr, newpos) != -(1 as libc::c_int) {
        // flush any previous data
        (*stream).buffsize = 0 as libc::c_int;
        return true_0 as libc::c_int
    }
    // failed to seek for some reasons
    return false_0 as libc::c_int;
}
/*
=================
Stream_GetPosMPG

assume stream is valid
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_GetPosMPG(mut stream: *mut stream_t)
 -> libc::c_int {
    return get_stream_pos((*stream).ptr);
}
/*
=================
Stream_FreeMPG

assume stream is valid
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Stream_FreeMPG(mut stream: *mut stream_t) {
    if !(*stream).ptr.is_null() {
        close_decoder((*stream).ptr);
        (*stream).ptr = 0 as *mut libc::c_void
    }
    if !(*stream).file.is_null() {
        FS_Close((*stream).file);
        (*stream).file = 0 as *mut file_t
    }
    _Mem_Free(stream as *mut libc::c_void,
              b"../engine/common/soundlib/snd_mp3.c\x00" as *const u8 as
                  *const libc::c_char, 262 as libc::c_int);
}
